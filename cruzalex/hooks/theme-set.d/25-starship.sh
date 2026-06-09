#!/bin/bash
# Theme hook: Update starship prompt colors
# Generates a 5-band pastel-powerline palette from the theme's bright colors
# (color1-5) so the ribbon is visible regardless of wallpaper transparency.

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
STARSHIP_CONFIG="$HOME/.config/starship.toml"

[ -f "$STARSHIP_CONFIG" ] || exit 0

get_colors() {
    local colors_file="$THEME_DIR/colors.toml"
    if [ -f "$colors_file" ]; then
        FG=$(grep '^foreground' "$colors_file" | head -1 | cut -d'"' -f2)
        BG=$(grep '^background' "$colors_file" | head -1 | cut -d'"' -f2)
        ACCENT=$(grep '^accent' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR0=$(grep '^color0 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR1=$(grep '^color1 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR2=$(grep '^color2 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR3=$(grep '^color3 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR4=$(grep '^color4 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR5=$(grep '^color5 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR6=$(grep '^color6 *=' "$colors_file" | head -1 | cut -d'"' -f2)
        COLOR8=$(grep '^color8 *=' "$colors_file" | head -1 | cut -d'"' -f2)
    fi

    FG="${FG:-#c0caf5}"
    BG="${BG:-#1a1b26}"
    ACCENT="${ACCENT:-#7aa2f7}"
    COLOR0="${COLOR0:-#15161e}"
    COLOR1="${COLOR1:-#f7768e}"
    COLOR2="${COLOR2:-#9ece6a}"
    COLOR3="${COLOR3:-#e0af68}"
    COLOR4="${COLOR4:-#7aa2f7}"
    COLOR5="${COLOR5:-#bb9af7}"
    COLOR6="${COLOR6:-#7dcfff}"
    COLOR8="${COLOR8:-#414868}"
}

is_light_color() {
    local hex="$1"
    hex="${hex#\#}"
    local r=$((16#${hex:0:2}))
    local g=$((16#${hex:2:2}))
    local b=$((16#${hex:4:2}))
    local luminance=$(( (299 * r + 587 * g + 114 * b) / 1000 ))
    if [ "$luminance" -gt 140 ]; then
        echo "1"
    else
        echo "0"
    fi
}

get_contrast_fg() {
    local bg_color="$1"
    if [ "$(is_light_color "$bg_color")" = "1" ]; then
        echo "#000000"
    else
        echo "#ffffff"
    fi
}

update_starship_palette() {
    get_colors

    # 5-band pastel ribbon mapping to bright theme colors:
    # orange band (user/os) -> COLOR1 (red/coral)
    # yellow band (dir)     -> COLOR3 (yellow)
    # aqua band (git)       -> COLOR2 (green/mint)
    # blue band (lang)      -> COLOR4 (blue)
    # bg3 band (time)       -> COLOR5 (purple/magenta)
    local c_orange="$COLOR1"
    local c_yellow="$COLOR3"
    local c_aqua="$COLOR2"
    local c_blue="$COLOR4"
    local c_bg3="$COLOR5"
    local c_bg1="$COLOR8"

    local fg_orange=$(get_contrast_fg "$c_orange")
    local fg_yellow=$(get_contrast_fg "$c_yellow")
    local fg_aqua=$(get_contrast_fg "$c_aqua")
    local fg_blue=$(get_contrast_fg "$c_blue")
    local fg_bg3=$(get_contrast_fg "$c_bg3")
    local fg_bg1=$(get_contrast_fg "$c_bg1")

    local palette_content="[palettes.cruzalex]
color_fg0 = '#fbf1c7'
color_fg_dark = '#1a1b26'
color_fg_orange = '$fg_orange'
color_fg_yellow = '$fg_yellow'
color_fg_aqua = '$fg_aqua'
color_fg_blue = '$fg_blue'
color_fg_bg3 = '$fg_bg3'
color_fg_bg1 = '$fg_bg1'
color_bg1 = '$c_bg1'
color_bg3 = '$c_bg3'
color_blue = '$c_blue'
color_aqua = '$c_aqua'
color_green = '$COLOR2'
color_orange = '$c_orange'
color_purple = '$COLOR5'
color_red = '$COLOR1'
color_yellow = '$c_yellow'"

    if grep -q '^\[palettes\.cruzalex\]' "$STARSHIP_CONFIG"; then
        sed -i '/^\[palettes\.cruzalex\]/,$d' "$STARSHIP_CONFIG"
        sed -i -e :a -e '/^\n*$/{$d;N;ba' -e '}' "$STARSHIP_CONFIG"
    fi

    echo "" >> "$STARSHIP_CONFIG"
    echo "$palette_content" >> "$STARSHIP_CONFIG"

    if grep -q "^palette = " "$STARSHIP_CONFIG"; then
        sed -i "s/^palette = .*/palette = 'cruzalex'/" "$STARSHIP_CONFIG"
    fi
}

update_starship_palette
