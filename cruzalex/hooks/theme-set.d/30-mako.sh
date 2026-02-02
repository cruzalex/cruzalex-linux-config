#!/bin/bash
# Theme hook: Update Mako notification daemon

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
MAKO_CONFIG="$HOME/.config/mako"
MAKO_CONF="$MAKO_CONFIG/config"

mkdir -p "$MAKO_CONFIG"

# Get colors from theme
get_colors() {
    local colors_file="$THEME_DIR/colors.toml"
    if [ -f "$colors_file" ]; then
        BG=$(grep '^background' "$colors_file" | head -1 | cut -d'"' -f2)
        FG=$(grep '^foreground' "$colors_file" | head -1 | cut -d'"' -f2)
        ACCENT=$(grep '^accent' "$colors_file" | head -1 | cut -d'"' -f2)
    fi
    # Defaults if not found
    BG="${BG:-#1a1b26}"
    FG="${FG:-#c0caf5}"
    ACCENT="${ACCENT:-#7aa2f7}"
}

# Update colors in main config
update_mako_colors() {
    if [ ! -f "$MAKO_CONF" ]; then
        return
    fi

    get_colors

    # Update color lines in the main config
    sed -i "s/^background-color=.*/background-color=${BG}ee/" "$MAKO_CONF"
    sed -i "s/^text-color=.*/text-color=${FG}/" "$MAKO_CONF"
    sed -i "s/^border-color=.*/border-color=${ACCENT}/" "$MAKO_CONF"
    sed -i "s/^progress-color=.*/progress-color=over ${ACCENT}66/" "$MAKO_CONF"
}

update_mako_colors

# Reload mako (with timeout to prevent hanging)
if pgrep -x "mako" > /dev/null; then
    timeout --kill-after=0.2 0.5 makoctl reload 2>/dev/null || true
fi
