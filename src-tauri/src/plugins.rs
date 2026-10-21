// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Install the viewer plugins that paint the sidecar highlights: an Obsidian
//! plugin per vault and one VS Code extension per user. Both are built into
//! `plugins/dist/` by `npm run build:plugins` and embedded here.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

const PLUGIN_ID: &str = "risco";
/// Plugin and extension id from before the app was called Risco.
const OLD_ID: &str = "subrayador";
const OBSIDIAN_FILES: [(&str, &str); 3] = [
    ("main.js", include_str!("../../plugins/dist/obsidian/main.js")),
    ("manifest.json", include_str!("../../plugins/dist/obsidian/manifest.json")),
    ("styles.css", include_str!("../../plugins/dist/obsidian/styles.css")),
];
const VSIX: &[u8] = include_bytes!("../../plugins/dist/risco.vsix");
/// `publisher.name` from `plugins/vscode/package.json`.
const EXTENSION_ID: &str = "risco.risco";
/// The snippet earlier versions installed instead of the plugin.
const OLD_SNIPPET: &str = "liquid-highlight";

#[derive(Debug, Clone, Serialize)]
pub struct Viewer {
    pub kind: &'static str,
    pub name: String,
    pub path: PathBuf,
    /// This app's version of the plugin is in place.
    pub installed: bool,
    /// Some version of the plugin is there, maybe an older one.
    pub present: bool,
}

pub fn viewers() -> Vec<Viewer> {
    let mut out: Vec<Viewer> = crate::locate::obsidian_vaults()
        .into_iter()
        .map(|p| Viewer {
            kind: "obsidian",
            name: folder_name(&p),
            installed: obsidian_installed(&p),
            present: obsidian_present(&p),
            path: p,
        })
        .collect();
    if let Some(dir) = vscode_extensions_dir().filter(|d| d.is_dir()) {
        out.push(Viewer {
            kind: "vscode",
            name: crate::i18n::t("plugins.vscode_name"),
            installed: vscode_installed(&dir),
            present: vscode_present(&dir),
            path: dir,
        });
    }
    out
}

/// Installs the plugin in every viewer that has never been seen, and
/// updates it wherever an older version is in place. A viewer the user
/// removed the plugin from is left alone.
pub fn sync(seen: &[PathBuf]) -> Vec<(Viewer, Result<(), String>)> {
    viewers()
        .into_iter()
        .filter(|v| !v.installed && (v.present || !seen.contains(&v.path)))
        .map(|v| {
            let result = install(v.kind, &v.path);
            (v, result)
        })
        .collect()
}

/// The version the build stamped into the embedded plugins.
fn version() -> String {
    let manifest: serde_json::Value = serde_json::from_str(OBSIDIAN_FILES[1].1).unwrap_or_default();
    manifest["version"].as_str().unwrap_or_default().to_string()
}

fn folder_name(p: &Path) -> String {
    p.file_name().map_or_else(|| p.display().to_string(), |n| n.to_string_lossy().into_owned())
}

pub fn install(kind: &str, path: &Path) -> Result<(), String> {
    match kind {
        "obsidian" => install_obsidian(path).map_err(|e| e.to_string()),
        "vscode" => install_vscode(),
        _ => Err(crate::i18n::tf("plugins.unknown", &[("kind", kind)])),
    }
}

fn obsidian_present(vault: &Path) -> bool {
    vault.join(".obsidian/plugins").join(PLUGIN_ID).join("main.js").is_file()
        || vault.join(".obsidian/plugins").join(OLD_ID).is_dir()
}

fn obsidian_installed(vault: &Path) -> bool {
    let dir = vault.join(".obsidian/plugins").join(PLUGIN_ID);
    let enabled = read_json(&vault.join(".obsidian/community-plugins.json"));
    dir.join("main.js").is_file()
        && read_json(&dir.join("manifest.json"))["version"] == version().as_str()
        && enabled.as_array().is_some_and(|a| a.iter().any(|s| s == PLUGIN_ID))
}

/// Copies the plugin, switches it on, and drops the old CSS snippet and the
/// plugin's pre-rename copy.
fn install_obsidian(vault: &Path) -> std::io::Result<()> {
    let dir = vault.join(".obsidian/plugins").join(PLUGIN_ID);
    std::fs::create_dir_all(&dir)?;
    for (name, contents) in OBSIDIAN_FILES {
        std::fs::write(dir.join(name), contents)?;
    }

    let file = vault.join(".obsidian/community-plugins.json");
    let mut enabled = read_json(&file);
    if !enabled.is_array() {
        enabled = serde_json::json!([]);
    }
    let arr = enabled.as_array_mut().expect("array");
    arr.retain(|s| s != OLD_ID);
    if !arr.iter().any(|s| s == PLUGIN_ID) {
        arr.push(PLUGIN_ID.into());
    }
    std::fs::write(file, serde_json::to_string_pretty(&enabled)?)?;

    let appearance = vault.join(".obsidian/appearance.json");
    let mut json = read_json(&appearance);
    if let Some(list) = json.get_mut("enabledCssSnippets").and_then(|l| l.as_array_mut()) {
        let before = list.len();
        list.retain(|s| s != OLD_SNIPPET);
        if list.len() != before {
            std::fs::write(appearance, serde_json::to_string_pretty(&json)?)?;
        }
    }
    let _ = std::fs::remove_file(vault.join(".obsidian/snippets").join(format!("{OLD_SNIPPET}.css")));
    let _ = std::fs::remove_dir_all(vault.join(".obsidian/plugins").join(OLD_ID));
    Ok(())
}

fn vscode_extensions_dir() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".vscode/extensions"))
}

fn vscode_extensions(dir: &Path) -> Vec<String> {
    std::fs::read_dir(dir)
        .map(|it| it.filter_map(Result::ok).map(|e| e.file_name().to_string_lossy().into_owned()).collect())
        .unwrap_or_default()
}

fn vscode_present(dir: &Path) -> bool {
    let prefixes = [format!("{EXTENSION_ID}-"), format!("{OLD_ID}.{OLD_ID}-")];
    vscode_extensions(dir).iter().any(|e| prefixes.iter().any(|p| e.starts_with(p)))
}

fn vscode_installed(dir: &Path) -> bool {
    let current = format!("{EXTENSION_ID}-{}", version());
    vscode_extensions(dir).contains(&current)
}

/// Hands the packaged extension to VS Code's own CLI, which works while it runs.
fn install_vscode() -> Result<(), String> {
    let vsix = std::env::temp_dir().join(format!("risco-{}.vsix", std::process::id()));
    std::fs::write(&vsix, VSIX).map_err(|e| e.to_string())?;
    let result = ["code", "code-insiders", "codium"].iter().find_map(|cli| {
        let out = Command::new(cli).arg("--install-extension").arg(&vsix).arg("--force").output().ok()?;
        let old = format!("{OLD_ID}.{OLD_ID}");
        let _ = Command::new(cli).arg("--uninstall-extension").arg(old).output();
        Some(out)
    });
    let _ = std::fs::remove_file(&vsix);
    match result {
        Some(out) if out.status.success() => Ok(()),
        Some(out) => Err(crate::i18n::tf("plugins.vscode_failed", &[("error", String::from_utf8_lossy(&out.stderr).trim())])),
        None => Err(crate::i18n::t("plugins.no_code")),
    }
}

fn read_json(path: &Path) -> serde_json::Value {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .unwrap_or(serde_json::Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_into_vault() {
        let dir = std::env::temp_dir().join(format!("mh-plugins-{}", std::process::id()));
        std::fs::create_dir_all(dir.join(".obsidian/snippets")).unwrap();
        std::fs::write(dir.join(".obsidian/community-plugins.json"), r#"["otro","subrayador"]"#).unwrap();
        std::fs::create_dir_all(dir.join(".obsidian/plugins/subrayador")).unwrap();
        std::fs::write(dir.join(".obsidian/appearance.json"), r#"{"enabledCssSnippets":["x","liquid-highlight"]}"#)
            .unwrap();
        std::fs::write(dir.join(".obsidian/snippets/liquid-highlight.css"), "").unwrap();
        install_obsidian(&dir).unwrap();
        install_obsidian(&dir).unwrap();
        assert!(obsidian_installed(&dir));
        assert_eq!(read_json(&dir.join(".obsidian/community-plugins.json")), serde_json::json!(["otro", PLUGIN_ID]));
        assert_eq!(read_json(&dir.join(".obsidian/appearance.json"))["enabledCssSnippets"], serde_json::json!(["x"]));
        assert!(!dir.join(".obsidian/snippets/liquid-highlight.css").exists());
        assert!(!dir.join(".obsidian/plugins/subrayador").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn sync_policy() {
        let dir = std::env::temp_dir().join(format!("mh-sync-{}", std::process::id()));
        std::fs::create_dir_all(dir.join(".obsidian/plugins/subrayador")).unwrap();
        assert!(obsidian_present(&dir) && !obsidian_installed(&dir));
        install_obsidian(&dir).unwrap();
        assert!(obsidian_installed(&dir));

        // An older copy counts as present but not installed.
        let manifest = dir.join(".obsidian/plugins/risco/manifest.json");
        std::fs::write(&manifest, r#"{"version":"0.0.1"}"#).unwrap();
        assert!(obsidian_present(&dir) && !obsidian_installed(&dir));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn embedded_plugins_carry_a_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn embedded_extension_id_matches_manifest() {
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../plugins/vscode/package.json")).unwrap();
        assert_eq!(format!("{}.{}", manifest["publisher"].as_str().unwrap(), manifest["name"].as_str().unwrap()), EXTENSION_ID);
    }
}
