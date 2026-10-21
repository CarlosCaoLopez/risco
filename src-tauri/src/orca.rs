// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Orca's window title is just "Orca", but its live profile state (SQLite)
//! records the active worktree and the file open in each one.

use rusqlite::{Connection, OpenFlags};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// The most recently used profile's state database.
fn database() -> Option<PathBuf> {
    let profiles = dirs::config_dir()?.join("orca").join("profiles");
    let modified = |p: &Path| {
        // Writes land in the WAL first, so it is the freshest timestamp.
        let wal = p.with_extension("db-wal");
        [p.to_path_buf(), wal]
            .iter()
            .filter_map(|f| std::fs::metadata(f).and_then(|m| m.modified()).ok())
            .max()
            .unwrap_or(SystemTime::UNIX_EPOCH)
    };
    std::fs::read_dir(profiles)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path().join("profile-state.db"))
        .filter(|p| p.is_file())
        .max_by_key(|p| modified(p))
}

fn document(conn: &Connection, domain: &str) -> Option<Value> {
    let payload: String = conn
        .query_row(
            "SELECT payload FROM profile_state_documents WHERE domain = ?1",
            [domain],
            |row| row.get(0),
        )
        .ok()?;
    serde_json::from_str(&payload).ok()
}

fn open() -> Option<Connection> {
    Connection::open_with_flags(database()?, OpenFlags::SQLITE_OPEN_READ_ONLY).ok()
}

/// Worktree keys look like `<repo id>::<path>`.
fn worktree_path(key: &str) -> Option<PathBuf> {
    key.split_once("::").map(|(_, p)| PathBuf::from(p))
}

/// The file shown in the active worktree's editor, if any.
pub fn active_file() -> Option<PathBuf> {
    let session = document(&open()?, "workspaceSession")?;
    active_file_in(&session)
}

fn active_file_in(session: &Value) -> Option<PathBuf> {
    let worktree = session["activeWorktreeId"].as_str()?;
    if session["activeTabTypeByWorktree"][worktree].as_str().is_some_and(|t| t != "editor") {
        return None;
    }
    session["activeFileIdByWorktree"][worktree].as_str().map(PathBuf::from)
}

/// Repositories and worktrees Orca knows about.
pub fn folders() -> Vec<PathBuf> {
    let Some(conn) = open() else { return Vec::new() };
    let mut out: Vec<PathBuf> = document(&conn, "repos")
        .and_then(|r| r.as_array().cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|r| r["path"].as_str().map(PathBuf::from))
        .collect();
    if let Some(session) = document(&conn, "workspaceSession") {
        if let Some(map) = session["activeFileIdByWorktree"].as_object() {
            out.extend(map.keys().filter_map(|k| worktree_path(k)));
        }
    }
    out.sort();
    out.dedup();
    out.retain(|p| p.is_dir());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_editor_file() {
        let session = serde_json::json!({
            "activeWorktreeId": "r1::/w/a",
            "activeTabTypeByWorktree": { "r1::/w/a": "editor" },
            "activeFileIdByWorktree": { "r1::/w/a": "/w/a/notas.md", "r2::/w/b": "/w/b/x.md" }
        });
        assert_eq!(active_file_in(&session), Some(PathBuf::from("/w/a/notas.md")));

        let terminal = serde_json::json!({
            "activeWorktreeId": "r1::/w/a",
            "activeTabTypeByWorktree": { "r1::/w/a": "terminal" },
            "activeFileIdByWorktree": { "r1::/w/a": "/w/a/notas.md" }
        });
        assert_eq!(active_file_in(&terminal), None);
        assert_eq!(worktree_path("r1::/w/a"), Some(PathBuf::from("/w/a")));
    }
}
