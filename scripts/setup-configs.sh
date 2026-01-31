#!/bin/bash
# Symlink configuration files to appropriate locations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
CONFIG_DIR="$REPO_DIR/config"
CRUZALEX_DIR="$REPO_DIR/cruzalex"
HOME_DIR="$REPO_DIR/home"
BACKUP_DIR="$HOME/.config-backup-$(date +%Y%m%d-%H%M%S)"

echo "=== Setting up configuration files ==="

# Function to safely create symlink with backup
safe_symlink() {
    local src="$1"
    local dest="$2"

    if [ -e "$dest" ] && [ ! -L "$dest" ]; then
        echo "  Backing up existing: $dest"
        mkdir -p "$BACKUP_DIR"
        mv "$dest" "$BACKUP_DIR/"
    elif [ -L "$dest" ]; then
        rm "$dest"
    fi

    mkdir -p "$(dirname "$dest")"
    ln -s "$src" "$dest"
    echo "  Linked: $dest -> $src"
}

# Symlink ~/.config directories
echo "Linking config directories..."
for dir in "$CONFIG_DIR"/*/; do
    if [ -d "$dir" ]; then
        dirname=$(basename "$dir")
        safe_symlink "$dir" "$HOME/.config/$dirname"
    fi
done

# Symlink standalone config files (like starship.toml)
for file in "$CONFIG_DIR"/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        safe_symlink "$file" "$HOME/.config/$filename"
    fi
done

# Symlink cruzalex theme system
echo "Linking cruzalex theme system..."
safe_symlink "$CRUZALEX_DIR" "$HOME/.config/cruzalex"

# Symlink home dotfiles
echo "Linking home dotfiles..."
for file in "$HOME_DIR"/.*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        safe_symlink "$file" "$HOME/$filename"
    fi
done

# Make theme scripts executable
chmod +x "$CRUZALEX_DIR"/bin/* 2>/dev/null || true
chmod +x "$CRUZALEX_DIR"/hooks/theme-set.d/* 2>/dev/null || true

# Add cruzalex bin to PATH if not already present
if ! grep -q 'cruzalex/bin' "$HOME/.zshrc" 2>/dev/null; then
    echo "Adding cruzalex bin to PATH in .zshrc..."
fi

echo ""
if [ -d "$BACKUP_DIR" ]; then
    echo "Backups stored in: $BACKUP_DIR"
fi

echo "=== Configuration setup complete ==="
