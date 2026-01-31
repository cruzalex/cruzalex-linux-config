#!/bin/bash
# Install Rust and cargo applications

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
CARGO_FILE="$REPO_DIR/packages/cargo.txt"

echo "=== Installing Rust and Cargo applications ==="

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed: $(rustc --version)"
fi

# Ensure cargo is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Update Rust
echo "Updating Rust..."
rustup update stable

if [ ! -f "$CARGO_FILE" ]; then
    echo "Error: Cargo package list not found: $CARGO_FILE"
    exit 1
fi

# Read packages from file
PACKAGES=$(grep -v '^#' "$CARGO_FILE" | grep -v '^$')

echo "Installing cargo packages..."
for pkg in $PACKAGES; do
    echo "  Installing: $pkg"
    cargo install "$pkg" --locked 2>/dev/null || cargo install "$pkg"
done

echo "=== Cargo applications installed successfully ==="
