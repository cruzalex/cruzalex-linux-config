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

# Update package database
echo "Updating package database..."
$PKG_UPDATE

# Install packages
echo "Installing packages..."
$PKG_INSTALL $PACKAGES

echo "=== System packages installed successfully ==="
