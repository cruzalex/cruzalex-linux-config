//! Theme data structures and loading

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Theme status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThemeStatus {
    /// Theme is currently active
    Active,
    /// Theme is installed locally
    Installed,
    /// Theme is available for download
    Available,
}

impl ThemeStatus {
    pub fn symbol(&self) -> &str {
        match self {
            ThemeStatus::Active => "●",
            ThemeStatus::Installed => "○",
            ThemeStatus::Available => "◌",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            ThemeStatus::Active => "Active",
            ThemeStatus::Installed => "Installed",
            ThemeStatus::Available => "Available",
        }
    }
}

/// Color palette from colors.toml
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ColorPalette {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub accent: Option<String>,
    pub cursor: Option<String>,
    pub selection_background: Option<String>,
    pub selection_foreground: Option<String>,
    pub color0: Option<String>,
    pub color1: Option<String>,
    pub color2: Option<String>,
    pub color3: Option<String>,
    pub color4: Option<String>,
    pub color5: Option<String>,
    pub color6: Option<String>,
    pub color7: Option<String>,
    pub color8: Option<String>,
    pub color9: Option<String>,
    pub color10: Option<String>,
    pub color11: Option<String>,
    pub color12: Option<String>,
    pub color13: Option<String>,
    pub color14: Option<String>,
    pub color15: Option<String>,
}

/// Theme metadata
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name (directory name)
    pub name: String,
    /// Display name (formatted)
    pub display_name: String,
    /// Theme status
    pub status: ThemeStatus,
    /// Local path (if installed)
    pub local_path: Option<PathBuf>,
    /// Remote URL (GitHub)
    pub remote_url: Option<String>,
    /// Preview image path
    pub preview_path: Option<PathBuf>,
    /// Color palette
    pub colors: Option<ColorPalette>,
    /// Is this a light theme?
    pub is_light: bool,
    /// Number of backgrounds available
    pub background_count: usize,
    /// Author/source
    pub author: Option<String>,
    /// GitHub stars (if from GitHub)
    pub stars: Option<u32>,
}

impl Theme {
    /// Create a new theme from a local directory
    pub fn from_local(path: PathBuf, current_theme: Option<&str>) -> Result<Self> {
        let name = path
            .file_name()
            .context("Invalid theme directory")?
            .to_string_lossy()
            .to_string();

        let display_name = format_theme_name(&name);

        let status = if current_theme == Some(&name) {
            ThemeStatus::Active
        } else {
            ThemeStatus::Installed
        };

        let colors_path = path.join("colors.toml");
        let colors = if colors_path.exists() {
            let content = std::fs::read_to_string(&colors_path)?;
            Some(toml::from_str(&content)?)
        } else {
            None
        };

        let preview_path = find_preview_image(&path);
        let is_light = path.join("light.mode").exists();
        let background_count = count_backgrounds(&path);

        Ok(Self {
            name,
            display_name,
            status,
            local_path: Some(path),
            remote_url: None,
            preview_path,
            colors,
            is_light,
            background_count,
            author: None,
            stars: None,
        })
    }

    /// Create a theme from remote GitHub data
    pub fn from_github(repo: &GitHubRepo) -> Self {
        let name = repo.name
            .strip_prefix("omarchy-")
            .unwrap_or(&repo.name)
            .strip_suffix("-theme")
            .unwrap_or(&repo.name)
            .to_string();

        let display_name = format_theme_name(&name);

        Self {
            name,
            display_name,
            status: ThemeStatus::Available,
            local_path: None,
            remote_url: Some(repo.clone_url.clone()),
            preview_path: None,
            colors: None,
            is_light: false,
            background_count: 0,
            author: Some(repo.owner.login.clone()),
            stars: Some(repo.stargazers_count),
        }
    }
}

/// GitHub repository data
#[derive(Debug, Deserialize)]
pub struct GitHubRepo {
    pub name: String,
    pub full_name: String,
    pub clone_url: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub owner: GitHubOwner,
}

#[derive(Debug, Deserialize)]
pub struct GitHubOwner {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubSearchResult {
    pub items: Vec<GitHubRepo>,
    pub total_count: u32,
}

/// Format theme name for display
fn format_theme_name(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Find preview image in theme directory
fn find_preview_image(path: &PathBuf) -> Option<PathBuf> {
    for name in ["preview.png", "preview.jpg", "preview.jpeg", "screenshot.png"] {
        let preview = path.join(name);
        if preview.exists() {
            return Some(preview);
        }
    }
    None
}

/// Count background images in theme
fn count_backgrounds(path: &PathBuf) -> usize {
    let bg_dir = path.join("backgrounds");
    if !bg_dir.exists() {
        return 0;
    }

    std::fs::read_dir(bg_dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| {
                            let ext = ext.to_string_lossy().to_lowercase();
                            ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "webp"
                        })
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0)
}

/// Load all local themes
pub fn load_local_themes(themes_dir: &PathBuf, current_theme: Option<&str>) -> Result<Vec<Theme>> {
    let mut themes = Vec::new();

    if !themes_dir.exists() {
        return Ok(themes);
    }

    for entry in std::fs::read_dir(themes_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip 'current' symlink and non-directories
        if !path.is_dir() || entry.file_name() == "current" {
            continue;
        }

        // Must have colors.toml to be a valid theme
        if !path.join("colors.toml").exists() {
            continue;
        }

        match Theme::from_local(path, current_theme) {
            Ok(theme) => themes.push(theme),
            Err(e) => eprintln!("Warning: Failed to load theme: {e}"),
        }
    }

    themes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(themes)
}

/// Fetch themes from GitHub
pub async fn fetch_github_themes() -> Result<Vec<Theme>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.github.com/search/repositories")
        .query(&[
            ("q", "topic:omarchy-theme"),
            ("sort", "stars"),
            ("order", "desc"),
            ("per_page", "100"),
        ])
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "cruzalex-themes/0.1")
        .send()
        .await?;

    let result: GitHubSearchResult = response.json().await?;

    Ok(result.items.into_iter().map(|repo| Theme::from_github(&repo)).collect())
}
