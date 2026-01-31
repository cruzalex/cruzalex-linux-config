#!/bin/bash
# Theme hook: Update Neovim colorscheme

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
NVIM_CONFIG="$HOME/.config/nvim"

# Write theme name to a file that Neovim can read
THEME_FILE="$CRUZALEX_DIR/.current-theme"
echo "${THEME_NAME:-default}" > "$THEME_FILE"

# Check for theme-specific colorscheme mapping
COLORSCHEME_FILE="$THEME_DIR/nvim-colorscheme"
if [ -f "$COLORSCHEME_FILE" ]; then
    NVIM_COLORSCHEME=$(cat "$COLORSCHEME_FILE")
else
    # Try to map theme name to common Neovim colorschemes
    case "${THEME_NAME}" in
        tokyo-night*) NVIM_COLORSCHEME="tokyonight" ;;
        gruvbox*) NVIM_COLORSCHEME="gruvbox" ;;
        catppuccin*) NVIM_COLORSCHEME="catppuccin" ;;
        dracula*) NVIM_COLORSCHEME="dracula" ;;
        nord*) NVIM_COLORSCHEME="nord" ;;
        one-dark*|onedark*) NVIM_COLORSCHEME="onedark" ;;
        solarized*) NVIM_COLORSCHEME="solarized" ;;
        cobalt2*) NVIM_COLORSCHEME="cobalt2" ;;
        *) NVIM_COLORSCHEME="" ;;
    esac
fi

# If we have a colorscheme, tell running Neovim instances
if [ -n "$NVIM_COLORSCHEME" ]; then
    echo "$NVIM_COLORSCHEME" > "$CRUZALEX_DIR/.nvim-colorscheme"

    # Try to notify running Neovim instances via their sockets
    for sock in /run/user/$(id -u)/nvim.*.0 /tmp/nvim.*/0; do
        if [ -S "$sock" ]; then
            nvim --server "$sock" --remote-send "<Cmd>colorscheme $NVIM_COLORSCHEME<CR>" 2>/dev/null || true
        fi
    done
fi
