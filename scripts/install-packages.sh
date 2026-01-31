#!/bin/bash
# Install system packages based on detected distro

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

# Source distro detection if not already done
if [ -z "$DISTRO" ]; then
    source "$SCRIPT_DIR/detect-distro.sh"
fi

PKG_FILE="$REPO_DIR/packages/$PKG_LIST_FILE"

if [ ! -f "$PKG_FILE" ]; then
    echo "Error: Package list not found: $PKG_FILE"
    exit 1
fi

echo "=== Installing system packages for $DISTRO ==="

# Read packages from file, filtering comments and empty lines
PACKAGES=$(grep -v '^#' "$PKG_FILE" | grep -v '^$' | tr '\n' ' ')

echo "Packages to install:"
echo "$PACKAGES"
echo ""

if [ "$DISTRO" = "fedora" ]; then
    # Add COPR repos for packages not in main repos
    echo "Adding COPR repositories..."

    # Lazygit
    sudo dnf copr enable -y atim/lazygit 2>/dev/null || true

    # SwayOSD
    sudo dnf copr enable -y erikreider/SwayNotificationCenter 2>/dev/null || true

    # Update package database
    echo "Updating package database..."
    $PKG_UPDATE

    # Install packages (skip unavailable ones)
    echo "Installing packages..."
    sudo dnf install -y --skip-unavailable $PACKAGES || true

    # Install lazygit from COPR
    echo "Installing lazygit..."
    sudo dnf install -y lazygit 2>/dev/null || echo "  lazygit will be installed via cargo"

elif [ "$DISTRO" = "arch" ]; then
    # Update package database
    echo "Updating package database..."
    $PKG_UPDATE

    # Install packages
    echo "Installing packages..."
    sudo pacman -S --needed --noconfirm $PACKAGES || true

    # Check for yay/paru for AUR packages
    if command -v yay &> /dev/null; then
        echo "Installing AUR packages with yay..."
        yay -S --needed --noconfirm walker-bin swayosd-git lazydocker impala bluetui 2>/dev/null || true
    elif command -v paru &> /dev/null; then
        echo "Installing AUR packages with paru..."
        paru -S --needed --noconfirm walker-bin swayosd-git lazydocker impala bluetui 2>/dev/null || true
    else
        echo "  Note: Install yay or paru for AUR packages (walker, swayosd, etc.)"
    fi
else
    echo "Updating package database..."
    $PKG_UPDATE

    echo "Installing packages..."
    $PKG_INSTALL $PACKAGES || true
fi

echo "=== System packages installed ==="
