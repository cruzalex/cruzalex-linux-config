#!/bin/bash
# Install Rust and cargo applications

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
    echo "Warning: Cargo package list not found: $CARGO_FILE"
    echo "=== Skipping cargo packages ==="
    exit 0
fi

# Try to install yazi from system repos first (more stable)
echo "Checking for yazi in system repos..."
if command -v dnf &> /dev/null; then
    sudo dnf install -y yazi 2>/dev/null && echo "  Installed yazi from Fedora repos" || true
elif command -v pacman &> /dev/null; then
    sudo pacman -S --needed --noconfirm yazi 2>/dev/null && echo "  Installed yazi from repos" || true
fi

# Read packages from file
PACKAGES=$(grep -v '^#' "$CARGO_FILE" | grep -v '^$')

echo "Installing cargo packages..."
FAILED=""
for pkg in $PACKAGES; do
    # Skip yazi if already installed from repos
    if [[ "$pkg" == "yazi-fm" || "$pkg" == "yazi-cli" ]]; then
        if command -v yazi &> /dev/null; then
            echo "  Skipping $pkg (yazi already installed)"
            continue
        fi
    fi

    echo "  Installing: $pkg"
    if cargo install "$pkg" --locked 2>/dev/null; then
        echo "    ✓ $pkg installed"
    elif cargo install "$pkg" 2>/dev/null; then
        echo "    ✓ $pkg installed"
    else
        echo "    ✗ $pkg failed (will continue)"
        FAILED="$FAILED $pkg"
    fi
done

if [ -n "$FAILED" ]; then
    echo ""
    echo "Some packages failed to install:$FAILED"
    echo "You can try installing them manually later with: cargo install <package>"
fi

echo "=== Cargo applications installation complete ==="
