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

    # Parse accent color for borders (use = to exact match, head -1 for first match)
    local accent=$(grep '^accent *=' "$colors_file" | head -1 | cut -d'"' -f2 | tr -d '#')
    local background=$(grep '^background *=' "$colors_file" | head -1 | cut -d'"' -f2 | tr -d '#')

    if [ -n "$accent" ]; then
        # Set active border color (with timeout)
        timeout --kill-after=0.2 0.5 \
            hyprctl keyword general:col.active_border "rgb($accent)" 2>/dev/null || true
    fi

    if [ -n "$background" ]; then
        # Set inactive border color (dimmed) - use rgba() for alpha support
        timeout --kill-after=0.2 0.5 \
            hyprctl keyword general:col.inactive_border "rgba(${background}88)" 2>/dev/null || true
    fi
}

# Reload Hyprland first to apply theme.conf (with timeout)
timeout --kill-after=0.5 2 hyprctl reload 2>/dev/null || true

# Then override with colors from colors.toml (accent for borders)
# This ensures the accent color is always used for active borders
sleep 0.2
apply_colors
