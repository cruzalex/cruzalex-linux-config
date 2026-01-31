#!/bin/bash
# Theme hook: Update Hyprland configuration

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
HYPR_CONFIG="$HOME/.config/hypr"

# Check if Hyprland is running
if ! pgrep -x "Hyprland" > /dev/null; then
    exit 0
fi

# Link theme-specific Hyprland config if it exists
if [ -f "$THEME_DIR/hyprland.conf" ]; then
    ln -sf "$THEME_DIR/hyprland.conf" "$HYPR_CONFIG/theme.conf"
fi

# Extract colors from colors.toml and apply via hyprctl
apply_colors() {
    local colors_file="$THEME_DIR/colors.toml"
    if [ ! -f "$colors_file" ]; then
        return
    fi

    # Parse accent color for borders
    local accent=$(grep '^accent' "$colors_file" | cut -d'"' -f2 | tr -d '#')
    local background=$(grep '^background' "$colors_file" | cut -d'"' -f2 | tr -d '#')

    if [ -n "$accent" ]; then
        # Set active border color
        hyprctl keyword general:col.active_border "rgb($accent)" 2>/dev/null || true
    fi

    if [ -n "$background" ]; then
        # Set inactive border color (dimmed)
        hyprctl keyword general:col.inactive_border "rgb(${background}88)" 2>/dev/null || true
    fi
}

apply_colors

# Reload Hyprland
hyprctl reload 2>/dev/null || true
