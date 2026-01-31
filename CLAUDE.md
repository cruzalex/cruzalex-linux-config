# cruzAlex Linux

A TUI-first, keyboard-driven, beautiful Linux experience for Fedora Asahi (Apple Silicon) and Arch Linux.

**Inspired by [Omarchy](https://omarchy.org/)** - bringing the same opinionated, keyboard-driven workflow to any system.

## Vision

cruzAlex Linux provides:
1. **Omarchy-compatible experience** - Same keybindings, same theme format, same TUI-first approach
2. **Theme browser TUI** - Browse, preview, and install 190+ Omarchy themes with image previews
3. **Wallpaper rotation** - Manual or automatic (5-minute intervals)
4. **Cross-platform** - Works on Fedora Asahi (ARM) and Arch Linux (x86)

## Project Structure

```
cruzalex-linux-config/
├── VERSION                   # Semantic version (0.1.0)
├── README.md                 # User documentation
├── CHANGELOG.md              # Version history
├── LICENSE                   # MIT License
├── CLAUDE.md                 # This file (dev notes)
├── install.sh                # Main bootstrap script
├── scripts/
│   ├── detect-distro.sh      # Detect Fedora vs Arch
│   ├── install-packages.sh   # Distro-agnostic package installer
│   ├── install-cargo-apps.sh # Rust TUI apps
│   └── setup-configs.sh      # Symlink all dotfiles
├── packages/
│   ├── fedora.txt            # DNF packages
│   ├── arch.txt              # Pacman/AUR packages
│   └── cargo.txt             # Cargo packages (shared)
├── config/                   # Dotfiles → ~/.config/
│   ├── hypr/                 # Hyprland WM (7 files)
│   ├── waybar/               # Status bar (3 files)
│   ├── ghostty/              # Primary terminal
│   ├── kitty/                # Backup terminal
│   ├── nvim/                 # LazyVim config
│   ├── btop/                 # System monitor
│   ├── mako/                 # Notifications
│   ├── yazi/                 # File manager
│   ├── spotify-player/       # Spotify TUI
│   ├── walker/               # App launcher
│   └── starship.toml         # Prompt
├── cruzalex/                 # Theme system → ~/.config/cruzalex/
│   ├── themes/               # Downloaded themes
│   │   └── cobalt2/          # Default theme
│   ├── hooks/theme-set.d/    # Per-app theme hooks (8 scripts)
│   └── bin/                  # CLI tools (7 scripts)
├── tools/                    # Rust TUI tools
│   └── cruzalex-themes/      # Theme browser TUI (Ratatui)
├── home/                     # Home dotfiles → ~/
│   └── .zshrc
└── docs/                     # Documentation
    └── KEYBINDINGS.md
```

## Key Commands

### Installation
- `./install.sh` - Full bootstrap (run once on fresh system)

### Theme Management
- `cruzalex-themes` - TUI theme browser with image previews
- `cruzalex-theme-set <theme>` - Switch theme across all apps
- `cruzalex-theme-list` - List installed themes
- `cruzalex-theme-install <url>` - Install theme from GitHub/Omarchy
- `cruzalex-theme-menu` - Interactive theme picker (launcher-based)

### Wallpaper
- `cruzalex-wallpaper-next` - Cycle to next wallpaper
- `cruzalex-wallpaper-rotate start 5` - Auto-rotate every 5 minutes
- `cruzalex-wallpaper-rotate stop` - Stop auto-rotation

### System
- `cruzalex-menu` - Main control menu (like Omarchy)
- `cruzalex-power-menu` - Lock/suspend/reboot/shutdown
- `cruzalex-keybindings` - Show all keybindings (searchable with fzf)

## Theme System

Themes follow the **Omarchy format** with `colors.toml`:

```toml
foreground = "#c0caf5"
background = "#1a1b26"
accent = "#7aa2f7"
cursor = "#c0caf5"
selection_background = "#33467c"
selection_foreground = "#c0caf5"
color0 = "#15161e"
color1 = "#f7768e"
# ... color2-color15
```

Theme hooks in `~/.config/cruzalex/hooks/theme-set.d/` update each app:
- `10-terminals.sh` - Ghostty, Kitty
- `20-hyprland.sh` - Window borders
- `20-waybar.sh` - Status bar CSS
- `30-btop.sh` - System monitor
- `30-mako.sh` - Notifications
- `40-neovim.sh` - Editor colorscheme
- `50-spotify.sh` - Music player
- `50-yazi.sh` - File manager

## Keybindings (Omarchy-compatible)

### Core
| Binding | Action |
|---------|--------|
| Super + Space | App launcher (Walker) |
| Super + Alt + Space | cruzAlex menu |
| Super + Return | Terminal (Ghostty) |
| Super + K | Show keybindings |
| Super + Escape | Power menu |

### Windows
| Binding | Action |
|---------|--------|
| Super + W | Close window |
| Super + T | Toggle floating |
| Super + F | Fullscreen |
| Super + Arrows | Focus direction |
| Super + Shift + Arrows | Move window |

### Workspaces
| Binding | Action |
|---------|--------|
| Super + 1-9 | Switch workspace |
| Super + Shift + 1-9 | Move to workspace |
| Super + Tab | Next workspace |
| Super + S | Scratchpad |

### Applications
| Binding | Action |
|---------|--------|
| Super + Shift + B | Browser |
| Super + Shift + F | File manager (Yazi) |
| Super + Shift + M | Music (Spotify) |
| Super + Shift + N | Neovim |
| Super + Shift + G | Lazygit |
| Super + Shift + D | Lazydocker |

### Theme & Style
| Binding | Action |
|---------|--------|
| Super + Ctrl + Shift + Space | Theme browser TUI |
| Super + Ctrl + Space | Next wallpaper |
| Super + Backspace | Toggle transparency |

## Target Systems

1. **Fedora Asahi** (ARM64) - MacBook Pro 16" M1 Max (primary dev machine)
2. **Arch Linux** (x86_64) - Future machines

## Development Notes

- All configs support theme variables via hooks
- Test changes on Fedora Asahi first
- Theme browser uses Ratatui (Rust) with Kitty graphics protocol for images
- Ghostty is configured for image display (`image-storage-limit`)
- Version follows semantic versioning in `VERSION` file

## Theme Browser TUI

Built with Ratatui, the theme browser (`tools/cruzalex-themes/`) features:
- Browse 190+ themes from omarchythemes.com via GitHub API
- Show theme previews with Kitty graphics protocol
- Filter: All / Installed / Available
- Search themes
- One-key install and apply
- Color palette preview

Dependencies: `ratatui`, `ratatui-image`, `tokio`, `reqwest`, `git2`

## Future Features

- [ ] Add more Omarchy themes to the default install
- [ ] Add ratatui-image support for full preview images
- [ ] Add web app shortcuts (HEY, Basecamp, etc.)
- [ ] Add mise for development language management
- [ ] Add fingerprint/FIDO2 setup scripts
- [ ] Add btrfs snapshot integration
