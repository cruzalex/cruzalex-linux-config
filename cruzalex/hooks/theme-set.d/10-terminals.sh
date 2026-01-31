#!/bin/bash
# Theme hook: Update terminal emulator configurations
# Supports: Ghostty, Kitty

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"

# Ghostty configuration
update_ghostty() {
    local ghostty_config="$HOME/.config/ghostty"
    local ghostty_theme="$THEME_DIR/ghostty"

    if [ -d "$ghostty_theme" ] || [ -f "$THEME_DIR/ghostty.conf" ]; then
        mkdir -p "$ghostty_config"

        # Link theme config if it exists as a directory
        if [ -d "$ghostty_theme" ]; then
            ln -sf "$ghostty_theme/config" "$ghostty_config/theme" 2>/dev/null || true
        elif [ -f "$THEME_DIR/ghostty.conf" ]; then
            ln -sf "$THEME_DIR/ghostty.conf" "$ghostty_config/theme" 2>/dev/null || true
        fi

        # Reload Ghostty if running (send USR1 signal)
        pkill -USR1 ghostty 2>/dev/null || true
    fi
}

# Kitty configuration
update_kitty() {
    local kitty_config="$HOME/.config/kitty"
    local kitty_theme="$THEME_DIR/kitty"

    if [ -d "$kitty_theme" ] || [ -f "$THEME_DIR/kitty.conf" ]; then
        mkdir -p "$kitty_config"

        if [ -f "$kitty_theme/colors.conf" ]; then
            ln -sf "$kitty_theme/colors.conf" "$kitty_config/theme.conf" 2>/dev/null || true
        elif [ -f "$THEME_DIR/kitty.conf" ]; then
            ln -sf "$THEME_DIR/kitty.conf" "$kitty_config/theme.conf" 2>/dev/null || true
        fi

        # Reload Kitty if running
        if command -v kitty &>/dev/null; then
            kitty @ set-colors --all "$kitty_config/theme.conf" 2>/dev/null || true
        fi
    fi
}

# Generate terminal config from colors.toml if no native config exists
generate_from_colors() {
    local colors_file="$THEME_DIR/colors.toml"
    if [ ! -f "$colors_file" ]; then
        return
    fi

    # This would parse colors.toml and generate configs
    # For now, we rely on theme-provided configs
}

update_ghostty
update_kitty
