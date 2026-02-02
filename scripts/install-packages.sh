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

# Install JetBrainsMono Nerd Font (required for TUI icons)
echo ""
echo "=== Installing JetBrainsMono Nerd Font ==="

FONT_DIR="$HOME/.local/share/fonts/JetBrainsMono"
FONT_URL="https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.zip"

# Check if already installed
if fc-list | grep -qi "JetBrainsMono Nerd Font"; then
    echo "  JetBrainsMono Nerd Font already installed"
else
    echo "  Downloading JetBrainsMono Nerd Font..."
    mkdir -p "$FONT_DIR"

    if curl -fLo /tmp/JetBrainsMono.zip "$FONT_URL" 2>/dev/null; then
        echo "  Extracting fonts..."
        unzip -oq /tmp/JetBrainsMono.zip -d "$FONT_DIR"
        rm -f /tmp/JetBrainsMono.zip

        echo "  Rebuilding font cache..."
        fc-cache -f "$FONT_DIR"

        echo "  JetBrainsMono Nerd Font installed successfully"
    else
        echo "  Warning: Could not download JetBrainsMono Nerd Font"
        echo "  TUI apps may not display icons correctly"
        echo "  You can install manually from: $FONT_URL"
    fi
fi

# Install Flatpak apps
echo ""
echo "=== Installing Flatpak apps ==="

FLATPAK_FILE="$REPO_DIR/packages/flatpak.txt"
if [ -f "$FLATPAK_FILE" ] && command -v flatpak &> /dev/null; then
    # Ensure Flathub is added
    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo 2>/dev/null || true

    FLATPAKS=$(grep -v '^#' "$FLATPAK_FILE" | grep -v '^$' | tr '\n' ' ')
    if [ -n "$FLATPAKS" ]; then
        echo "Installing: $FLATPAKS"
        for pkg in $FLATPAKS; do
            flatpak install -y flathub "$pkg" 2>/dev/null || echo "  Could not install $pkg"
        done
    fi
else
    echo "  Flatpak not available or no flatpak.txt found, skipping"
fi

echo "=== System packages installed ==="
