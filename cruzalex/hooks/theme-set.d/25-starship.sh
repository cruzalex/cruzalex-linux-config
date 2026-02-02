#!/bin/bash
# Theme hook: Update starship prompt colors
# This generates a cruzalex palette based on the current theme

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
STARSHIP_CONFIG="$HOME/.config/starship.toml"

# Only proceed if starship config exists
[ -f "$STARSHIP_CONFIG" ] || exit 0

# Get colors from theme
get_colors() {
    local colors_file="$THEME_DIR/colors.toml"
    if [ -f "$colors_file" ]; then
        FG=$(grep '^foreground' "$colors_file" | head -1 | cut -d'"' -f2)
        BG=$(grep '^background' "$colors_file" | head -1 | cut -d'"' -f2)
        ACCENT=$(grep '^accent' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR1=$(grep '^color1 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # red
        COLOR2=$(grep '^color2 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # green
        COLOR3=$(grep '^color3 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # yellow
        COLOR4=$(grep '^color4 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # blue
        COLOR5=$(grep '^color5 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # magenta
        COLOR6=$(grep '^color6 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # cyan
        COLOR8=$(grep '^color8 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # bright black
    fi

    # Defaults if not found
    FG="${FG:-#c0caf5}"
    BG="${BG:-#1a1b26}"
    ACCENT="${ACCENT:-#7aa2f7}"
    COLOR1="${COLOR1:-#f7768e}"
    COLOR2="${COLOR2:-#9ece6a}"
    COLOR3="${COLOR3:-#e0af68}"
    COLOR4="${COLOR4:-#7aa2f7}"
    COLOR5="${COLOR5:-#bb9af7}"
    COLOR6="${COLOR6:-#7dcfff}"
    COLOR8="${COLOR8:-#414868}"
}

# Generate darker version of a color for backgrounds
darken_color() {
    local hex="$1"
    # Remove # and extract RGB
    hex="${hex#\#}"
    local r=$((16#${hex:0:2}))
    local g=$((16#${hex:2:2}))
    local b=$((16#${hex:4:2}))

    # Darken by 40%
    r=$((r * 60 / 100))
    g=$((g * 60 / 100))
    b=$((b * 60 / 100))

    printf '#%02x%02x%02x' "$r" "$g" "$b"
}

update_starship_palette() {
    get_colors

    # Calculate background colors
    local bg1=$(darken_color "$COLOR8")
    local bg3="$COLOR8"

    # Create the cruzalex palette
    local palette_content="[palettes.cruzalex]
color_fg0 = '$FG'
color_bg1 = '$bg1'
color_bg3 = '$bg3'
color_blue = '$COLOR4'
color_aqua = '$COLOR6'
color_green = '$COLOR2'
color_orange = '$ACCENT'
color_purple = '$COLOR5'
color_red = '$COLOR1'
color_yellow = '$COLOR3'"

    # Check if cruzalex palette already exists
    if grep -q '^\[palettes\.cruzalex\]' "$STARSHIP_CONFIG"; then
        # Remove existing cruzalex palette section (from [palettes.cruzalex] to next section or EOF)
        sed -i '/^\[palettes\.cruzalex\]/,/^\[/{/^\[palettes\.cruzalex\]/d;/^\[/!d}' "$STARSHIP_CONFIG"
    fi

    # Append the new palette
    echo "" >> "$STARSHIP_CONFIG"
    echo "$palette_content" >> "$STARSHIP_CONFIG"

    # Update the palette selection to use cruzalex
    if grep -q "^palette = " "$STARSHIP_CONFIG"; then
        sed -i "s/^palette = .*/palette = 'cruzalex'/" "$STARSHIP_CONFIG"
    fi
}

update_starship_palette
