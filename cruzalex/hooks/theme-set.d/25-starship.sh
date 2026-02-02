#!/bin/bash
# Theme hook: Update starship prompt colors
# This generates a cruzalex palette based on the current theme
# with proper contrast handling for light/dark backgrounds

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
        COLOR0=$(grep '^color0 *=' "$colors_file" | head -1 | cut -d'"' -f2)  # black
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
    COLOR0="${COLOR0:-#15161e}"
    COLOR1="${COLOR1:-#f7768e}"
    COLOR2="${COLOR2:-#9ece6a}"
    COLOR3="${COLOR3:-#e0af68}"
    COLOR4="${COLOR4:-#7aa2f7}"
    COLOR5="${COLOR5:-#bb9af7}"
    COLOR6="${COLOR6:-#7dcfff}"
    COLOR8="${COLOR8:-#414868}"
}

# Calculate relative luminance of a color (0-255 scale)
# Returns 1 if light, 0 if dark
is_light_color() {
    local hex="$1"
    hex="${hex#\#}"
    local r=$((16#${hex:0:2}))
    local g=$((16#${hex:2:2}))
    local b=$((16#${hex:4:2}))

    # Calculate perceived luminance (weighted for human perception)
    # Formula: 0.299*R + 0.587*G + 0.114*B
    local luminance=$(( (299 * r + 587 * g + 114 * b) / 1000 ))

    # Threshold at 140 (out of 255) - colors above this need dark text
    if [ "$luminance" -gt 140 ]; then
        echo "1"
    else
        echo "0"
    fi
}

# Generate darker version of a color for backgrounds
darken_color() {
    local hex="$1"
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

# Get contrasting foreground for a background color
get_contrast_fg() {
    local bg_color="$1"
    if [ "$(is_light_color "$bg_color")" = "1" ]; then
        echo "#1a1b26"  # Dark text for light backgrounds
    else
        echo "#fbf1c7"  # Light text for dark backgrounds
    fi
}

update_starship_palette() {
    get_colors

    # Calculate background colors
    local bg1=$(darken_color "$COLOR8")
    local bg3="$COLOR8"

    # Calculate contrast foregrounds for each segment color
    local fg_orange=$(get_contrast_fg "$ACCENT")
    local fg_yellow=$(get_contrast_fg "$COLOR3")
    local fg_aqua=$(get_contrast_fg "$COLOR6")
    local fg_blue=$(get_contrast_fg "$COLOR4")
    local fg_bg3=$(get_contrast_fg "$bg3")
    local fg_bg1=$(get_contrast_fg "$bg1")

    # Create the cruzalex palette with contrast-aware foregrounds
    local palette_content="[palettes.cruzalex]
# Light foreground for dark backgrounds
color_fg0 = '#fbf1c7'
# Dark foreground for light backgrounds
color_fg_dark = '#1a1b26'
# Segment-specific foregrounds (contrast-aware)
color_fg_orange = '$fg_orange'
color_fg_yellow = '$fg_yellow'
color_fg_aqua = '$fg_aqua'
color_fg_blue = '$fg_blue'
color_fg_bg3 = '$fg_bg3'
color_fg_bg1 = '$fg_bg1'
# Background colors
color_bg1 = '$bg1'
color_bg3 = '$bg3'
# Accent colors
color_blue = '$COLOR4'
color_aqua = '$COLOR6'
color_green = '$COLOR2'
color_orange = '$ACCENT'
color_purple = '$COLOR5'
color_red = '$COLOR1'
color_yellow = '$COLOR3'"

    # Check if cruzalex palette already exists
    if grep -q '^\[palettes\.cruzalex\]' "$STARSHIP_CONFIG"; then
        # Remove existing cruzalex palette section
        sed -i '/^\[palettes\.cruzalex\]/,/^\[/{/^\[palettes\.cruzalex\]/d;/^\[/!d}' "$STARSHIP_CONFIG"
    fi

    # Append the new palette
    echo "" >> "$STARSHIP_CONFIG"
    echo "$palette_content" >> "$STARSHIP_CONFIG"

    # Update the palette selection to use cruzalex
    if grep -q "^palette = " "$STARSHIP_CONFIG"; then
        sed -i "s/^palette = .*/palette = 'cruzalex'/" "$STARSHIP_CONFIG"
    fi

    # Update format strings to use contrast-aware foregrounds
    # Handle both "fg:X bg:Y" and "bg:Y fg:X" patterns

    # Orange segments (os, username)
    sed -i 's/fg:color_fg0 bg:color_orange/fg:color_fg_orange bg:color_orange/g' "$STARSHIP_CONFIG"
    sed -i 's/bg:color_orange fg:color_fg0/bg:color_orange fg:color_fg_orange/g' "$STARSHIP_CONFIG"

    # Yellow segments (directory)
    sed -i 's/fg:color_fg0 bg:color_yellow/fg:color_fg_yellow bg:color_yellow/g' "$STARSHIP_CONFIG"
    sed -i 's/bg:color_yellow fg:color_fg0/bg:color_yellow fg:color_fg_yellow/g' "$STARSHIP_CONFIG"

    # Aqua segments (git)
    sed -i 's/fg:color_fg0 bg:color_aqua/fg:color_fg_aqua bg:color_aqua/g' "$STARSHIP_CONFIG"
    sed -i 's/bg:color_aqua fg:color_fg0/bg:color_aqua fg:color_fg_aqua/g' "$STARSHIP_CONFIG"

    # Blue segments (languages)
    sed -i 's/fg:color_fg0 bg:color_blue/fg:color_fg_blue bg:color_blue/g' "$STARSHIP_CONFIG"
    sed -i 's/bg:color_blue fg:color_fg0/bg:color_blue fg:color_fg_blue/g' "$STARSHIP_CONFIG"

    # bg3 segments (docker, conda)
    sed -i 's/fg:color_fg0 bg:color_bg3/fg:color_fg_bg3 bg:color_bg3/g' "$STARSHIP_CONFIG"
    sed -i 's/bg:color_bg3 fg:color_fg0/bg:color_bg3 fg:color_fg_bg3/g' "$STARSHIP_CONFIG"

    # bg1 segments (time)
    sed -i 's/fg:color_fg0 bg:color_bg1/fg:color_fg_bg1 bg:color_bg1/g' "$STARSHIP_CONFIG"
    sed -i 's/bg:color_bg1 fg:color_fg0/bg:color_bg1 fg:color_fg_bg1/g' "$STARSHIP_CONFIG"
}

update_starship_palette
