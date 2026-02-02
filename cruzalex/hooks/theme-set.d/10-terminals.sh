#!/bin/bash
# Theme hook: Update terminal emulator configurations
# Supports: Ghostty, Kitty (Omarchy-compatible)

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"

# Ghostty configuration
update_ghostty() {
    local ghostty_config="$HOME/.config/ghostty"
    mkdir -p "$ghostty_config"

    # Link ghostty.conf from theme (Omarchy format)
    if [ -f "$THEME_DIR/ghostty.conf" ]; then
        ln -sf "$THEME_DIR/ghostty.conf" "$ghostty_config/theme" 2>/dev/null || true
    fi

    # Note: Ghostty does not support live config reload via signals.
    # New terminal windows will automatically use the updated theme.
    # Existing sessions need to be closed and reopened for the theme to apply.
}

# Kitty configuration
update_kitty() {
    local kitty_config="$HOME/.config/kitty"
    mkdir -p "$kitty_config"

    # Link kitty.conf from theme (Omarchy format)
    if [ -f "$THEME_DIR/kitty.conf" ]; then
        ln -sf "$THEME_DIR/kitty.conf" "$kitty_config/theme.conf" 2>/dev/null || true

        # Reload Kitty if running - try remote control first (with timeout), then signal
        if pgrep -x kitty > /dev/null; then
            timeout --kill-after=0.5 1 \
                kitty @ set-colors --all "$THEME_DIR/kitty.conf" 2>/dev/null || \
                pkill -USR1 kitty 2>/dev/null || true
        fi
    fi
}

update_ghostty
update_kitty
