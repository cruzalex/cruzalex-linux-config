//! UI rendering

use crate::app::App;
use crate::theme::ThemeStatus;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use ratatui_image::StatefulImage;

// Synthwave 80s palette — overrides terminal theme so the TUI chrome reads the
// same regardless of which theme is currently active.
const NEON_PINK: Color = Color::Rgb(255, 16, 240);
const NEON_PINK_SOFT: Color = Color::Rgb(255, 113, 206);
const NEON_CYAN: Color = Color::Rgb(1, 205, 254);
const NEON_LIME: Color = Color::Rgb(5, 255, 161);
const NEON_PURPLE: Color = Color::Rgb(185, 103, 255);
const NEON_YELLOW: Color = Color::Rgb(255, 251, 150);
const NEON_ORANGE: Color = Color::Rgb(255, 158, 100);
const PANEL_BG: Color = Color::Rgb(31, 13, 64);
const MUTED: Color = Color::Rgb(164, 138, 212);

/// Main draw function
pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Footer/status
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_main(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);

    if app.searching {
        draw_search_overlay(f, app);
    }
    if app.about_open {
        draw_about_modal(f, app);
    }
    if app.zoom_open {
        draw_zoom_modal(f, app);
    }
}

fn draw_about_modal(f: &mut Frame, app: &App) {
    let area = centered_rect_pct(70, 80, f.area());
    f.render_widget(Clear, area);

    let (active, installed, available, favorites) = app.counts();
    let total = app.themes.len();

    let lines = vec![
        Line::from(vec![Span::styled(
            "cruzAlex Themes",
            Style::default().fg(NEON_PINK).add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!("version {}", env!("CARGO_PKG_VERSION"))),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Library",
            Style::default().fg(NEON_YELLOW).add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!(
            "  {} total  |  {} active  |  {} installed  |  {} available  |  {} favorites",
            total, active, installed, available, favorites
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Theme sources",
            Style::default().fg(NEON_YELLOW).add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Pulled into this TUI:"),
        Line::from("    • aorumbayev/awesome-omarchy (curated, hardcoded)"),
        Line::from("    • GitHub topic:omarchy-theme (live search)"),
        Line::from("  Browse manually for more:"),
        Line::from("    • omarchythemes.com"),
        Line::from("    • Wheel-Smith/awesome-omarchy (built-in themes)"),
        Line::from("    • learn.omacom.io (official Omarchy docs)"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Keybindings",
            Style::default().fg(NEON_YELLOW).add_modifier(Modifier::BOLD),
        )]),
        Line::from("  j/k ↑/↓        Navigate"),
        Line::from("  Enter          Apply theme"),
        Line::from("  i              Install (for Available themes)"),
        Line::from("  x              Delete installed theme"),
        Line::from("  f              Toggle favorite"),
        Line::from("  /              Search"),
        Line::from("  Tab            Cycle filter (All/Installed/Available/Favorites)"),
        Line::from("  s              Cycle sort (Name/Stars)"),
        Line::from("  p              Toggle preview panel"),
        Line::from("  z              Zoom preview"),
        Line::from("  r              Refresh remote themes"),
        Line::from("  ?              About (this screen)"),
        Line::from("  q / Esc        Quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Paths",
            Style::default().fg(NEON_YELLOW).add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!("  themes:  {}", app.themes_dir.display())),
        Line::from(format!("  config:  {}", app.config_dir.display())),
        Line::from(format!("  cache:   {}", app.cache_dir.display())),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press ? or Esc to close",
            Style::default().fg(MUTED),
        )]),
    ];

    let block = Block::default()
        .title(" About ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(NEON_PINK));
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn draw_zoom_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect_pct(85, 90, f.area());
    f.render_widget(Clear, area);

    let theme_name = app
        .selected_theme()
        .map(|t| t.display_name.clone())
        .unwrap_or_default();
    let title = format!(" Preview — {}  (press z or Esc to close) ", theme_name);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(NEON_CYAN));
    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(image) = app.current_preview_image.as_mut() {
        f.render_stateful_widget(StatefulImage::new(None), inner, image);
    } else {
        let msg = if app.image_loading {
            "Loading preview..."
        } else {
            "No preview available for this theme."
        };
        let p = Paragraph::new(msg)
            .alignment(Alignment::Center)
            .style(Style::default().fg(MUTED));
        f.render_widget(p, inner);
    }
}

fn centered_rect_pct(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

/// Draw header with title, filter and sort
fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        " cruzAlex Themes | {} themes | Filter: {} | Sort: {} ",
        app.filtered_themes.len(),
        app.filter_mode.label(),
        app.sort_mode.label()
    );

    let loading = if app.loading { " [Loading...]" } else { "" };

    let header = Paragraph::new(Line::from(vec![
        Span::styled(title, Style::default().fg(NEON_CYAN)),
        Span::styled(loading, Style::default().fg(NEON_YELLOW)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(NEON_CYAN)),
    );

    f.render_widget(header, area);
}

/// Draw main content area
fn draw_main(f: &mut Frame, app: &mut App, area: Rect) {
    if app.show_preview {
        // Split into list and preview
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        draw_theme_list(f, app, chunks[0]);
        draw_preview(f, app, chunks[1]);
    } else {
        draw_theme_list(f, app, area);
    }
}

/// Draw theme list with scrolling support
fn draw_theme_list(f: &mut Frame, app: &mut App, area: Rect) {
    // Pre-collect theme data to avoid borrow issues
    let theme_data: Vec<_> = app
        .filtered_themes
        .iter()
        .map(|&theme_idx| {
            let theme = &app.themes[theme_idx];
            (
                theme.name.clone(),
                theme.display_name.clone(),
                theme.status.clone(),
                theme.is_light,
                theme.background_count,
                app.favorites.contains(&theme.name),
                theme.stars,
            )
        })
        .collect();

    let items: Vec<ListItem> = theme_data
        .iter()
        .map(|(_, display_name, status, is_light, bg_count, is_fav, stars)| {
            // Favorite star
            let fav_icon = if *is_fav {
                Span::styled("★ ", Style::default().fg(NEON_PINK))
            } else {
                Span::raw("  ")
            };

            let status_icon = match status {
                ThemeStatus::Active => Span::styled("● ", Style::default().fg(NEON_LIME)),
                ThemeStatus::Installed => Span::styled("○ ", Style::default().fg(NEON_PURPLE)),
                ThemeStatus::Available => Span::styled("◌ ", Style::default().fg(MUTED)),
            };

            let name = Span::styled(
                display_name.as_str(),
                Style::default().fg(Color::White),
            );

            let light_icon = if *is_light {
                Span::styled(" [light]", Style::default().fg(NEON_YELLOW))
            } else {
                Span::raw("")
            };

            let bg_count_span = if *bg_count > 0 {
                Span::styled(
                    format!(" ({} bg)", bg_count),
                    Style::default().fg(MUTED),
                )
            } else {
                Span::raw("")
            };

            // GitHub stars
            let stars_span = if let Some(s) = stars {
                Span::styled(
                    format!(" ⭐{}", s),
                    Style::default().fg(NEON_YELLOW),
                )
            } else {
                Span::raw("")
            };

            ListItem::new(Line::from(vec![fav_icon, status_icon, name, light_icon, bg_count_span, stars_span]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Themes (j/k to navigate, Enter to apply) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NEON_PURPLE)),
        )
        .highlight_style(
            Style::default()
                .bg(PANEL_BG)
                .fg(NEON_YELLOW)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // Use stateful widget for scrolling
    f.render_stateful_widget(list, area, &mut app.list_state);
}

/// Draw theme preview
fn draw_preview(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(NEON_PINK));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(theme) = app.selected_theme().cloned() {
        // Calculate layout based on whether we have an image or could have one (remote URL)
        let has_preview = theme.preview_path.is_some() || theme.preview_url.is_some() || app.image_loading;

        let chunks = if has_preview {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),  // Name
                    Constraint::Length(6),  // Color palette
                    Constraint::Min(8),     // Image preview area
                    Constraint::Length(4),  // Info
                ])
                .split(inner)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),  // Name
                    Constraint::Min(6),     // Color palette
                    Constraint::Length(5),  // Info
                ])
                .split(inner)
        };

        // Theme name
        let name = Paragraph::new(theme.display_name.clone())
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(name, chunks[0]);

        // Color palette preview
        if let Some(colors) = &theme.colors {
            draw_color_palette(f, colors, chunks[1]);
        } else {
            let no_colors = Paragraph::new("No color palette available")
                .style(Style::default().fg(MUTED))
                .alignment(Alignment::Center);
            f.render_widget(no_colors, chunks[1]);
        }

        // Image preview area
        if has_preview {
            let preview_area = chunks[2];

            // Try to render actual image if loaded
            if let Some(protocol) = &mut app.current_preview_image {
                let image = StatefulImage::new(None);
                f.render_stateful_widget(image, preview_area, protocol);
            } else if app.image_loading {
                // Show loading indicator
                let loading = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "Loading preview...",
                        Style::default().fg(NEON_YELLOW),
                    )),
                ])
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(MUTED)));
                f.render_widget(loading, preview_area);
            } else if theme.preview_path.is_some() || theme.preview_url.is_some() {
                // Fallback: show message that preview is being fetched or couldn't be loaded
                let msg = if theme.preview_url.is_some() && theme.preview_path.is_none() {
                    "Fetching preview..."
                } else {
                    "Preview unavailable"
                };
                let preview_text = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        msg,
                        Style::default().fg(MUTED),
                    )),
                ])
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(MUTED)));
                f.render_widget(preview_text, preview_area);
            } else {
                // No preview available at all
                let preview_text = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(Span::styled(
                        "No preview available",
                        Style::default().fg(MUTED),
                    )),
                ])
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(MUTED)));
                f.render_widget(preview_text, preview_area);
            }
        }

        // Theme info
        let info_chunk = if has_preview { chunks[3] } else { chunks[2] };
        let mut info_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(MUTED)),
                Span::styled(
                    theme.status.label(),
                    Style::default().fg(match theme.status {
                        ThemeStatus::Active => NEON_LIME,
                        ThemeStatus::Installed => NEON_PURPLE,
                        ThemeStatus::Available => NEON_YELLOW,
                    }),
                ),
            ]),
        ];

        if let Some(author) = &theme.author {
            info_lines.push(Line::from(vec![
                Span::styled("Author: ", Style::default().fg(MUTED)),
                Span::styled(author, Style::default().fg(Color::White)),
            ]));
        }

        if theme.background_count > 0 {
            info_lines.push(Line::from(vec![
                Span::styled("Backgrounds: ", Style::default().fg(MUTED)),
                Span::styled(
                    format!("{}", theme.background_count),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        let info = Paragraph::new(info_lines).wrap(Wrap { trim: true });
        f.render_widget(info, info_chunk);
    } else {
        let empty = Paragraph::new("No theme selected")
            .style(Style::default().fg(MUTED))
            .alignment(Alignment::Center);
        f.render_widget(empty, inner);
    }
}

/// Draw color palette preview
fn draw_color_palette(f: &mut Frame, colors: &crate::theme::ColorPalette, area: Rect) {
    let all_colors = [
        ("bg", &colors.background),
        ("fg", &colors.foreground),
        ("ac", &colors.accent),
        ("0", &colors.color0),
        ("1", &colors.color1),
        ("2", &colors.color2),
        ("3", &colors.color3),
        ("4", &colors.color4),
        ("5", &colors.color5),
        ("6", &colors.color6),
        ("7", &colors.color7),
        ("8", &colors.color8),
        ("9", &colors.color9),
        ("10", &colors.color10),
        ("11", &colors.color11),
        ("12", &colors.color12),
        ("13", &colors.color13),
        ("14", &colors.color14),
        ("15", &colors.color15),
    ];

    let mut lines = Vec::new();

    // First row: bg, fg, accent
    let first_row: Vec<Span> = all_colors[0..3]
        .iter()
        .filter_map(|(label, color)| {
            color.as_ref().map(|c| {
                let rgb = parse_hex_color(c);
                Span::styled(
                    format!(" {} ", label),
                    Style::default().bg(rgb).fg(contrast_color(rgb)),
                )
            })
        })
        .collect();
    if !first_row.is_empty() {
        lines.push(Line::from(first_row));
    }

    // Colors 0-7 in one row
    let row_0_7: Vec<Span> = all_colors[3..11]
        .iter()
        .filter_map(|(label, color)| {
            color.as_ref().map(|c| {
                let rgb = parse_hex_color(c);
                Span::styled(
                    format!("{}", label),
                    Style::default().bg(rgb).fg(contrast_color(rgb)),
                )
            })
        })
        .collect();
    if !row_0_7.is_empty() {
        lines.push(Line::from(row_0_7));
    }

    // Colors 8-15 in another row
    let row_8_15: Vec<Span> = all_colors[11..19]
        .iter()
        .filter_map(|(label, color)| {
            color.as_ref().map(|c| {
                let rgb = parse_hex_color(c);
                Span::styled(
                    format!("{}", label),
                    Style::default().bg(rgb).fg(contrast_color(rgb)),
                )
            })
        })
        .collect();
    if !row_8_15.is_empty() {
        lines.push(Line::from(row_8_15));
    }

    let palette = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(palette, area);
}

/// Parse hex color to RGB
fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    Color::White
}

/// Get contrasting text color
fn contrast_color(bg: Color) -> Color {
    if let Color::Rgb(r, g, b) = bg {
        let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
        if luminance > 0.5 {
            Color::Black
        } else {
            Color::White
        }
    } else {
        Color::White
    }
}

/// Draw footer with keybindings and status
fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let keybindings = "[j/k] Nav [Enter] Apply [i] Install [f] Fav [Tab] Filter [s] Sort [/] Search [z] Zoom [r] Refresh [?] About [q] Quit";

    let status = app.status_message.as_deref().unwrap_or("");

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(keybindings, Style::default().fg(MUTED)),
        Span::styled(" | ", Style::default().fg(MUTED)),
        Span::styled(status, Style::default().fg(NEON_YELLOW)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MUTED)),
    );

    f.render_widget(footer, area);
}

/// Draw search overlay
fn draw_search_overlay(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 3, f.area());

    f.render_widget(Clear, area);

    let search = Paragraph::new(format!("Search: {}|", app.search_query))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(" Search (Enter to confirm, Esc to cancel) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NEON_YELLOW)),
        );

    f.render_widget(search, area);
}

/// Create centered rectangle
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
