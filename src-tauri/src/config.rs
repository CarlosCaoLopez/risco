// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

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
    pub animation: bool,
    pub sound: bool,
    pub volume: f32,
    /// Viewers the plugin has already been offered to, so one the user
    /// removed it from doesn't get it back.
    pub seen_viewers: Vec<PathBuf>,
    /// Set once the first launch has switched on autostart.
    pub first_run_done: bool,
    /// Interface language; `None` follows the system.
    pub language: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            color: "yellow".into(),
            // Ctrl+click on macOS is a right click.
            modifier: if cfg!(target_os = "macos") { Modifier::Alt } else { Modifier::Ctrl },
            roots: Vec::new(),
            animation: true,
            sound: true,
            volume: 0.6,
            seen_viewers: Vec::new(),
            first_run_done: false,
            language: None,
        }
    }
}

fn path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("risco").join("config.json"))
}

/// Where the app kept its settings before it was called Risco.
fn old_path() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join("markdown-highlighter").join("config.json"))
}

impl Config {
    pub fn load() -> Self {
        [path(), old_path()]
            .into_iter()
            .flatten()
            .find_map(|p| std::fs::read_to_string(p).ok())
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
