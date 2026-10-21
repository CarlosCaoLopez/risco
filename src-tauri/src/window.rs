// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Guess which Markdown file the focused window shows, from its title.

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum App {
    VsCode,
    Obsidian,
    Orca,
    Other,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Hint {
    /// File name or stem shown in the title, e.g. `nota.md` or `nota`.
    pub name: Option<String>,
    /// Workspace or vault folder name shown in the title.
    pub folder: Option<String>,
    /// The editor shows unsaved changes; writing the file would clash.
    pub dirty: bool,
    /// Exact file, when the app reports it (Orca).
    pub path: Option<PathBuf>,
    /// The viewer shows the rendered note; `None` when it can't be told.
    pub reading: Option<bool>,
}

pub fn active() -> Option<(App, Hint)> {
    let win = active_win_pos_rs::get_active_window().ok()?;
    let ident = format!("{} {}", win.app_name, win.process_path.display()).to_lowercase();
    let app = if ident.contains("obsidian") {
        App::Obsidian
    } else if ident.contains("orca") {
        App::Orca
    } else if ident.contains("code") || win.title.ends_with("Visual Studio Code") {
        App::VsCode
    } else {
        App::Other
    };
    let mut hint = parse_title(app, &win.title);
    if app == App::Orca {
        hint.path = crate::orca::active_file();
    }
    match app {
        App::Obsidian => hint.reading = hint.folder.as_deref().and_then(obsidian_reading),
        App::VsCode => hint.reading = vscode_reading().or(hint.reading),
        _ => {}
    }
    Some((app, hint))
}

/// Whether a gesture in the focused window should highlight. Highlights
/// only show in rendered views, so editors are left alone.
pub fn reading_view() -> bool {
    active().is_none_or(|(_, hint)| hint.reading != Some(false))
}

/// The VS Code extension records whether the focused window shows a
/// Markdown preview, since the title's `Preview` prefix is translated.
fn vscode_reading() -> Option<bool> {
    let state = std::fs::read_to_string(std::env::temp_dir().join("risco-vscode.json")).ok()?;
    serde_json::from_str::<serde_json::Value>(&state).ok()?["reading"].as_bool()
}

/// The Obsidian plugin records whether the vault's active note is in
/// reading view; the title doesn't say.
fn obsidian_reading(vault_name: &str) -> Option<bool> {
    let vault = crate::locate::obsidian_vaults()
        .into_iter()
        .find(|v| v.file_name().is_some_and(|n| n.to_string_lossy() == vault_name))?;
    let state = std::fs::read_to_string(vault.join(".obsidian/plugins/risco/state.json")).ok()?;
    serde_json::from_str::<serde_json::Value>(&state).ok()?["reading"].as_bool()
}

pub fn parse_title(app: App, title: &str) -> Hint {
    let parts: Vec<&str> = title.split(" - ").map(str::trim).collect();
    match app {
        // `● nota.md - carpeta - Visual Studio Code`, `Preview nota.md - …`
        App::VsCode => {
            let first = parts.first().copied().unwrap_or_default();
            let dirty = first.starts_with('●');
            let name = first.trim_start_matches('●').trim();
            let preview = name.strip_prefix("Preview ");
            let name = preview.unwrap_or(name);
            Hint {
                name: (!name.is_empty()).then(|| name.to_string()),
                folder: (parts.len() > 2).then(|| parts[1].to_string()),
                dirty,
                path: None,
                reading: Some(preview.is_some()),
            }
        }
        // `nota - Vault - Obsidian v1.8.9`
        App::Obsidian => Hint {
            name: (parts.len() >= 3).then(|| format!("{}.md", parts[0])),
            folder: (parts.len() >= 3).then(|| parts[parts.len() - 2].to_string()),
            dirty: false,
            path: None,
            reading: None,
        },
        // The title carries nothing; the file comes from Orca's state.
        App::Orca => Hint::default(),
        App::Other => Hint {
            name: title
                .split(|c: char| c.is_whitespace() || "—–-|·()[]".contains(c))
                .find(|w| w.ends_with(".md") || w.ends_with(".markdown"))
                .map(str::to_string)
                .or_else(|| parts.first().filter(|_| parts.len() > 1).map(|s| s.to_string())),
            folder: None,
            dirty: title.starts_with('*') || title.starts_with('●'),
            path: None,
            reading: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vscode_titles() {
        let h = parse_title(App::VsCode, "● notas.md - apuntes - Visual Studio Code");
        assert_eq!(
            h,
            Hint {
                name: Some("notas.md".into()),
                folder: Some("apuntes".into()),
                dirty: true,
                path: None,
                reading: Some(false),
            }
        );
        let h = parse_title(App::VsCode, "Preview notas.md - apuntes - Visual Studio Code");
        assert_eq!(h.name.as_deref(), Some("notas.md"));
        assert!(!h.dirty);
        assert_eq!(h.reading, Some(true));
    }

    #[test]
    fn obsidian_title() {
        let h = parse_title(App::Obsidian, "Compiladores - Universidad - Obsidian v1.8.9");
        assert_eq!(h.name.as_deref(), Some("Compiladores.md"));
        assert_eq!(h.folder.as_deref(), Some("Universidad"));
    }

    #[test]
    fn generic_title() {
        assert_eq!(
            parse_title(App::Other, "readme.md (~/x) — Orca").name.as_deref(),
            Some("readme.md")
        );
    }
}
