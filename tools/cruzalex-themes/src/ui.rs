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

/// Main draw function
pub fn draw(f: &mut Frame, app: &App) {
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

    // Draw search overlay if searching
    if app.searching {
        draw_search_overlay(f, app);
    }
}

/// Draw header with title and filter
fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        " cruzAlex Themes │ {} themes │ Filter: {} ",
        app.filtered_themes.len(),
        app.filter_mode.label()
    );

    let loading = if app.loading { " ⟳ Loading..." } else { "" };

    let header = Paragraph::new(Line::from(vec![
        Span::styled(title, Style::default().fg(Color::Cyan)),
        Span::styled(loading, Style::default().fg(Color::Yellow)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(header, area);
}

/// Draw main content area
fn draw_main(f: &mut Frame, app: &App, area: Rect) {
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

/// Draw theme list
fn draw_theme_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_themes
        .iter()
        .enumerate()
        .map(|(i, &theme_idx)| {
            let theme = &app.themes[theme_idx];
            let is_selected = i == app.selected;

            let status_color = match theme.status {
                ThemeStatus::Active => Color::Green,
                ThemeStatus::Installed => Color::Blue,
                ThemeStatus::Available => Color::DarkGray,
            };

            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let line = Line::from(vec![
                Span::styled(format!("{} ", theme.status.symbol()), Style::default().fg(status_color)),
                Span::styled(&theme.display_name, style),
                if theme.is_light {
                    Span::styled(" ☀", Style::default().fg(Color::Yellow))
                } else {
                    Span::raw("")
                },
                if theme.background_count > 0 {
                    Span::styled(
                        format!(" ({} bg)", theme.background_count),
                        Style::default().fg(Color::DarkGray),
                    )
                } else {
                    Span::raw("")
                },
                if let Some(stars) = theme.stars {
                    Span::styled(format!(" ★{}", stars), Style::default().fg(Color::Yellow))
                } else {
                    Span::raw("")
                },
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Themes ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, area);
}

/// Draw theme preview
fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(theme) = app.selected_theme() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Name
                Constraint::Min(5),     // Color palette
                Constraint::Length(5),  // Info
            ])
            .split(inner);

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
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(no_colors, chunks[1]);
        }

        // Theme info
        let mut info_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    theme.status.label(),
                    Style::default().fg(match theme.status {
                        ThemeStatus::Active => Color::Green,
                        ThemeStatus::Installed => Color::Blue,
                        ThemeStatus::Available => Color::Yellow,
                    }),
                ),
            ]),
        ];

        if let Some(author) = &theme.author {
            info_lines.push(Line::from(vec![
                Span::styled("Author: ", Style::default().fg(Color::DarkGray)),
                Span::styled(author, Style::default().fg(Color::White)),
            ]));
        }

        if theme.background_count > 0 {
            info_lines.push(Line::from(vec![
                Span::styled("Backgrounds: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", theme.background_count),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        let info = Paragraph::new(info_lines).wrap(Wrap { trim: true });
        f.render_widget(info, chunks[2]);
    }
}

/// Draw color palette preview
fn draw_color_palette(f: &mut Frame, colors: &crate::theme::ColorPalette, area: Rect) {
    let all_colors = [
        ("bg", &colors.background),
        ("fg", &colors.foreground),
        ("acc", &colors.accent),
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

    for chunk in all_colors.chunks(6) {
        let spans: Vec<Span> = chunk
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

        if !spans.is_empty() {
            lines.push(Line::from(spans));
        }
    }

    let palette = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(palette, area);
}

/// Parse hex color to RGB
fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
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
    let keybindings = " [↑/k] Up  [↓/j] Down  [Enter] Apply  [i] Install  [d] Delete  [/] Search  [Tab] Filter  [p] Preview  [r] Refresh  [q] Quit ";

    let status = app.status_message.as_deref().unwrap_or("");

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(keybindings, Style::default().fg(Color::DarkGray)),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(status, Style::default().fg(Color::Yellow)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(footer, area);
}

/// Draw search overlay
fn draw_search_overlay(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 3, f.area());

    f.render_widget(Clear, area);

    let search = Paragraph::new(format!("Search: {}_", app.search_query))
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(" Search ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(search, area);
}

/// Create centered rectangle
fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - height) / 2),
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
