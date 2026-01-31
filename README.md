# cruzAlex Linux

<div align="center">

```
   ██████╗██████╗ ██╗   ██╗███████╗ █████╗ ██╗     ███████╗██╗  ██╗
  ██╔════╝██╔══██╗██║   ██║╚══███╔╝██╔══██╗██║     ██╔════╝╚██╗██╔╝
  ██║     ██████╔╝██║   ██║  ███╔╝ ███████║██║     █████╗   ╚███╔╝
  ██║     ██╔══██╗██║   ██║ ███╔╝  ██╔══██║██║     ██╔══╝   ██╔██╗
  ╚██████╗██║  ██║╚██████╔╝███████╗██║  ██║███████╗███████╗██╔╝ ██╗
   ╚═════╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝
```

**A TUI-first, keyboard-driven Linux experience inspired by Omarchy**

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](VERSION)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Fedora%20Asahi%20%7C%20Arch-orange.svg)]()

</div>

## Overview

cruzAlex Linux is an opinionated dotfiles configuration that brings the Omarchy experience to **Fedora Asahi** (Apple Silicon) and **Arch Linux** (x86_64). It features:

- **Hyprland** tiling window manager with Omarchy-style keybindings
- **TUI-first** workflow with beautiful terminal applications
- **Theme system** compatible with [Omarchy themes](https://omarchythemes.com/)
- **cruzalex-themes** - A TUI browser to discover, preview, and install themes with image support

## Features

### Theme Browser TUI

Browse 190+ Omarchy themes directly in your terminal with image previews:

```
┌─────────────────────────────────────────────────────────────────┐
│  cruzAlex Themes                                    ◉ Connected │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │ preview  │  │ preview  │  │ preview  │  │ preview  │       │
│  │  image   │  │  image   │  │  image   │  │  image   │       │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘       │
│  Tokyo Night   Catppuccin     Gruvbox       Nord              │
│  ● Active      ○ Installed    ◌ Available   ◌ Available       │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ [Enter] Apply  [i] Install  [d] Delete  [/] Search  [q] Quit   │
└─────────────────────────────────────────────────────────────────┘
```

### Wallpaper Rotation

- **Manual**: `Super + Ctrl + Space` - Next wallpaper
- **Auto-rotate**: Enable 5-minute rotation in settings

### Omarchy-Compatible Keybindings

Full keyboard-driven workflow matching Omarchy defaults.

## Quick Start

### One-Line Install

```bash
curl -fsSL https://raw.githubusercontent.com/cruzalex/cruzalex-linux-config/main/install.sh | bash
```

### Manual Install

```bash
git clone https://github.com/cruzalex/cruzalex-linux-config.git ~/.dotfiles
cd ~/.dotfiles
./install.sh
```

## Supported Systems

| System | Architecture | Status |
|--------|--------------|--------|
| Fedora Asahi | ARM64 (Apple Silicon) | Primary |
| Arch Linux | x86_64 | Supported |

## Keybindings

### Core

| Binding | Action |
|---------|--------|
| `Super + Space` | Application launcher (Walker) |
| `Super + Return` | Terminal (Ghostty) |
| `Super + W` | Close window |
| `Super + F` | Fullscreen |
| `Super + T` | Toggle floating |
| `Super + Escape` | Power menu |

### Navigation

| Binding | Action |
|---------|--------|
| `Super + H/J/K/L` | Focus window (vim-style) |
| `Super + Shift + H/J/K/L` | Move window |
| `Super + 1-9` | Switch workspace |
| `Super + Shift + 1-9` | Move window to workspace |
| `Super + Tab` | Next workspace |

### Apps

| Binding | Action |
|---------|--------|
| `Super + Shift + B` | Browser (Chromium) |
| `Super + Shift + F` | File manager (Yazi) |
| `Super + Shift + M` | Music (Spotify TUI) |
| `Super + Shift + N` | Editor (Neovim) |
| `Super + Shift + D` | Docker (Lazydocker) |

### Theme & Style

| Binding | Action |
|---------|--------|
| `Super + Ctrl + Shift + Space` | Theme picker TUI |
| `Super + Ctrl + Space` | Next wallpaper |
| `Super + Backspace` | Toggle transparency |

See [KEYBINDINGS.md](docs/KEYBINDINGS.md) for the complete list.

## Theme System

### Browse & Install Themes

```bash
# Launch theme browser TUI
cruzalex-themes

# Or via command line
cruzalex-theme-list              # List installed themes
cruzalex-theme-install tokyo-night  # Install from Omarchy
cruzalex-theme-set tokyo-night   # Apply theme
```

### Theme Structure

Themes follow the Omarchy format:

```
~/.config/cruzalex/themes/<theme-name>/
├── backgrounds/         # Wallpaper images
├── colors.toml          # Color palette (24 colors)
├── preview.png          # Theme preview
├── ghostty.conf         # Terminal theme
├── neovim.lua           # Editor theme
├── waybar.css           # Status bar theme
└── ...                  # Other app configs
```

### colors.toml Format

```toml
# Core colors
foreground = "#c0caf5"
background = "#1a1b26"
accent = "#7aa2f7"
cursor = "#c0caf5"
selection_background = "#33467c"
selection_foreground = "#c0caf5"

# ANSI colors (0-15)
color0 = "#15161e"
color1 = "#f7768e"
# ... color2-color15
```

## Applications

### Included TUI Apps

| App | Purpose |
|-----|---------|
| **Ghostty** | Terminal emulator |
| **Neovim** | Editor (LazyVim) |
| **Yazi** | File manager |
| **Lazygit** | Git TUI |
| **Lazydocker** | Docker TUI |
| **btop** | System monitor |
| **spotify-player** | Music player |

### Shell Tools

| Tool | Replaces |
|------|----------|
| `eza` | `ls` |
| `bat` | `cat` |
| `ripgrep` | `grep` |
| `fd` | `find` |
| `zoxide` | `cd` |
| `fzf` | fuzzy finder |
| `starship` | prompt |

## Project Structure

```
cruzalex-linux-config/
├── install.sh               # Bootstrap script
├── VERSION                  # Semantic version
├── config/                  # ~/.config/ dotfiles
│   ├── hypr/               # Hyprland WM
│   ├── waybar/             # Status bar
│   ├── ghostty/            # Terminal
│   ├── nvim/               # Neovim (LazyVim)
│   ├── yazi/               # File manager
│   └── ...                 # Other apps
├── cruzalex/               # Theme system
│   ├── bin/                # CLI tools
│   ├── hooks/              # Theme change hooks
│   └── themes/             # Installed themes
├── tools/                  # Rust TUI tools
│   └── cruzalex-themes/    # Theme browser
├── packages/               # Package lists
│   ├── fedora.txt
│   ├── arch.txt
│   └── cargo.txt
└── scripts/                # Installation scripts
```

## Development

### Building the Theme Browser

```bash
cd tools/cruzalex-themes
cargo build --release
```

### Adding a New Hook

Create a script in `cruzalex/hooks/theme-set.d/`:

```bash
#!/bin/bash
# 60-myapp.sh
# Theme hook for myapp

source "$THEME_DIR/colors.toml"
# Apply colors to myapp...
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Acknowledgments

- [Omarchy](https://omarchy.org/) by DHH and Basecamp for the inspiration
- [Omarchy Themes](https://omarchythemes.com/) community for the theme ecosystem
- [Ratatui](https://ratatui.rs/) for the excellent TUI framework

## License

MIT License - see [LICENSE](LICENSE) for details.
