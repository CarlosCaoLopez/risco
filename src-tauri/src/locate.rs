// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Find the Markdown file a selection came from.

use crate::window::Hint;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

const SKIP_DIRS: [&str; 6] = [".git", ".obsidian", ".trash", "node_modules", "target", ".venv"];
const MAX_FILES: usize = 20_000;

/// Obsidian vaults listed in `obsidian.json`.
pub fn obsidian_vaults() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> =
        dirs::config_dir().into_iter().map(|d| d.join("obsidian")).collect();
    if let Some(home) = dirs::home_dir() {
        // Snap and Flatpak installs keep their config elsewhere.
        files.push(home.join("snap/obsidian/current/.config/obsidian"));
        files.push(home.join(".var/app/md.obsidian.Obsidian/config/obsidian"));
    }
    let mut out: Vec<PathBuf> = files
        .iter()
        .filter_map(|d| read_json(&d.join("obsidian.json")))
        .flat_map(|json| {
            json["vaults"]
                .as_object()
                .into_iter()
                .flat_map(|v| v.values())
                .filter_map(|v| v["path"].as_str().map(PathBuf::from))
                .collect::<Vec<_>>()
        })
        .filter(|p| p.is_dir())
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Folders VS Code has open or opened recently, from its global storage.
pub fn vscode_folders() -> Vec<PathBuf> {
    let Some(config) = dirs::config_dir() else { return Vec::new() };
    let mut out = Vec::new();
    for product in ["Code", "Code - OSS", "VSCodium", "Cursor"] {
        let file = config.join(product).join("User/globalStorage/storage.json");
        if let Some(json) = read_json(&file) {
            collect_file_uris(&json, &mut out);
        }
    }
    out.sort();
    out.dedup();
    out.retain(|p| p.is_dir());
    out
}

fn collect_file_uris(v: &serde_json::Value, out: &mut Vec<PathBuf>) {
    match v {
        serde_json::Value::String(s) => {
            if let Some(path) = file_uri_to_path(s) {
                out.push(path);
            }
        }
        serde_json::Value::Array(a) => a.iter().for_each(|x| collect_file_uris(x, out)),
        serde_json::Value::Object(o) => o.values().for_each(|x| collect_file_uris(x, out)),
        _ => {}
    }
}

fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let decoded = percent_decode(rest);
    // `file:///c%3A/Users/…` on Windows.
    let decoded = match decoded.as_bytes() {
        [b'/', d, b':', ..] if d.is_ascii_alphabetic() => decoded[1..].to_string(),
        _ => decoded,
    };
    Some(PathBuf::from(decoded))
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn read_json(path: &Path) -> Option<serde_json::Value> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

/// Every folder worth searching: configured ones first, then detected.
pub fn roots(configured: &[PathBuf]) -> Vec<PathBuf> {
    let mut all: Vec<PathBuf> = configured.to_vec();
    all.extend(obsidian_vaults());
    all.extend(vscode_folders());
    all.extend(crate::orca::folders());
    let mut out: Vec<PathBuf> = Vec::new();
    for root in all {
        // Drop folders nested inside one already listed.
        if out.iter().any(|r| root.starts_with(r)) {
            continue;
        }
        out.retain(|r| !r.starts_with(&root));
        out.push(root);
    }
    out
}

pub fn markdown_files(roots: &[PathBuf]) -> Vec<PathBuf> {
    files(roots, is_markdown)
}

/// Files under `roots` that pass `want`, skipping tool and trash folders.
pub fn files(roots: &[PathBuf], want: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let keep = |e: &DirEntry| {
        let name = e.file_name().to_string_lossy();
        !(e.file_type().is_dir() && SKIP_DIRS.contains(&name.as_ref()))
    };
    roots
        .iter()
        .flat_map(|r| WalkDir::new(r).max_depth(12).into_iter().filter_entry(keep))
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && want(e.path()))
        .map(DirEntry::into_path)
        .take(MAX_FILES)
        .collect()
}

/// Inside a folder the scans skip, such as `.git` or `.trash`.
pub fn skipped(path: &Path) -> bool {
    path.components().any(|c| SKIP_DIRS.iter().any(|d| c.as_os_str() == *d))
}

pub fn is_markdown(p: &Path) -> bool {
    p.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("md") || e.eq_ignore_ascii_case("markdown"))
}

fn matches_hint(path: &Path, hint: &Hint) -> bool {
    let Some(name) = &hint.name else { return true };
    let file = path.file_name().and_then(|f| f.to_str()).unwrap_or_default();
    let stem = path.file_stem().and_then(|f| f.to_str()).unwrap_or_default();
    let name_ok = file.eq_ignore_ascii_case(name) || stem.eq_ignore_ascii_case(name);
    let folder_ok = hint.folder.as_ref().is_none_or(|f| {
        path.ancestors().any(|a| a.file_name().is_some_and(|n| n.to_string_lossy() == *f))
    });
    name_ok && folder_ok
}

fn modified(p: &Path) -> SystemTime {
    std::fs::metadata(p).and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH)
}

/// The file containing `selection`, preferring ones named in the window title
/// and, among those, the most recently modified.
pub fn find(roots: &[PathBuf], hint: &Hint, selection: &str) -> Option<PathBuf> {
    if let Some(path) = hint.path.as_ref().filter(|p| is_markdown(p)) {
        if std::fs::read_to_string(path).is_ok_and(|src| mdmark::contains(&src, selection)) {
            return Some(path.clone());
        }
    }
    let files = markdown_files(roots);
    let hinted: Vec<&PathBuf> = files.iter().filter(|p| matches_hint(p, hint)).collect();
    let pick = |candidates: Vec<&PathBuf>| {
        candidates
            .into_iter()
            .filter(|p| {
                std::fs::read_to_string(p).is_ok_and(|src| mdmark::contains(&src, selection))
            })
            .max_by_key(|p| modified(p))
            .cloned()
    };
    if hint.name.is_some() && !hinted.is_empty() {
        if let Some(found) = pick(hinted) {
            return Some(found);
        }
    }
    pick(files.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        assert_eq!(
            file_uri_to_path("file:///home/a/mis%20notas"),
            Some("/home/a/mis notas".into())
        );
        assert_eq!(file_uri_to_path("file:///c%3A/Users/x"), Some("c:/Users/x".into()));
        assert_eq!(file_uri_to_path("vscode-remote://x"), None);
    }

    #[test]
    fn finds_hinted_file() {
        let dir = std::env::temp_dir().join(format!("mh-locate-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("vault/sub")).unwrap();
        std::fs::write(dir.join("vault/a.md"), "hola **mundo**").unwrap();
        std::fs::write(dir.join("vault/sub/b.md"), "hola mundo").unwrap();
        let roots = vec![dir.clone()];
        let hint =
            Hint { name: Some("b.md".into()), folder: Some("sub".into()), ..Hint::default() };
        assert_eq!(find(&roots, &hint, "hola mundo"), Some(dir.join("vault/sub/b.md")));
        let none = Hint::default();
        assert!(find(&roots, &none, "hola mundo").is_some());
        assert_eq!(find(&roots, &none, "adiós"), None);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
