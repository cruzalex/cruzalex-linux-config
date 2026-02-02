//! cruzAlex Themes - TUI Theme Browser
//!
//! Browse, preview, and install Omarchy-compatible themes

mod app;
mod theme;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Refresh theme list from remote sources
    #[arg(short, long)]
    refresh: bool,

    /// Show only installed themes
    #[arg(short, long)]
    installed: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new().await?;

    // Initialize image picker for terminal graphics protocol detection
    app.init_image_picker();
    // Load preview for initial selection
    app.load_selected_preview();

    // Auto-refresh themes from GitHub on startup (unless --installed flag)
    if !args.installed {
        app.refresh_remote_themes().await?;
        // Fetch GitHub stars in background
        app.fetch_stars();
    }

    if args.installed {
        app.filter_installed();
    }

    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        // Process background task results
        app.tick()?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events, not release
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Handle search mode input separately
                if app.searching {
                    match key.code {
                        KeyCode::Esc => app.exit_search_mode(),
                        KeyCode::Enter => app.search_submit(),
                        KeyCode::Backspace => app.search_backspace(),
                        KeyCode::Char(c) => app.search_input(c),
                        _ => {}
                    }
                    continue;
                }

                // Normal mode key handling
                match (key.modifiers, key.code) {
                    // Quit - always works
                    (_, KeyCode::Char('q')) => return Ok(()),
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(()),
                    (_, KeyCode::Esc) => return Ok(()),

                    // Navigation
                    (_, KeyCode::Up) | (_, KeyCode::Char('k')) => app.previous(),
                    (_, KeyCode::Down) | (_, KeyCode::Char('j')) => app.next(),
                    (_, KeyCode::PageUp) | (KeyModifiers::CONTROL, KeyCode::Char('u')) => app.previous_page(),
                    (_, KeyCode::PageDown) | (KeyModifiers::CONTROL, KeyCode::Char('d')) => app.next_page(),
                    (_, KeyCode::Home) | (_, KeyCode::Char('g')) => app.first(),
                    (_, KeyCode::End) | (_, KeyCode::Char('G')) => app.last(),

                    // Actions
                    (_, KeyCode::Enter) => { app.apply_theme()?; }
                    (_, KeyCode::Char('i')) => app.install_theme(),
                    (_, KeyCode::Char('x')) => { app.delete_theme()?; }
                    (_, KeyCode::Char('r')) => { app.refresh_remote_themes().await?; }

                    // Favorites
                    (_, KeyCode::Char('f')) => app.toggle_favorite(),

                    // Search
                    (_, KeyCode::Char('/')) => app.enter_search_mode(),

                    // Filter
                    (_, KeyCode::Tab) => app.cycle_filter(),

                    // Sort
                    (_, KeyCode::Char('s')) => app.cycle_sort(),

                    // Preview toggle
                    (_, KeyCode::Char('p')) => app.toggle_preview(),

                    _ => {}
                }
            }
        }
    }
}
