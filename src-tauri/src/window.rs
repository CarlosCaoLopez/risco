//! Guess which Markdown file the focused window shows, from its title.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum App {
    VsCode,
    Obsidian,
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
}

pub fn active() -> Option<(App, Hint)> {
    let win = active_win_pos_rs::get_active_window().ok()?;
    let ident = format!("{} {}", win.app_name, win.process_path.display()).to_lowercase();
    let app = if ident.contains("obsidian") {
        App::Obsidian
    } else if ident.contains("code") || win.title.ends_with("Visual Studio Code") {
        App::VsCode
    } else {
        App::Other
    };
    Some((app, parse_title(app, &win.title)))
}

pub fn parse_title(app: App, title: &str) -> Hint {
    let parts: Vec<&str> = title.split(" - ").map(str::trim).collect();
    match app {
        // `● nota.md - carpeta - Visual Studio Code`, `Preview nota.md - …`
        App::VsCode => {
            let first = parts.first().copied().unwrap_or_default();
            let dirty = first.starts_with('●');
            let name = first.trim_start_matches('●').trim();
            let name = name.strip_prefix("Preview ").unwrap_or(name);
            Hint {
                name: (!name.is_empty()).then(|| name.to_string()),
                folder: (parts.len() > 2).then(|| parts[1].to_string()),
                dirty,
            }
        }
        // `nota - Vault - Obsidian v1.8.9`
        App::Obsidian => Hint {
            name: (parts.len() >= 3).then(|| format!("{}.md", parts[0])),
            folder: (parts.len() >= 3).then(|| parts[parts.len() - 2].to_string()),
            dirty: false,
        },
        App::Other => Hint {
            name: title
                .split(|c: char| c.is_whitespace() || "—–-|·()[]".contains(c))
                .find(|w| w.ends_with(".md") || w.ends_with(".markdown"))
                .map(str::to_string)
                .or_else(|| parts.first().filter(|_| parts.len() > 1).map(|s| s.to_string())),
            folder: None,
            dirty: title.starts_with('*') || title.starts_with('●'),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vscode_titles() {
        let h = parse_title(App::VsCode, "● notas.md - apuntes - Visual Studio Code");
        assert_eq!(h, Hint { name: Some("notas.md".into()), folder: Some("apuntes".into()), dirty: true });
        let h = parse_title(App::VsCode, "Preview notas.md - apuntes - Visual Studio Code");
        assert_eq!(h.name.as_deref(), Some("notas.md"));
        assert!(!h.dirty);
    }

    #[test]
    fn obsidian_title() {
        let h = parse_title(App::Obsidian, "Compiladores - Universidad - Obsidian v1.8.9");
        assert_eq!(h.name.as_deref(), Some("Compiladores.md"));
        assert_eq!(h.folder.as_deref(), Some("Universidad"));
    }

    #[test]
    fn generic_title() {
        assert_eq!(parse_title(App::Other, "readme.md (~/x) — Orca").name.as_deref(), Some("readme.md"));
    }
}
