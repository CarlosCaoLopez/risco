//! Persistent settings, stored as JSON in the OS config dir.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Meta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub enabled: bool,
    pub color: String,
    pub modifier: Modifier,
    /// Folders searched for Markdown files, on top of the detected ones.
    pub roots: Vec<PathBuf>,
    /// Write `style="…"` into each mark, for viewers without custom CSS.
    pub inline_style: bool,
    pub animation: bool,
    pub sound: bool,
    pub volume: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            color: "yellow".into(),
            // Ctrl+click on macOS is a right click.
            modifier: if cfg!(target_os = "macos") { Modifier::Alt } else { Modifier::Ctrl },
            roots: Vec::new(),
            inline_style: false,
            animation: true,
            sound: true,
            volume: 0.6,
        }
    }
}

fn path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("markdown-highlighter").join("config.json"))
}

impl Config {
    pub fn load() -> Self {
        path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let Some(p) = path() else { return Ok(()) };
        if let Some(dir) = p.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(p, serde_json::to_string_pretty(self)?)
    }
}
