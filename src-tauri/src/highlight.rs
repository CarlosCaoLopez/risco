// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Turns finished gestures into edits: read the selection, find the file,
//! toggle the highlight, and drive the animation and sound along the way.

use crate::config::Config;
use crate::input::Gesture;
use crate::overlay::{self, MarkerEvent};
use crate::sound::Sound;
use crate::{i18n, locate, selection, store, window};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// New contents for a file; `None` deletes it.
#[derive(Debug, Clone)]
pub struct Change {
    pub path: PathBuf,
    pub contents: Option<String>,
}

/// The files as they were before the last edit, for "Deshacer último".
pub type Undo = Arc<Mutex<Vec<Change>>>;

pub fn spawn(app: AppHandle, config: Arc<RwLock<Config>>, sound: Sound, undo: Undo, rx: Receiver<Gesture>) {
    std::thread::Builder::new()
        .name("highlight".into())
        .spawn(move || {
            let mut last = (0.0, 0.0, Instant::now());
            let mut speed = 0.0f32;
            // Whether the current stroke started in a reading view; others are ignored.
            let mut active = false;
            for gesture in rx {
                let cfg = config.read().expect("config lock").clone();
                let event = |kind, x, y, result| MarkerEvent { kind, x, y, color: cfg.color.clone(), result };
                match gesture {
                    Gesture::Start { x, y } => {
                        active = window::reading_view();
                        if !active {
                            continue;
                        }
                        last = (x, y, Instant::now());
                        speed = 0.0;
                        if cfg.sound {
                            sound.start(cfg.volume);
                        }
                        if cfg.animation {
                            overlay::send(&app, event("start", x, y, None));
                        }
                    }
                    Gesture::Move { .. } | Gesture::End { .. } if !active => {}
                    Gesture::Move { x, y } => {
                        let dt = last.2.elapsed().as_secs_f32().max(1e-3);
                        let dist = ((x - last.0).powi(2) + (y - last.1).powi(2)).sqrt() as f32;
                        speed += (dist / dt - speed) * 0.35;
                        last = (x, y, Instant::now());
                        if cfg.sound {
                            sound.speed(speed);
                        }
                        if cfg.animation {
                            overlay::send(&app, event("move", x, y, None));
                        }
                    }
                    Gesture::End { x, y } => {
                        if cfg.sound {
                            sound.speed(0.0);
                        }
                        let result = apply(&cfg, &undo);
                        if let Err(e) = &result {
                            eprintln!("risco: error: {e}");
                        }
                        if cfg.sound {
                            sound.stop(result.is_ok());
                        }
                        let tag = match &result {
                            Ok(mdmark::Action::Added) => "added",
                            Ok(mdmark::Action::Removed) => "removed",
                            Err(_) => "error",
                        };
                        if cfg.animation {
                            overlay::send(&app, event("end", x, y, Some(tag)));
                        }
                        if let Err(msg) = result {
                            notify(&app, &msg);
                        }
                    }
                }
            }
        })
        .expect("spawn highlight thread");
}

fn apply(cfg: &Config, undo: &Undo) -> Result<mdmark::Action, String> {
    selection::settle();
    let text = selection::selected_text().ok_or_else(|| i18n::t("error.no_selection"))?;
    let (app, hint) = window::active().unwrap_or((window::App::Other, window::Hint::default()));
    if hint.dirty {
        return Err(i18n::t("error.unsaved"));
    }
    let roots = locate::roots(&cfg.roots);
    let path = locate::find(&roots, &hint, &text).ok_or_else(|| {
        if roots.is_empty() {
            i18n::t("error.no_roots")
        } else {
            i18n::t("error.not_found")
        }
    })?;

    let source = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    // Orca's editor shows HTML as text and loads no plugins: only `++` shows.
    let plus = app == window::App::Orca;
    // A note moved while nobody watched gets its old highlights back first.
    store::adopt(&path, &roots);
    let edit = store::toggle(&path, &source, &text, &cfg.color, plus)?;
    *undo.lock().expect("undo lock") = commit(&edit.changes).map_err(|e| e.to_string())?;
    eprintln!("risco: {app:?} {} → {:?}", path.display(), edit.action);
    Ok(edit.action)
}

/// Apply `changes`, returning what undoes them.
pub fn commit(changes: &[Change]) -> std::io::Result<Vec<Change>> {
    let before: Vec<Change> = changes
        .iter()
        .map(|c| Change { path: c.path.clone(), contents: std::fs::read_to_string(&c.path).ok() })
        .collect();
    for c in changes {
        apply_change(c)?;
    }
    Ok(before)
}

pub fn apply_change(change: &Change) -> std::io::Result<()> {
    match &change.contents {
        Some(text) => write_atomic(&change.path, text),
        None => match std::fs::remove_file(&change.path) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        },
    }
}

/// Write beside the file and rename over it, so viewers never see half a file.
pub fn write_atomic(path: &Path, contents: &str) -> std::io::Result<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".mh-tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, contents)?;
    if let Ok(meta) = std::fs::metadata(path) {
        let _ = std::fs::set_permissions(&tmp, meta.permissions());
    }
    std::fs::rename(&tmp, path)
}

pub fn notify(app: &AppHandle, body: &str) {
    let _ = app.notification().builder().title("Risco").body(body).show();
}
