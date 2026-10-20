//! Install the liquid-glass stylesheet into the viewers that can load one.

use serde::Serialize;
use std::path::{Path, PathBuf};

pub const CSS: &str = include_str!("../../assets/liquid-highlight.css");
const FILE_NAME: &str = "liquid-highlight.css";
const SNIPPET: &str = "liquid-highlight";

#[derive(Debug, Clone, Serialize)]
pub struct Viewer {
    pub kind: &'static str,
    pub name: String,
    pub path: PathBuf,
    pub installed: bool,
}

pub fn viewers() -> Vec<Viewer> {
    let vaults = crate::locate::obsidian_vaults();
    let mut out: Vec<Viewer> = vaults
        .iter()
        .map(|p| Viewer {
            kind: "obsidian",
            name: folder_name(p),
            path: p.clone(),
            installed: obsidian_installed(p),
        })
        .collect();
    out.extend(
        crate::locate::vscode_folders()
            .into_iter()
            .filter(|p| !vaults.iter().any(|v| p.starts_with(v)))
            .map(|p| Viewer {
                kind: "vscode",
                name: folder_name(&p),
                installed: vscode_installed(&p),
                path: p,
            }),
    );
    out
}

fn folder_name(p: &Path) -> String {
    p.file_name().map_or_else(|| p.display().to_string(), |n| n.to_string_lossy().into_owned())
}

pub fn install(kind: &str, folder: &Path) -> Result<(), String> {
    match kind {
        "obsidian" => install_obsidian(folder),
        "vscode" => install_vscode(folder),
        _ => return Err(format!("visor desconocido: {kind}")),
    }
    .map_err(|e| e.to_string())
}

fn obsidian_installed(vault: &Path) -> bool {
    let appearance = read_object(&vault.join(".obsidian/appearance.json"));
    vault.join(".obsidian/snippets").join(FILE_NAME).is_file()
        && appearance["enabledCssSnippets"]
            .as_array()
            .is_some_and(|a| a.iter().any(|s| s == SNIPPET))
}

/// Copies the snippet and switches it on in `appearance.json`.
fn install_obsidian(vault: &Path) -> std::io::Result<()> {
    let dir = vault.join(".obsidian/snippets");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(FILE_NAME), CSS)?;

    let file = vault.join(".obsidian/appearance.json");
    let mut json = read_object(&file);
    let list = json
        .as_object_mut()
        .expect("object")
        .entry("enabledCssSnippets")
        .or_insert_with(|| serde_json::json!([]));
    if !list.is_array() {
        *list = serde_json::json!([]);
    }
    let arr = list.as_array_mut().expect("array");
    if !arr.iter().any(|s| s == SNIPPET) {
        arr.push(SNIPPET.into());
    }
    std::fs::write(file, serde_json::to_string_pretty(&json)?)
}

fn vscode_installed(folder: &Path) -> bool {
    folder.join(".vscode").join(FILE_NAME).is_file()
}

/// VS Code's preview only loads styles from inside the workspace, so the
/// sheet goes to `.vscode/` and is listed in the workspace `markdown.styles`.
fn install_vscode(folder: &Path) -> std::io::Result<()> {
    let dir = folder.join(".vscode");
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(FILE_NAME), CSS)?;

    let file = dir.join("settings.json");
    let entry = format!(".vscode/{FILE_NAME}");
    let text = std::fs::read_to_string(&file).unwrap_or_default();
    if text.contains(&entry) {
        return Ok(());
    }
    let mut json: serde_json::Value = if text.trim().is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_str(&text).map_err(|_| {
            std::io::Error::other(format!(
                "{} tiene comentarios o no es JSON válido; añade \"{entry}\" a \"markdown.styles\" a mano",
                file.display()
            ))
        })?
    };
    let styles = json
        .as_object_mut()
        .ok_or_else(|| std::io::Error::other("settings.json no es un objeto"))?
        .entry("markdown.styles")
        .or_insert_with(|| serde_json::json!([]));
    if let Some(arr) = styles.as_array_mut() {
        arr.push(entry.into());
    }
    std::fs::write(file, serde_json::to_string_pretty(&json)?)
}

fn read_object(path: &Path) -> serde_json::Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .filter(serde_json::Value::is_object)
        .unwrap_or_else(|| serde_json::json!({}))
}

/// Self-contained `style` attribute for viewers that ignore custom CSS.
pub fn inline_css(color: &str) -> String {
    let rgb = match color {
        "pink" => "255,64,129",
        "green" => "52,211,105",
        "blue" => "64,156,255",
        "purple" => "175,102,255",
        _ => "255,232,102",
    };
    format!(
        "background:linear-gradient(180deg,rgba(255,255,255,.5),rgba(255,255,255,0) 58%),\
         rgba({rgb},.38);border-radius:.32em .55em .38em .5em;padding:.1em .24em;\
         box-shadow:inset 0 .75px 0 rgba(255,255,255,.85),0 0 0 .5px rgba(0,0,0,.1),\
         0 2px 7px rgba({rgb},.26);-webkit-box-decoration-break:clone;\
         box-decoration-break:clone;color:inherit"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_into_vault_and_workspace() {
        let dir = std::env::temp_dir().join(format!("mh-styles-{}", std::process::id()));
        std::fs::create_dir_all(dir.join(".obsidian")).unwrap();
        std::fs::write(dir.join(".obsidian/appearance.json"), r#"{"enabledCssSnippets":["otro"]}"#).unwrap();
        install_obsidian(&dir).unwrap();
        install_obsidian(&dir).unwrap();
        assert!(obsidian_installed(&dir));
        let json = read_object(&dir.join(".obsidian/appearance.json"));
        assert_eq!(json["enabledCssSnippets"], serde_json::json!(["otro", SNIPPET]));

        install_vscode(&dir).unwrap();
        install_vscode(&dir).unwrap();
        let settings = read_object(&dir.join(".vscode/settings.json"));
        assert_eq!(settings["markdown.styles"], serde_json::json!([".vscode/liquid-highlight.css"]));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
