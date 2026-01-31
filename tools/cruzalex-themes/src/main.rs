//! cruzAlex Themes - TUI Theme Browser
//!
//! Browse, preview, and install Omarchy-compatible themes
//! with image preview support using Kitty graphics protocol.

mod app;
mod theme;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
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

    if args.refresh {
        app.refresh_remote_themes().await?;
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

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    // Quit
                    (_, KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        return Ok(());
                    }

                    // Navigation
                    (_, KeyCode::Up) | (_, KeyCode::Char('k')) => app.previous(),
                    (_, KeyCode::Down) | (_, KeyCode::Char('j')) => app.next(),
                    (_, KeyCode::Left) | (_, KeyCode::Char('h')) => app.previous_page(),
                    (_, KeyCode::Right) | (_, KeyCode::Char('l')) => app.next_page(),
                    (_, KeyCode::Home) | (_, KeyCode::Char('g')) => app.first(),
                    (_, KeyCode::End) | (_, KeyCode::Char('G')) => app.last(),

                    // Actions
                    (_, KeyCode::Enter) => app.apply_theme().await?,
                    (_, KeyCode::Char('i')) => app.install_theme().await?,
                    (_, KeyCode::Char('d')) => app.delete_theme()?,
                    (_, KeyCode::Char('r')) => app.refresh_remote_themes().await?,

                    // Search
                    (_, KeyCode::Char('/')) => app.enter_search_mode(),
                    (_, KeyCode::Esc) => app.exit_search_mode(),
                    (_, KeyCode::Backspace) if app.is_searching() => app.search_backspace(),
                    (_, KeyCode::Char(c)) if app.is_searching() => app.search_input(c),

                    // Filter
                    (_, KeyCode::Tab) => app.cycle_filter(),

                    // Preview toggle
                    (_, KeyCode::Char('p')) => app.toggle_preview(),

                    _ => {}
                }
            }
        }

        // Handle async operations
        app.tick().await?;
    }
}
