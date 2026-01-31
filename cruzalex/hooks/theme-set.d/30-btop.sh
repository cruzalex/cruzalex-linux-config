#!/bin/bash
# Theme hook: Update btop theme

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
BTOP_CONFIG="$HOME/.config/btop"
BTOP_THEMES="$BTOP_CONFIG/themes"

# Create themes directory
mkdir -p "$BTOP_THEMES"

# Link theme if it exists
if [ -f "$THEME_DIR/btop.theme" ]; then
    cp "$THEME_DIR/btop.theme" "$BTOP_THEMES/cruzalex.theme"
elif [ -f "$THEME_DIR/btop/theme.theme" ]; then
    cp "$THEME_DIR/btop/theme.theme" "$BTOP_THEMES/cruzalex.theme"
fi

# Update btop config to use the theme
BTOP_CONF="$BTOP_CONFIG/btop.conf"
if [ -f "$BTOP_CONF" ]; then
    # Update color_theme line if it exists
    if grep -q '^color_theme' "$BTOP_CONF"; then
        sed -i 's/^color_theme.*/color_theme = "cruzalex"/' "$BTOP_CONF"
    fi
fi

# btop doesn't support runtime reload, user needs to restart it
