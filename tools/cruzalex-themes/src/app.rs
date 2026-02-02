//! Application state and logic

use crate::theme::{fetch_github_themes, load_local_themes, Theme, ThemeStatus};
use anyhow::{Context, Result};
use image::ImageReader;
use ratatui::widgets::ListState;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::Command;
use tokio::sync::mpsc;

/// Filter mode for theme list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    All,
    Installed,
    Available,
    Favorites,
}

impl FilterMode {
    pub fn label(&self) -> &str {
        match self {
            FilterMode::All => "All",
            FilterMode::Installed => "Installed",
            FilterMode::Available => "Available",
            FilterMode::Favorites => "Favorites",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            FilterMode::All => FilterMode::Installed,
            FilterMode::Installed => FilterMode::Available,
            FilterMode::Available => FilterMode::Favorites,
            FilterMode::Favorites => FilterMode::All,
        }
    }
}

/// Sort mode for theme list
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Name,
    Stars,
}

impl SortMode {
    pub fn label(&self) -> &str {
        match self {
            SortMode::Name => "Name",
            SortMode::Stars => "Stars",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SortMode::Name => SortMode::Stars,
            SortMode::Stars => SortMode::Name,
        }
    }
}

/// Background task result
pub enum TaskResult {
    InstallComplete(String, Result<(), String>),
    ImageLoaded(PathBuf, Result<StatefulProtocol, String>),
    PreviewDownloaded(String, Result<PathBuf, String>),
    StarsFetched(std::collections::HashMap<String, u32>),
}

/// Application state
pub struct App {
    /// All themes (local + remote)
    pub themes: Vec<Theme>,
    /// Filtered themes based on current filter
    pub filtered_themes: Vec<usize>,
    /// List state for scrolling
    pub list_state: ListState,
    /// Current filter mode
    pub filter_mode: FilterMode,
    /// Current sort mode
    pub sort_mode: SortMode,
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
    /// Config directory
    pub config_dir: PathBuf,
    /// Cache directory for downloaded previews
    pub cache_dir: PathBuf,
    /// Current theme name
    pub current_theme: Option<String>,
    /// Is loading?
    pub loading: bool,
    /// Favorite themes
    pub favorites: HashSet<String>,
    /// Channel receiver for background tasks
    task_rx: mpsc::Receiver<TaskResult>,
    /// Channel sender for background tasks
    task_tx: mpsc::Sender<TaskResult>,
    /// Image picker for terminal graphics protocol detection
    pub image_picker: Option<Picker>,
    /// Current preview image (rendered protocol)
    pub current_preview_image: Option<StatefulProtocol>,
    /// Path of the currently loaded preview image
    pub current_preview_path: Option<PathBuf>,
    /// Is an image currently loading?
    pub image_loading: bool,
}

impl App {
    /// Create new app instance
    pub async fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("cruzalex");
        let themes_dir = config_dir.join("themes");

        // Cache directory for downloaded previews
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("cruzalex/previews");
        std::fs::create_dir_all(&cache_dir).ok();

        // Get current theme (symlink is at ~/.config/cruzalex/current, not in themes dir)
        let current_link = config_dir.join("current");
        let current_theme = if current_link.is_symlink() {
            std::fs::read_link(&current_link)
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        } else {
            None
        };

        // Load local themes
        let local_themes = load_local_themes(&themes_dir, current_theme.as_deref())?;

        // Load favorites
        let favorites = load_favorites(&config_dir);

        // Create channel for background tasks
        let (task_tx, task_rx) = mpsc::channel(10);

        let mut app = Self {
            themes: local_themes,
            filtered_themes: Vec::new(),
            list_state: ListState::default(),
            filter_mode: FilterMode::All,
            sort_mode: SortMode::Name,
            search_query: String::new(),
            searching: false,
            show_preview: true,
            status_message: Some("Loading themes...".to_string()),
            themes_dir,
            config_dir,
            cache_dir,
            current_theme,
            loading: false,
            favorites,
            task_rx,
            task_tx,
            image_picker: None,
            current_preview_image: None,
            current_preview_path: None,
            image_loading: false,
        };

        app.update_filter();
        // Select first item
        if !app.filtered_themes.is_empty() {
            app.list_state.select(Some(0));
        }
        Ok(app)
    }

    /// Get selected index
    pub fn selected(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    /// Check if theme is favorite
    pub fn is_favorite(&self, name: &str) -> bool {
        self.favorites.contains(name)
    }

    /// Toggle favorite for selected theme
    pub fn toggle_favorite(&mut self) {
        if let Some(theme) = self.selected_theme() {
            let name = theme.name.clone();
            if self.favorites.contains(&name) {
                self.favorites.remove(&name);
                self.status_message = Some(format!("Removed '{}' from favorites", name));
            } else {
                self.favorites.insert(name.clone());
                self.status_message = Some(format!("Added '{}' to favorites", name));
            }
            save_favorites(&self.config_dir, &self.favorites);
            self.update_filter();
        }
    }

    /// Update filtered themes based on filter mode, search, and sort
    pub fn update_filter(&mut self) {
        let mut filtered: Vec<usize> = self
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
                    FilterMode::Favorites => self.favorites.contains(&theme.name),
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

        // Apply sorting
        match self.sort_mode {
            SortMode::Name => {
                filtered.sort_by(|&a, &b| self.themes[a].name.cmp(&self.themes[b].name));
            }
            SortMode::Stars => {
                // Sort by stars descending, then by name
                filtered.sort_by(|&a, &b| {
                    let stars_a = self.themes[a].stars.unwrap_or(0);
                    let stars_b = self.themes[b].stars.unwrap_or(0);
                    stars_b.cmp(&stars_a).then_with(|| self.themes[a].name.cmp(&self.themes[b].name))
                });
            }
        }

        self.filtered_themes = filtered;

        // Reset selection if out of bounds
        let selected = self.selected();
        if selected >= self.filtered_themes.len() {
            let new_selected = self.filtered_themes.len().saturating_sub(1);
            self.list_state.select(if self.filtered_themes.is_empty() {
                None
            } else {
                Some(new_selected)
            });
        }
    }

    /// Get currently selected theme
    pub fn selected_theme(&self) -> Option<&Theme> {
        self.list_state
            .selected()
            .and_then(|i| self.filtered_themes.get(i))
            .and_then(|&idx| self.themes.get(idx))
    }

    /// Navigation
    pub fn next(&mut self) {
        if self.filtered_themes.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_themes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.load_selected_preview();
    }

    pub fn previous(&mut self) {
        if self.filtered_themes.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_themes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.load_selected_preview();
    }

    pub fn next_page(&mut self) {
        if self.filtered_themes.is_empty() {
            return;
        }
        let page_size = 10;
        let i = self.list_state.selected().unwrap_or(0);
        let new_i = (i + page_size).min(self.filtered_themes.len() - 1);
        self.list_state.select(Some(new_i));
        self.load_selected_preview();
    }

    pub fn previous_page(&mut self) {
        if self.filtered_themes.is_empty() {
            return;
        }
        let page_size = 10;
        let i = self.list_state.selected().unwrap_or(0);
        let new_i = i.saturating_sub(page_size);
        self.list_state.select(Some(new_i));
        self.load_selected_preview();
    }

    pub fn first(&mut self) {
        if !self.filtered_themes.is_empty() {
            self.list_state.select(Some(0));
            self.load_selected_preview();
        }
    }

    pub fn last(&mut self) {
        if !self.filtered_themes.is_empty() {
            self.list_state.select(Some(self.filtered_themes.len() - 1));
            self.load_selected_preview();
        }
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

    /// Sort cycling
    pub fn cycle_sort(&mut self) {
        self.sort_mode = self.sort_mode.next();
        self.update_filter();
        self.status_message = Some(format!("Sort: {}", self.sort_mode.label()));
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

    pub fn search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filter();
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.update_filter();
    }

    pub fn search_submit(&mut self) {
        self.searching = false;
        // Keep the search query active
    }

    /// Toggle preview panel
    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
    }

    /// Apply selected theme
    pub fn apply_theme(&mut self) -> Result<()> {
        if self.loading {
            self.status_message = Some("Please wait, operation in progress...".to_string());
            return Ok(());
        }

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

        // Run cruzalex-theme-set (use full path for reliability)
        let theme_set_cmd = dirs::config_dir()
            .map(|d| d.join("cruzalex/bin/cruzalex-theme-set"))
            .unwrap_or_else(|| std::path::PathBuf::from("cruzalex-theme-set"));

        let output = Command::new(&theme_set_cmd)
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
            self.status_message = Some(format!("Failed: {}", stderr.chars().take(50).collect::<String>()));
        }

        Ok(())
    }

    /// Install selected theme (non-blocking)
    pub fn install_theme(&mut self) {
        if self.loading {
            self.status_message = Some("Please wait, installation in progress...".to_string());
            return;
        }

        let Some(theme) = self.selected_theme() else {
            return;
        };

        if !matches!(theme.status, ThemeStatus::Available) {
            self.status_message = Some("Theme already installed.".to_string());
            return;
        }

        let Some(url) = &theme.remote_url else {
            self.status_message = Some("No remote URL for theme.".to_string());
            return;
        };

        let theme_name = theme.name.clone();
        let url = url.clone();
        let dest = self.themes_dir.join(&theme_name);
        let tx = self.task_tx.clone();

        self.status_message = Some(format!("Installing '{}'... (please wait)", theme_name));
        self.loading = true;

        // Spawn background task
        tokio::spawn(async move {
            let result = tokio::task::spawn_blocking(move || {
                git2::Repository::clone(&url, &dest)
            }).await;

            let msg = match result {
                Ok(Ok(_)) => Ok(()),
                Ok(Err(e)) => Err(format!("Git error: {}", e)),
                Err(e) => Err(format!("Task error: {}", e)),
            };

            let _ = tx.send(TaskResult::InstallComplete(theme_name, msg)).await;
        });
    }

    /// Delete selected theme
    pub fn delete_theme(&mut self) -> Result<()> {
        if self.loading {
            self.status_message = Some("Please wait, operation in progress...".to_string());
            return Ok(());
        }

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
        let selected = self.selected();
        if let Some(&idx) = self.filtered_themes.get(selected) {
            if let Some(theme) = self.themes.get_mut(idx) {
                theme.status = ThemeStatus::Available;
                theme.local_path = None;
                theme.preview_path = None;
                theme.colors = None;
            }
        }

        Ok(())
    }

    /// Refresh themes from GitHub
    pub async fn refresh_remote_themes(&mut self) -> Result<()> {
        self.status_message = Some("Fetching themes...".to_string());
        self.loading = true;

        match fetch_github_themes().await {
            Ok(remote_themes) => {
                // Keep installed themes, add new remote ones
                let installed_names: HashSet<String> = self.themes
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
                self.status_message = Some(format!("Failed to fetch: {}", e));
            }
        }

        self.loading = false;
        Ok(())
    }

    /// Process background task results
    pub fn tick(&mut self) -> Result<()> {
        // Check for completed background tasks
        while let Ok(result) = self.task_rx.try_recv() {
            match result {
                TaskResult::InstallComplete(theme_name, res) => {
                    self.loading = false;
                    match res {
                        Ok(()) => {
                            self.status_message = Some(format!("Theme '{}' installed!", theme_name));

                            // Reload local themes
                            if let Ok(local_themes) = load_local_themes(&self.themes_dir, self.current_theme.as_deref()) {
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
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Install failed: {}", e));
                        }
                    }
                }
                TaskResult::ImageLoaded(path, res) => {
                    self.image_loading = false;
                    // Only use the image if it's still the one we're expecting
                    if self.current_preview_path.as_ref() == Some(&path) {
                        self.current_preview_image = res.ok();
                    }
                }
                TaskResult::PreviewDownloaded(theme_name, res) => {
                    match res {
                        Ok(cached_path) => {
                            // Update the theme's preview_path with the cached file
                            if let Some(theme) = self.themes.iter_mut().find(|t| t.name == theme_name) {
                                theme.preview_path = Some(cached_path.clone());
                            }
                            // If this is the currently selected theme, trigger image load
                            if let Some(selected) = self.selected_theme() {
                                if selected.name == theme_name {
                                    self.load_selected_preview();
                                }
                            }
                        }
                        Err(_) => {
                            // Download failed - clear preview_url so we don't try again
                            // and clear loading state
                            if let Some(theme) = self.themes.iter_mut().find(|t| t.name == theme_name) {
                                theme.preview_url = None;
                            }
                            // Clear loading state if this was for the current theme
                            if let Some(selected) = self.selected_theme() {
                                if selected.name == theme_name {
                                    self.image_loading = false;
                                }
                            }
                        }
                    }
                }
                TaskResult::StarsFetched(stars_map) => {
                    // Update stars for all themes
                    for theme in &mut self.themes {
                        if let Some(&stars) = stars_map.get(&theme.name) {
                            theme.stars = Some(stars);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Fetch GitHub stars for all themes in background
    pub fn fetch_stars(&self) {
        let tx = self.task_tx.clone();
        let themes: Vec<(String, Option<String>)> = self.themes
            .iter()
            .filter(|t| t.remote_url.is_some())
            .map(|t| (t.name.clone(), t.remote_url.clone()))
            .collect();

        tokio::spawn(async move {
            let mut stars_map = std::collections::HashMap::new();

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default();

            // Fetch stars using GitHub search API (more efficient than individual requests)
            // Rate limit is 10 requests/minute for unauthenticated, so batch them
            for chunk in themes.chunks(30) {
                for (name, url) in chunk {
                    if let Some(url) = url {
                        if let Ok(stars) = fetch_repo_stars(&client, url).await {
                            stars_map.insert(name.clone(), stars);
                        }
                    }
                }
                // Small delay between batches to avoid rate limiting
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }

            let _ = tx.send(TaskResult::StarsFetched(stars_map)).await;
        });
    }

    /// Initialize the image picker for terminal graphics protocol detection
    pub fn init_image_picker(&mut self) {
        // Try to detect terminal graphics protocol (Kitty, Sixel, iTerm2)
        match Picker::from_query_stdio() {
            Ok(picker) => {
                self.image_picker = Some(picker);
            }
            Err(_) => {
                // Fallback to halfblocks (works in any terminal)
                // Use a reasonable font size estimate for halfblocks
                let picker = Picker::from_fontsize((8, 16));
                self.image_picker = Some(picker);
            }
        }
    }

    /// Load preview image for the currently selected theme
    pub fn load_selected_preview(&mut self) {
        let Some(theme) = self.selected_theme() else {
            self.current_preview_image = None;
            self.current_preview_path = None;
            return;
        };

        let theme_name = theme.name.clone();
        let preview_url = theme.preview_url.clone();

        // Check if we have a local preview
        if let Some(preview_path) = theme.preview_path.clone() {
            // Don't reload if we already have this image loaded or loading
            if self.current_preview_path.as_ref() == Some(&preview_path) {
                return;
            }

            let Some(picker) = &self.image_picker else {
                return;
            };

            self.current_preview_path = Some(preview_path.clone());
            self.current_preview_image = None;
            self.image_loading = true;

            let tx = self.task_tx.clone();
            let mut picker = picker.clone();
            let path_for_task = preview_path.clone();
            let path_for_send = preview_path;

            // Load image in background
            tokio::spawn(async move {
                let result = tokio::task::spawn_blocking(move || {
                    load_preview_image(&mut picker, &path_for_task)
                }).await;

                let msg = match result {
                    Ok(Ok(protocol)) => Ok(protocol),
                    Ok(Err(e)) => Err(e),
                    Err(e) => Err(format!("Task error: {}", e)),
                };

                let _ = tx.send(TaskResult::ImageLoaded(path_for_send, msg)).await;
            });
        } else if let Some(url) = preview_url {
            // No local preview, but we have a URL - download it
            // Check if already cached
            let cached_path = self.cache_dir.join(format!("{}.png", theme_name));
            if cached_path.exists() {
                // Already cached, load it
                let Some(picker) = &self.image_picker else {
                    return;
                };

                self.current_preview_path = Some(cached_path.clone());
                self.current_preview_image = None;
                self.image_loading = true;

                let tx = self.task_tx.clone();
                let mut picker = picker.clone();
                let path_for_task = cached_path.clone();
                let path_for_send = cached_path;

                tokio::spawn(async move {
                    let result = tokio::task::spawn_blocking(move || {
                        load_preview_image(&mut picker, &path_for_task)
                    }).await;

                    let msg = match result {
                        Ok(Ok(protocol)) => Ok(protocol),
                        Ok(Err(e)) => Err(e),
                        Err(e) => Err(format!("Task error: {}", e)),
                    };

                    let _ = tx.send(TaskResult::ImageLoaded(path_for_send, msg)).await;
                });
            } else {
                // Not cached, need to download
                self.current_preview_image = None;
                self.current_preview_path = None;
                self.image_loading = true;

                let tx = self.task_tx.clone();
                let cache_dir = self.cache_dir.clone();

                tokio::spawn(async move {
                    let result = download_preview(&url, &cache_dir, &theme_name).await;
                    let _ = tx.send(TaskResult::PreviewDownloaded(theme_name, result)).await;
                });
            }
        } else {
            // No preview available
            self.current_preview_image = None;
            self.current_preview_path = None;
        }
    }
}

/// Fetch stars for a single repo
async fn fetch_repo_stars(client: &reqwest::Client, github_url: &str) -> Result<u32, String> {
    let url = github_url.trim_end_matches(".git");
    let parts: Vec<&str> = url.split("github.com/").collect();
    if parts.len() != 2 {
        return Err("Invalid GitHub URL".to_string());
    }
    let repo_path = parts[1];

    let api_url = format!("https://api.github.com/repos/{}", repo_path);

    let response = client
        .get(&api_url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "cruzalex-themes/0.1")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }

    #[derive(serde::Deserialize)]
    struct RepoInfo {
        stargazers_count: u32,
    }

    let info: RepoInfo = response.json().await.map_err(|e| e.to_string())?;
    Ok(info.stargazers_count)
}

/// Download preview image from URL and cache it
async fn download_preview(url: &str, cache_dir: &PathBuf, theme_name: &str) -> Result<PathBuf, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // Try main branch first, then master
    let urls = [
        url.to_string(),
        url.replace("/main/", "/master/"),
    ];

    for try_url in &urls {
        match client.get(try_url).send().await {
            Ok(response) if response.status().is_success() => {
                let bytes = response.bytes().await
                    .map_err(|e| format!("Failed to read response: {}", e))?;

                let cached_path = cache_dir.join(format!("{}.png", theme_name));
                std::fs::write(&cached_path, &bytes)
                    .map_err(|e| format!("Failed to write cache file: {}", e))?;

                return Ok(cached_path);
            }
            _ => continue,
        }
    }

    Err("Preview not found".to_string())
}

/// Load and prepare a preview image for display
fn load_preview_image(picker: &mut Picker, path: &PathBuf) -> Result<StatefulProtocol, String> {
    // Read and decode the image
    let img = ImageReader::open(path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Create protocol for the image
    let protocol = picker.new_resize_protocol(img);

    Ok(protocol)
}

/// Load favorites from file
fn load_favorites(config_dir: &PathBuf) -> HashSet<String> {
    let favorites_file = config_dir.join(".favorites");
    if let Ok(content) = std::fs::read_to_string(&favorites_file) {
        content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    } else {
        HashSet::new()
    }
}

/// Save favorites to file
fn save_favorites(config_dir: &PathBuf, favorites: &HashSet<String>) {
    let favorites_file = config_dir.join(".favorites");
    let content: String = favorites.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n");
    let _ = std::fs::write(favorites_file, content);
}
