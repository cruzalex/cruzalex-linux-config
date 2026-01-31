#!/bin/bash
# Detect Linux distribution and set package manager variables

detect_distro() {
    if [ -f /etc/fedora-release ]; then
        DISTRO="fedora"
        PKG_INSTALL="sudo dnf install -y"
        PKG_UPDATE="sudo dnf update -y"
        PKG_LIST_FILE="fedora.txt"
        echo "Detected: Fedora"
    elif [ -f /etc/arch-release ]; then
        DISTRO="arch"
        PKG_INSTALL="sudo pacman -S --noconfirm"
        PKG_UPDATE="sudo pacman -Syu --noconfirm"
        PKG_LIST_FILE="arch.txt"
        echo "Detected: Arch Linux"
    else
        echo "Error: Unsupported distribution"
        echo "This script supports Fedora and Arch Linux only."
        exit 1
    fi

    # Detect architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "aarch64" ]; then
        echo "Architecture: ARM64 (Apple Silicon / Asahi)"
        IS_ARM=true
    else
        echo "Architecture: x86_64"
        IS_ARM=false
    fi

    export DISTRO PKG_INSTALL PKG_UPDATE PKG_LIST_FILE ARCH IS_ARM
}

# Run detection if sourced or executed
detect_distro
