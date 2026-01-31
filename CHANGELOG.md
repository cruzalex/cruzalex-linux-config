# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Theme browser TUI with image preview support
- Auto wallpaper rotation (5-minute intervals)
- Additional Omarchy theme compatibility

## [0.1.0] - 2026-01-31

### Added
- Initial release
- Bootstrap installer for Fedora Asahi and Arch Linux
- Hyprland configuration with Omarchy-style keybindings
- Theme system compatible with Omarchy themes
- Theme management CLI tools:
  - `cruzalex-theme-set` - Apply a theme
  - `cruzalex-theme-list` - List installed themes
  - `cruzalex-theme-install` - Install themes from GitHub/Omarchy
  - `cruzalex-theme-menu` - Interactive theme picker
  - `cruzalex-wallpaper-next` - Cycle wallpapers
- Application configurations:
  - Ghostty (primary terminal)
  - Kitty (backup terminal)
  - Neovim with LazyVim
  - Waybar status bar
  - Yazi file manager
  - btop system monitor
  - Mako notifications
  - Walker app launcher
  - spotify-player
  - Starship prompt
- ZSH configuration with modern shell tools
- Theme hooks for automatic app theming
- Cobalt2 theme included as default

### Technical Details
- Supports Fedora Asahi (ARM64) and Arch Linux (x86_64)
- Uses symbolic links for dotfile management
- Modular Hyprland configuration
- 24-color theme format (Omarchy compatible)
