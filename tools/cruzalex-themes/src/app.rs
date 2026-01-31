//! Application state and logic

use crate::theme::{fetch_github_themes, load_local_themes, Theme, ThemeStatus};
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// Filter mode for theme list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    All,
    Installed,
    Available,
}

impl FilterMode {
    pub fn label(&self) -> &str {
        match self {
            FilterMode::All => "All",
            FilterMode::Installed => "Installed",
            FilterMode::Available => "Available",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            FilterMode::All => FilterMode::Installed,
            FilterMode::Installed => FilterMode::Available,
            FilterMode::Available => FilterMode::All,
        }
    }
}

/// Application state
pub struct App {
    /// All themes (local + remote)
    pub themes: Vec<Theme>,
    /// Filtered themes based on current filter
    pub filtered_themes: Vec<usize>,
    /// Currently selected theme index (in filtered list)
    pub selected: usize,
    /// Current filter mode
    pub filter_mode: FilterMode,
    /// Search query
    pub search_query: String,
    /// Is search mode active?
    pub searching: bool,
    /// Show preview panel?
    pub show_preview: bool,
    /// Status message
    pub status_message: Option<String>,
    /// Themes directory
    pub themes_dir: PathBuf,
    /// Current theme name
    pub current_theme: Option<String>,
    /// Is loading?
    pub loading: bool,
    /// Columns for grid view
    pub columns: usize,
}

impl App {
    /// Create new app instance
    pub async fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("cruzalex");
        let themes_dir = config_dir.join("themes");

        // Get current theme
        let current_link = themes_dir.join("current");
        let current_theme = if current_link.is_symlink() {
            std::fs::read_link(&current_link)
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        } else {
            None
        };

        // Load local themes
        let local_themes = load_local_themes(&themes_dir, current_theme.as_deref())?;

        let mut app = Self {
            themes: local_themes,
            filtered_themes: Vec::new(),
            selected: 0,
            filter_mode: FilterMode::All,
            search_query: String::new(),
            searching: false,
            show_preview: true,
            status_message: Some("Press 'r' to refresh theme list from GitHub".to_string()),
            themes_dir,
            current_theme,
            loading: false,
            columns: 4,
        };

        app.update_filter();
        Ok(app)
    }

    /// Update filtered themes based on filter mode and search
    pub fn update_filter(&mut self) {
        self.filtered_themes = self
            .themes
            .iter()
            .enumerate()
            .filter(|(_, theme)| {
                // Filter by mode
                let mode_match = match self.filter_mode {
                    FilterMode::All => true,
                    FilterMode::Installed => {
                        matches!(theme.status, ThemeStatus::Active | ThemeStatus::Installed)
                    }
                    FilterMode::Available => matches!(theme.status, ThemeStatus::Available),
                };

                // Filter by search
                let search_match = if self.search_query.is_empty() {
                    true
                } else {
                    theme
                        .name
                        .to_lowercase()
                        .contains(&self.search_query.to_lowercase())
                        || theme
                            .display_name
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase())
                };

                mode_match && search_match
            })
            .map(|(i, _)| i)
            .collect();

        // Reset selection if out of bounds
        if self.selected >= self.filtered_themes.len() {
            self.selected = self.filtered_themes.len().saturating_sub(1);
        }
    }

    /// Get currently selected theme
    pub fn selected_theme(&self) -> Option<&Theme> {
        self.filtered_themes
            .get(self.selected)
            .and_then(|&i| self.themes.get(i))
    }

    /// Navigation
    pub fn next(&mut self) {
        if !self.filtered_themes.is_empty() {
            self.selected = (self.selected + 1) % self.filtered_themes.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.filtered_themes.is_empty() {
            self.selected = self
                .selected
                .checked_sub(1)
                .unwrap_or(self.filtered_themes.len() - 1);
        }
    }

    pub fn next_page(&mut self) {
        let page_size = self.columns * 2;
        self.selected = (self.selected + page_size).min(self.filtered_themes.len().saturating_sub(1));
    }

    pub fn previous_page(&mut self) {
        let page_size = self.columns * 2;
        self.selected = self.selected.saturating_sub(page_size);
    }

    pub fn first(&mut self) {
        self.selected = 0;
    }

    pub fn last(&mut self) {
        self.selected = self.filtered_themes.len().saturating_sub(1);
    }

    /// Filter cycling
    pub fn cycle_filter(&mut self) {
        self.filter_mode = self.filter_mode.next();
        self.update_filter();
    }

    pub fn filter_installed(&mut self) {
        self.filter_mode = FilterMode::Installed;
        self.update_filter();
    }

    /// Search
    pub fn enter_search_mode(&mut self) {
        self.searching = true;
        self.search_query.clear();
    }

    pub fn exit_search_mode(&mut self) {
        self.searching = false;
        self.search_query.clear();
        self.update_filter();
    }

    pub fn is_searching(&self) -> bool {
        self.searching
    }

    pub fn search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filter();
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.update_filter();
    }

    /// Toggle preview panel
    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
    }

    /// Apply selected theme
    pub async fn apply_theme(&mut self) -> Result<()> {
        let Some(theme) = self.selected_theme() else {
            return Ok(());
        };

        match theme.status {
            ThemeStatus::Available => {
                self.status_message = Some("Theme not installed. Press 'i' to install first.".to_string());
                return Ok(());
            }
            _ => {}
        }

        let theme_name = theme.name.clone();
        self.status_message = Some(format!("Applying theme: {}...", theme_name));

        // Run cruzalex-theme-set
        let output = Command::new("cruzalex-theme-set")
            .arg(&theme_name)
            .output()
            .context("Failed to run cruzalex-theme-set")?;

        if output.status.success() {
            self.status_message = Some(format!("Theme '{}' applied!", theme_name));
            self.current_theme = Some(theme_name.clone());

            // Update theme statuses
            for theme in &mut self.themes {
                if theme.name == theme_name {
                    theme.status = ThemeStatus::Active;
                } else if matches!(theme.status, ThemeStatus::Active) {
                    theme.status = ThemeStatus::Installed;
                }
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.status_message = Some(format!("Failed to apply theme: {}", stderr));
        }

        Ok(())
    }

    /// Install selected theme
    pub async fn install_theme(&mut self) -> Result<()> {
        let Some(theme) = self.selected_theme() else {
            return Ok(());
        };

        if !matches!(theme.status, ThemeStatus::Available) {
            self.status_message = Some("Theme already installed.".to_string());
            return Ok(());
        }

        let Some(url) = &theme.remote_url else {
            self.status_message = Some("No remote URL for theme.".to_string());
            return Ok(());
        };

        let theme_name = theme.name.clone();
        let url = url.clone();
        self.status_message = Some(format!("Installing theme: {}...", theme_name));
        self.loading = true;

        // Clone the repository
        let dest = self.themes_dir.join(&theme_name);

        let result = tokio::task::spawn_blocking(move || {
            git2::Repository::clone(&url, &dest)
        }).await?;

        self.loading = false;

        match result {
            Ok(_) => {
                self.status_message = Some(format!("Theme '{}' installed!", theme_name));

                // Reload local themes
                let local_themes = load_local_themes(&self.themes_dir, self.current_theme.as_deref())?;

                // Merge with existing remote themes
                let remote_themes: Vec<Theme> = self.themes
                    .iter()
                    .filter(|t| matches!(t.status, ThemeStatus::Available))
                    .filter(|t| t.name != theme_name)
                    .cloned()
                    .collect();

                self.themes = local_themes;
                for remote in remote_themes {
                    if !self.themes.iter().any(|t| t.name == remote.name) {
                        self.themes.push(remote);
                    }
                }
                self.themes.sort_by(|a, b| a.name.cmp(&b.name));
                self.update_filter();
            }
            Err(e) => {
                self.status_message = Some(format!("Failed to install: {}", e));
            }
        }

        Ok(())
    }

    /// Delete selected theme
    pub fn delete_theme(&mut self) -> Result<()> {
        let Some(theme) = self.selected_theme() else {
            return Ok(());
        };

        if matches!(theme.status, ThemeStatus::Active) {
            self.status_message = Some("Cannot delete active theme.".to_string());
            return Ok(());
        }

        if matches!(theme.status, ThemeStatus::Available) {
            self.status_message = Some("Theme not installed.".to_string());
            return Ok(());
        }

        let Some(path) = &theme.local_path else {
            return Ok(());
        };

        let theme_name = theme.name.clone();
        std::fs::remove_dir_all(path)?;

        self.status_message = Some(format!("Theme '{}' deleted.", theme_name));

        // Update theme list
        if let Some(idx) = self.filtered_themes.get(self.selected) {
            if let Some(theme) = self.themes.get_mut(*idx) {
                theme.status = ThemeStatus::Available;
                theme.local_path = None;
                theme.preview_path = None;
            }
        }

        Ok(())
    }

    /// Refresh themes from GitHub
    pub async fn refresh_remote_themes(&mut self) -> Result<()> {
        self.status_message = Some("Fetching themes from GitHub...".to_string());
        self.loading = true;

        match fetch_github_themes().await {
            Ok(remote_themes) => {
                // Keep installed themes, add new remote ones
                let installed_names: std::collections::HashSet<String> = self.themes
                    .iter()
                    .filter(|t| matches!(t.status, ThemeStatus::Active | ThemeStatus::Installed))
                    .map(|t| t.name.clone())
                    .collect();

                for remote in remote_themes {
                    if !installed_names.contains(&remote.name) {
                        self.themes.push(remote);
                    }
                }

                self.themes.sort_by(|a, b| a.name.cmp(&b.name));
                self.update_filter();
                self.status_message = Some(format!("Found {} themes", self.themes.len()));
            }
            Err(e) => {
                self.status_message = Some(format!("Failed to fetch themes: {}", e));
            }
        }

        self.loading = false;
        Ok(())
    }

    /// Tick for async updates
    pub async fn tick(&mut self) -> Result<()> {
        // Could be used for async image loading, etc.
        Ok(())
    }
}
