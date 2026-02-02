#!/bin/bash
# Theme hook: Initialize wallpaper from new theme

CRUZALEX_DIR="${CRUZALEX_DIR:-$HOME/.config/cruzalex}"
THEME_DIR="${THEME_DIR:-$CRUZALEX_DIR/current}"
WALLPAPER_STATE="$CRUZALEX_DIR/.current-wallpaper"
WALLPAPER_INDEX="$CRUZALEX_DIR/.wallpaper-index"

# Get background color from theme for fallback
get_bg_color() {
    local colors_file="$THEME_DIR/colors.toml"
    if [ -f "$colors_file" ]; then
        grep '^background *=' "$colors_file" | head -1 | cut -d'"' -f2
    else
        echo "#193549"
    fi
}

# Find a wallpaper from theme
find_wallpaper() {
    # Check 'backgrounds' directory (Omarchy standard)
    local bg_dir="$THEME_DIR/backgrounds"
    if [ -d "$bg_dir" ]; then
        local wallpaper=$(find "$bg_dir" -type f \( -name "*.jpg" -o -name "*.jpeg" -o -name "*.png" -o -name "*.webp" \) 2>/dev/null | sort | head -1)
        if [ -n "$wallpaper" ]; then
            echo "$wallpaper"
            return 0
        fi
    fi

    # Check 'wallpapers' directory (alternative)
    bg_dir="$THEME_DIR/wallpapers"
    if [ -d "$bg_dir" ]; then
        local wallpaper=$(find "$bg_dir" -type f \( -name "*.jpg" -o -name "*.jpeg" -o -name "*.png" -o -name "*.webp" \) 2>/dev/null | sort | head -1)
        if [ -n "$wallpaper" ]; then
            echo "$wallpaper"
            return 0
        fi
    fi

    # Check for wallpaper.* in theme root
    for ext in png jpg jpeg webp; do
        if [ -f "$THEME_DIR/wallpaper.$ext" ]; then
            echo "$THEME_DIR/wallpaper.$ext"
            return 0
        fi
    done

    return 1
}

# Kill existing swaybg and wait for it to fully terminate
if pgrep -x swaybg > /dev/null; then
    # Graceful shutdown first
    pkill -TERM swaybg 2>/dev/null || true

    # Wait for swaybg to actually exit (up to 1 second)
    for i in {1..10}; do
        pgrep -x swaybg > /dev/null || break
        sleep 0.1
    done

    # Force kill if still running
    pgrep -x swaybg > /dev/null && pkill -9 swaybg 2>/dev/null || true
fi

# Reset wallpaper index for new theme
echo "0" > "$WALLPAPER_INDEX"

# Try to find and set wallpaper
wallpaper=$(find_wallpaper)

if [ -n "$wallpaper" ] && [ -f "$wallpaper" ]; then
    echo "$wallpaper" > "$WALLPAPER_STATE"
    swaybg -i "$wallpaper" -m fill &
    disown
else
    # Fallback to solid color
    bg_color=$(get_bg_color)
    echo "" > "$WALLPAPER_STATE"
    swaybg -c "$bg_color" &
    disown
fi
