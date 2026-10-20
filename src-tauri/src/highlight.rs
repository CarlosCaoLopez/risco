//! Turns finished gestures into edits: read the selection, find the file,
//! toggle the highlight, and drive the animation and sound along the way.

use crate::config::Config;
use crate::input::Gesture;
use crate::overlay::{self, MarkerEvent};
use crate::sound::Sound;
use crate::{locate, selection, styles, window};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// The file as it was before the last edit, for "Deshacer último".
pub type Undo = Arc<Mutex<Option<(PathBuf, String)>>>;

pub fn spawn(app: AppHandle, config: Arc<RwLock<Config>>, sound: Sound, undo: Undo, rx: Receiver<Gesture>) {
    std::thread::Builder::new()
        .name("highlight".into())
        .spawn(move || {
            let mut last = (0.0, 0.0, Instant::now());
            let mut speed = 0.0f32;
            for gesture in rx {
                let cfg = config.read().expect("config lock").clone();
                let event = |kind, x, y, result| MarkerEvent { kind, x, y, color: cfg.color.clone(), result };
                match gesture {
                    Gesture::Start { x, y } => {
                        last = (x, y, Instant::now());
                        speed = 0.0;
                        if cfg.sound {
                            sound.start(cfg.volume);
                        }
                        if cfg.animation {
                            overlay::send(&app, event("start", x, y, None));
                        }
                    }
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
    let text = selection::selected_text().ok_or("No hay texto seleccionado.")?;
    let (_, hint) = window::active().unwrap_or((window::App::Other, window::Hint::default()));
    if hint.dirty {
        return Err("El archivo tiene cambios sin guardar. Guárdalo y vuelve a subrayar.".into());
    }
    let roots = locate::roots(&cfg.roots);
    let path = locate::find(&roots, &hint, &text).ok_or_else(|| {
        if roots.is_empty() {
            "No hay carpetas configuradas. Añade la carpeta de tus notas en Ajustes.".to_string()
        } else {
            "No encontré ese texto en ningún .md de tus carpetas.".to_string()
        }
    })?;

    let source = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let css = cfg.inline_style.then(|| styles::inline_css(&cfg.color));
    let style = mdmark::Style { color: &cfg.color, inline_css: css.as_deref() };
    let out = mdmark::toggle(&source, &text, style).map_err(|e| e.to_string())?;
    write_atomic(&path, &out.source).map_err(|e| e.to_string())?;
    *undo.lock().expect("undo lock") = Some((path, source));
    Ok(out.action)
}

/// Write beside the file and rename over it, so viewers never see half a file.
pub fn write_atomic(path: &Path, contents: &str) -> std::io::Result<()> {
    let tmp = path.with_extension("md.mh-tmp");
    std::fs::write(&tmp, contents)?;
    if let Ok(meta) = std::fs::metadata(path) {
        let _ = std::fs::set_permissions(&tmp, meta.permissions());
    }
    std::fs::rename(&tmp, path)
}

pub fn notify(app: &AppHandle, body: &str) {
    let _ = app.notification().builder().title("Subrayador").body(body).show();
}
