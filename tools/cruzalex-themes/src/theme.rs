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
    /// Preview image path (local file)
    pub preview_path: Option<PathBuf>,
    /// Preview image URL (for remote themes)
    pub preview_url: Option<String>,
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
            // Try to parse, but don't fail the whole theme if TOML is invalid
            // Some themes have malformed colors.toml (e.g., duplicate keys)
            toml::from_str(&content).ok()
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
            preview_url: None,
            colors,
            is_light,
            background_count,
            author: None,
            stars: None,
        })
    }

    /// Create a theme from remote data (name and URL)
    pub fn from_remote(name: &str, url: &str, author: Option<&str>) -> Self {
        let display_name = format_theme_name(name);
        let preview_url = github_clone_url_to_preview_url(url);

        Self {
            name: name.to_string(),
            display_name,
            status: ThemeStatus::Available,
            local_path: None,
            remote_url: Some(url.to_string()),
            preview_path: None,
            preview_url,
            colors: None,
            is_light: name.contains("light"),
            background_count: 0,
            author: author.map(|s| s.to_string()),
            stars: None,
        }
    }

    /// Create a theme from GitHub API data (fallback)
    pub fn from_github(repo: &GitHubRepo) -> Self {
        let name = repo.name
            .strip_prefix("omarchy-")
            .unwrap_or(&repo.name)
            .strip_suffix("-theme")
            .unwrap_or(&repo.name)
            .to_string();

        let display_name = format_theme_name(&name);
        let preview_url = github_clone_url_to_preview_url(&repo.clone_url);

        Self {
            name,
            display_name,
            status: ThemeStatus::Available,
            local_path: None,
            remote_url: Some(repo.clone_url.clone()),
            preview_path: None,
            preview_url,
            colors: None,
            is_light: false,
            background_count: 0,
            author: Some(repo.owner.login.clone()),
            stars: Some(repo.stargazers_count),
        }
    }
}

/// Convert GitHub clone URL to raw preview image URL
/// e.g., https://github.com/user/repo.git -> https://raw.githubusercontent.com/user/repo/main/preview.png
fn github_clone_url_to_preview_url(clone_url: &str) -> Option<String> {
    // Handle both https://github.com/user/repo and https://github.com/user/repo.git
    let url = clone_url.trim_end_matches(".git");

    if url.contains("github.com") {
        // Extract owner and repo from URL
        // https://github.com/owner/repo -> owner/repo
        let parts: Vec<&str> = url.split("github.com/").collect();
        if parts.len() == 2 {
            let path = parts[1];
            // Try main branch first, then master
            return Some(format!(
                "https://raw.githubusercontent.com/{}/main/preview.png",
                path
            ));
        }
    }
    None
}

/// GitHub repository data (for fallback API search)
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

        // Accept themes with colors.toml OR any app config files
        // Some Omarchy themes use per-app configs instead of centralized colors.toml
        let has_colors = path.join("colors.toml").exists();
        let has_ghostty = path.join("ghostty.conf").exists();
        let has_kitty = path.join("kitty.conf").exists();
        let has_alacritty = path.join("alacritty.toml").exists();
        let has_hyprland = path.join("hyprland.conf").exists();

        if !has_colors && !has_ghostty && !has_kitty && !has_alacritty && !has_hyprland {
            continue;
        }

        match Theme::from_local(path, current_theme) {
            Ok(theme) => themes.push(theme),
            Err(_) => {
                // Silently skip themes with invalid configs (e.g., TOML parse errors)
                // Don't use eprintln as it corrupts the TUI
                continue;
            }
        }
    }

    themes.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(themes)
}

/// Hardcoded list of Omarchy themes from awesome-omarchy
/// Source: https://github.com/aorumbayev/awesome-omarchy
pub fn get_awesome_omarchy_themes() -> Vec<Theme> {
    let themes_data = vec![
        ("aetheria", "https://github.com/JJDizz1L/aetheria", "JJDizz1L"),
        ("aamis", "https://github.com/vyrx-dev/omarchy-aamis-theme", "vyrx-dev"),
        ("alabaster", "https://github.com/grierson/omarchy-alabaster-theme", "grierson"),
        ("akane", "https://github.com/Grenish/omarchy-akane-theme", "Grenish"),
        ("agentuity", "https://github.com/rblalock/omarchy-agentuity.theme", "rblalock"),
        ("all-hallows-eve", "https://github.com/guilhermetk/omarchy-all-hallows-eve-theme", "guilhermetk"),
        ("amberbyte", "https://github.com/tahfizhabib/omarchy-amberbyte-theme", "tahfizhabib"),
        ("anonymous", "https://github.com/j4v3l/omarchy-anonymous-theme", "j4v3l"),
        ("arc-blueberry", "https://github.com/vale-c/omarchy-arc-blueberry", "vale-c"),
        ("archriot", "https://github.com/CyphrRiot/omarchy-archriot-theme", "CyphrRiot"),
        ("ash", "https://github.com/bjarneo/omarchy-ash-theme", "bjarneo"),
        ("aura", "https://github.com/bjarneo/omarchy-aura-theme", "bjarneo"),
        ("ayaka", "https://github.com/abhijeet-swami/omarchy-ayaka-theme", "abhijeet-swami"),
        ("ayu-dark", "https://github.com/fdidron/omarchy-ayu-dark-theme", "fdidron"),
        ("ayu-light", "https://github.com/fdidron/omarchy-ayu-light-theme", "fdidron"),
        ("ayu-mirage", "https://github.com/fdidron/omarchy-ayumirage", "fdidron"),
        ("azure-glow", "https://github.com/Hydradevx/omarchy-azure-glow-theme", "Hydradevx"),
        ("batou", "https://github.com/HANCORE-linux/omarchy-batou-theme", "HANCORE-linux"),
        ("bauhaus", "https://github.com/somerocketeer/omarchy-bauhaus-theme", "somerocketeer"),
        ("blackgold", "https://github.com/HANCORE-linux/omarchy-blackgold-theme", "HANCORE-linux"),
        ("blackmoney", "https://github.com/HANCORE-linux/omarchy-blackmoney-theme", "HANCORE-linux"),
        ("blackturq", "https://github.com/HANCORE-linux/omarchy-blackturq-theme", "HANCORE-linux"),
        ("blueridge-dark", "https://github.com/hipsterusername/omarchy-blueridge-dark-theme", "hipsterusername"),
        ("c64", "https://github.com/scar45/omarchy-c64-theme", "scar45"),
        ("catppuccin-mocha", "https://github.com/KidDogDad/omarchy-catppuccin-mocha-theme", "KidDogDad"),
        ("cobalt2", "https://github.com/hoblin/omarchy-cobalt2-theme", "hoblin"),
        ("crimson-gold", "https://github.com/knappkevin/omarchy-crimson-gold-theme", "knappkevin"),
        ("delorean", "https://github.com/jbnunn/omarchy-delorean-theme", "jbnunn"),
        ("doom", "https://github.com/AX200M/omarchy-doom-theme", "AX200M"),
        ("dracula", "https://github.com/catlee/omarchy-dracula-theme", "catlee"),
        ("dracula-official", "https://github.com/dracula/omarchy", "dracula"),
        ("elysian", "https://github.com/bjarneo/omarchy-elysian-theme", "bjarneo"),
        ("ember-n-ash", "https://github.com/Hydradevx/omarchy-ember-n-ash-theme", "Hydradevx"),
        ("everblush", "https://github.com/Swarnim114/omarchy-everblush-theme", "Swarnim114"),
        ("evergarden", "https://github.com/celsobenedetti/omarchy-evergarden", "celsobenedetti"),
        ("f1", "https://github.com/999Gabriel/F1-omarchy", "999Gabriel"),
        ("felix", "https://github.com/TyRichards/omarchy-felix-theme", "TyRichards"),
        ("fiery-ocean", "https://github.com/bjarneo/omarchy-fiery-ocean-theme", "bjarneo"),
        ("fireside", "https://github.com/bjarneo/omarchy-fireside-theme", "bjarneo"),
        ("firesky", "https://github.com/bjarneo/omarchy-firesky-theme", "bjarneo"),
        ("flexoki-dark", "https://github.com/euandeas/omarchy-flexoki-dark-theme", "euandeas"),
        ("flexoki-light", "https://github.com/euandeas/omarchy-flexoki-light-theme", "euandeas"),
        ("forest-green", "https://github.com/abhijeet-swami/omarchy-forest-green-theme", "abhijeet-swami"),
        ("frost", "https://github.com/bjarneo/omarchy-frost-theme", "bjarneo"),
        ("futurism", "https://github.com/bjarneo/omarchy-futurism-theme", "bjarneo"),
        ("github-light", "https://github.com/ryanyogan/omarchy-github-light-theme", "ryanyogan"),
        ("gold-rush", "https://github.com/tahayvr/omarchy-gold-rush-theme", "tahayvr"),
        ("green-garden", "https://github.com/kalk-ak/omarchy-green-garden-theme", "kalk-ak"),
        ("gtk", "https://github.com/bjarneo/omarchy-gtk-theme", "bjarneo"),
        ("harbor", "https://github.com/HANCORE-linux/omarchy-harbor-theme", "HANCORE-linux"),
        ("harbordark", "https://github.com/HANCORE-linux/omarchy-harbordark-theme", "HANCORE-linux"),
        ("hollow-knight", "https://github.com/bjarneo/omarchy-hollow-knight-theme", "bjarneo"),
        ("inkypinky", "https://github.com/HANCORE-linux/omarchy-inkypinky-theme", "HANCORE-linux"),
        ("kanagawa-dragon", "https://github.com/MrTrigger/omarchy-kanagawa-dragon-theme", "MrTrigger"),
        ("kimiko", "https://github.com/krymzonn/omarchy-kimiko-theme", "krymzonn"),
        ("mars", "https://github.com/steve-lohmeyer/omarchy-mars-theme", "steve-lohmeyer"),
        ("matte-black", "https://github.com/tahayvr/omarchy-matte-black", "tahayvr"),
        ("mechanoonna", "https://github.com/HANCORE-linux/omarchy-mechanoonna-theme", "HANCORE-linux"),
        ("midnight", "https://github.com/JaxonWright/omarchy-midnight-theme", "JaxonWright"),
        ("milkmatcha-light", "https://github.com/hipsterusername/omarchy-milkmatcha-light-theme", "hipsterusername"),
        ("monochrome", "https://github.com/Swarnim114/omarchy-monochrome-theme", "Swarnim114"),
        ("monokai", "https://github.com/bjarneo/omarchy-monokai-theme", "bjarneo"),
        ("monokai-dark", "https://github.com/ericrswanny/omarchy-monokai-dark-theme", "ericrswanny"),
        ("moodpeak", "https://github.com/HANCORE-linux/omarchy-moodpeak-theme", "HANCORE-linux"),
        ("motivator", "https://github.com/rondilley/omarchy-motivator-theme", "rondilley"),
        ("nagai-twilight", "https://github.com/somerocketeer/omarchy-nagai-twilight-theme", "somerocketeer"),
        ("nes", "https://github.com/bjarneo/omarchy-nes-theme", "bjarneo"),
        ("night-owl", "https://github.com/maxberggren/omarchy-night-owl-theme", "maxberggren"),
        ("obsidian", "https://github.com/Hydradevx/omarchy-obsidian-theme", "Hydradevx"),
        ("one-dark-pro", "https://github.com/sc0ttman/omarchy-one-dark-pro", "sc0ttman"),
        ("osaka-jade", "https://github.com/Justikun/omarchy-osaka-jade-theme", "Justikun"),
        ("pina", "https://github.com/bjarneo/omarchy-pina-theme", "bjarneo"),
        ("pink-blood", "https://github.com/ITSZXY/pink-blood-omarchy-theme", "ITSZXY"),
        ("pulsar", "https://github.com/bjarneo/omarchy-pulsar-theme", "bjarneo"),
        ("pure-latin", "https://github.com/daurydicaprio/omarchy-pure-latin-theme", "daurydicaprio"),
        ("purplewave", "https://github.com/dotsilva/omarchy-purplewave-theme", "dotsilva"),
        ("retro-fallout", "https://github.com/zdravkodanailov7/omarchy-retro-fallout-theme", "zdravkodanailov7"),
        ("retropc", "https://github.com/rondilley/omarchy-retropc-theme", "rondilley"),
        ("reverie", "https://github.com/bjarneo/omarchy-reverie-theme", "bjarneo"),
        ("rose-pine", "https://github.com/guilhermetk/omarchy-rose-pine", "guilhermetk"),
        ("rose-pine-dark", "https://github.com/guilhermetk/omarchy-rose-pine-dark", "guilhermetk"),
        ("rose-pine-dawn", "https://github.com/ryanyogan/omarchy-rose-pine-dawn-theme", "ryanyogan"),
        ("sakura", "https://github.com/bjarneo/omarchy-sakura-theme", "bjarneo"),
        ("sapphire", "https://github.com/HANCORE-linux/omarchy-sapphire-theme", "HANCORE-linux"),
        ("serenity", "https://github.com/bjarneo/omarchy-serenity-theme", "bjarneo"),
        ("shadesofjade", "https://github.com/HANCORE-linux/omarchy-shadesofjade-theme", "HANCORE-linux"),
        ("snow", "https://github.com/bjarneo/omarchy-snow-theme", "bjarneo"),
        ("solarized", "https://github.com/Gazler/omarchy-solarized-theme", "Gazler"),
        ("solarized-light", "https://github.com/dfrico/omarchy-solarized-light-theme", "dfrico"),
        ("solarized-osaka", "https://github.com/motorsss/omarchy-solarizedosaka-theme", "motorsss"),
        ("space-monkey", "https://github.com/TyRichards/omarchy-space-monkey-theme", "TyRichards"),
        ("spectra", "https://github.com/abhijeet-swami/omarchy-spectra-theme", "abhijeet-swami"),
        ("sunset-drive", "https://github.com/tahayvr/omarchy-sunset-drive-theme", "tahayvr"),
        ("synthwave84", "https://github.com/omacom-io/omarchy-synthwave84-theme", "omacom-io"),
        ("thegreek", "https://github.com/HANCORE-linux/omarchy-thegreek-theme", "HANCORE-linux"),
        ("tycho", "https://github.com/leonardobetti/omarchy-tycho", "leonardobetti"),
        ("velocity", "https://github.com/perfektnacht/omarchy-velocity-theme", "perfektnacht"),
        ("velvetnight", "https://github.com/HANCORE-linux/omarchy-velvetnight-theme", "HANCORE-linux"),
        ("vesper", "https://github.com/thmoee/omarchy-vesper-theme", "thmoee"),
        ("vhs80", "https://github.com/tahayvr/omarchy-vhs80-theme", "tahayvr"),
        ("void", "https://github.com/vyrx-dev/omarchy-void-theme", "vyrx-dev"),
        ("wasteland", "https://github.com/perfektnacht/omarchy-wasteland-theme", "perfektnacht"),
        ("waveform-dark", "https://github.com/hipsterusername/omarchy-waveform-dark-theme", "hipsterusername"),
        ("whitegold", "https://github.com/HANCORE-linux/omarchy-whitegold-theme", "HANCORE-linux"),
        ("wood", "https://github.com/bjarneo/omarchy-wood-theme", "bjarneo"),
    ];

    themes_data
        .into_iter()
        .map(|(name, url, author)| Theme::from_remote(name, url, Some(author)))
        .collect()
}

/// Fetch themes - uses hardcoded list from awesome-omarchy
pub async fn fetch_github_themes() -> Result<Vec<Theme>> {
    // Return the curated list from awesome-omarchy
    // This is more reliable than GitHub API search which may miss themes
    Ok(get_awesome_omarchy_themes())
}

/// Fetch themes from GitHub API (fallback/additional)
pub async fn fetch_github_api_themes() -> Result<Vec<Theme>> {
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

/// Fetch GitHub stars for a specific theme repo
/// URL should be in format: https://github.com/owner/repo
pub async fn fetch_theme_stars(github_url: &str) -> Result<u32> {
    // Extract owner/repo from URL
    let url = github_url.trim_end_matches(".git");
    let parts: Vec<&str> = url.split("github.com/").collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid GitHub URL");
    }
    let repo_path = parts[1];

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let api_url = format!("https://api.github.com/repos/{}", repo_path);

    let response = client
        .get(&api_url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "cruzalex-themes/0.1")
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("GitHub API error: {}", response.status());
    }

    #[derive(Deserialize)]
    struct RepoInfo {
        stargazers_count: u32,
    }

    let info: RepoInfo = response.json().await?;
    Ok(info.stargazers_count)
}
