#!/bin/bash
# cruzAlex Linux - Main Bootstrap Script
# One command to set up everything

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
VERSION=$(cat "$REPO_DIR/VERSION" 2>/dev/null || echo "0.1.0")

echo -e "${CYAN}"
cat << 'EOF'
   ██████╗██████╗ ██╗   ██╗███████╗ █████╗ ██╗     ███████╗██╗  ██╗
  ██╔════╝██╔══██╗██║   ██║╚══███╔╝██╔══██╗██║     ██╔════╝╚██╗██╔╝
  ██║     ██████╔╝██║   ██║  ███╔╝ ███████║██║     █████╗   ╚███╔╝
  ██║     ██╔══██╗██║   ██║ ███╔╝  ██╔══██║██║     ██╔══╝   ██╔██╗
  ╚██████╗██║  ██║╚██████╔╝███████╗██║  ██║███████╗███████╗██╔╝ ██╗
   ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝
EOF
echo -e "${NC}"
echo -e "${BLUE}  TUI-first, keyboard-driven Linux experience v${VERSION}${NC}"
echo -e "${BLUE}  Inspired by Omarchy | github.com/cruzalex/cruzalex-linux-config${NC}"
echo ""

# Detect distribution
echo -e "${YELLOW}[1/6] Detecting distribution...${NC}"
source "$REPO_DIR/scripts/detect-distro.sh"
echo ""

# Confirm with user
echo -e "${YELLOW}This will install packages and set up configurations.${NC}"
echo "Repository: $REPO_DIR"
echo "Target: $DISTRO ($ARCH)"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Create necessary directories
echo ""
echo -e "${YELLOW}[2/6] Creating directories...${NC}"
mkdir -p ~/Pictures/Screenshots
mkdir -p ~/Videos/Recordings
mkdir -p ~/.config/cruzalex/themes
mkdir -p ~/.config/cruzalex/wallpapers
mkdir -p ~/.local/bin
echo "  Created user directories"

# Install system packages
echo ""
echo -e "${YELLOW}[3/6] Installing system packages...${NC}"
source "$REPO_DIR/scripts/install-packages.sh"

# Install cargo apps
echo ""
echo -e "${YELLOW}[4/6] Installing Rust and cargo applications...${NC}"
source "$REPO_DIR/scripts/install-cargo-apps.sh"

# Build and install cruzalex-themes TUI
echo ""
echo -e "${YELLOW}[5/6] Building cruzalex-themes TUI...${NC}"
if [ -d "$REPO_DIR/tools/cruzalex-themes" ]; then
    cd "$REPO_DIR/tools/cruzalex-themes"
    if cargo build --release 2>/dev/null; then
        cp target/release/cruzalex-themes ~/.local/bin/
        echo "  Built and installed cruzalex-themes"
    else
        echo "  Warning: Could not build cruzalex-themes (optional)"
    fi
    cd "$REPO_DIR"
fi

# Setup configurations
echo ""
echo -e "${YELLOW}[6/6] Setting up configuration files...${NC}"
source "$REPO_DIR/scripts/setup-configs.sh"

# Final setup
echo ""
echo -e "${YELLOW}Finishing up...${NC}"

# Save version
echo "$VERSION" > ~/.config/cruzalex/version

# Change default shell to zsh
if [ "$SHELL" != "$(which zsh)" ]; then
    echo "Changing default shell to zsh..."
    chsh -s "$(which zsh)"
fi

# Apply default theme if themes exist
if [ -d "$HOME/.config/cruzalex/themes/cobalt2" ]; then
    echo "Applying default theme (cobalt2)..."
    "$HOME/.config/cruzalex/bin/cruzalex-theme-set" cobalt2 2>/dev/null || true
fi

echo ""
echo -e "${GREEN}"
cat << 'EOF'
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   Installation Complete!                                          ║
║                                                                   ║
║   Next steps:                                                     ║
║   1. Log out and back in (or reboot) for all changes              ║
║   2. Press Super + K to see all keybindings                       ║
║   3. Press Super + Ctrl + Shift + Space for theme browser         ║
║                                                                   ║
║   Quick start keybindings:                                        ║
║   • Super + Space           → App launcher                        ║
║   • Super + Return          → Terminal                            ║
║   • Super + Shift + F       → File manager (Yazi)                 ║
║   • Super + Shift + N       → Editor (Neovim)                     ║
║   • Super + Ctrl + Space    → Next wallpaper                      ║
║   • Super + Escape          → Power menu                          ║
║                                                                   ║
║   Theme commands:                                                 ║
║   • cruzalex-themes         → TUI theme browser                   ║
║   • cruzalex-theme-list     → List installed themes               ║
║   • cruzalex-theme-set NAME → Apply a theme                       ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"
