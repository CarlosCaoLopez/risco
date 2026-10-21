// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Transparent, click-through windows (one per monitor) where the marker
//! animation is drawn while the user selects.

use serde::Serialize;
use std::time::Duration;
use tauri::{
    AppHandle, Emitter, Manager, Monitor, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkerEvent {
    pub kind: &'static str,
    /// Global physical pixels, as reported by the input hook.
    pub x: f64,
    pub y: f64,
    pub color: String,
    /// On `end`: `added`, `removed` or `error`.
    pub result: Option<&'static str>,
}

/// How often monitors are checked, so plugging one in gets an overlay.
const SYNC_EVERY: Duration = Duration::from_secs(3);

pub fn create(app: &AppHandle) -> tauri::Result<()> {
    sync(app)?;
    let app = app.clone();
    std::thread::Builder::new()
        .name("overlay".into())
        .spawn(move || loop {
            std::thread::sleep(SYNC_EVERY);
            // Window toolkits want their calls on the main thread.
            let handle = app.clone();
            let _ = app.run_on_main_thread(move || {
                if let Err(e) = sync(&handle) {
                    eprintln!("overlay: {e}");
                }
            });
        })
        .expect("spawn overlay thread");
    Ok(())
}

/// Keep exactly one overlay per monitor, covering it. Labels carry the
/// monitor's origin, so a changed layout replaces the overlays that moved.
fn sync(app: &AppHandle) -> tauri::Result<()> {
    let monitors = app.available_monitors()?;
    let label = |m: &Monitor| format!("overlay-{}_{}", m.position().x, m.position().y);
    let wanted: Vec<String> = monitors.iter().map(label).collect();
    for (l, win) in app.webview_windows() {
        if l.starts_with("overlay-") && !wanted.contains(&l) {
            win.destroy()?;
        }
    }
    for m in &monitors {
        let (pos, size) = (*m.position(), *m.size());
        let win = match app.get_webview_window(&label(m)) {
            Some(win) => win,
            None => {
                let url = format!("overlay.html?x={}&y={}&s={}", pos.x, pos.y, m.scale_factor());
                // Hidden until placed and out of the window manager's hands.
                let win = WebviewWindowBuilder::new(app, label(m), WebviewUrl::App(url.into()))
                    .title("Marker overlay")
                    .transparent(true)
                    .decorations(false)
                    .shadow(false)
                    .resizable(false)
                    .always_on_top(true)
                    .visible_on_all_workspaces(true)
                    .skip_taskbar(true)
                    .focused(false)
                    .visible(false)
                    .build()?;
                #[cfg(target_os = "linux")]
                unmanage(&win);
                place(&win, pos, size)?;
                win.show()?;
                win.set_ignore_cursor_events(true)?;
                continue;
            }
        };
        if win.outer_position()? != pos || win.inner_size()? != size {
            place(&win, pos, size)?;
        }
    }
    Ok(())
}

/// On X11 a managed window of a monitor's full size gets moved by the window
/// manager (GNOME pushes it below the top bar, onto another monitor…). An
/// override-redirect window, like a tooltip, stays exactly where it is put.
#[cfg(target_os = "linux")]
fn unmanage(win: &WebviewWindow) {
    use gtk::prelude::*;
    let Ok(gtk) = win.gtk_window() else { return };
    gtk.realize();
    if let Some(gdk) = gtk.window() {
        gdk.set_override_redirect(true);
    }
}

fn place(win: &WebviewWindow, pos: PhysicalPosition<i32>, size: PhysicalSize<u32>) -> tauri::Result<()> {
    win.set_size(size)?;
    win.set_position(pos)
}

pub fn send(app: &AppHandle, event: MarkerEvent) {
    let _ = app.emit("marker", event);
}
