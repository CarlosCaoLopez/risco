//! Transparent, click-through windows (one per monitor) where the marker
//! animation is drawn while the user selects.

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder};

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

pub fn create(app: &AppHandle) -> tauri::Result<()> {
    for (i, m) in app.available_monitors()?.iter().enumerate() {
        let label = format!("overlay-{i}");
        if app.get_webview_window(&label).is_some() {
            continue;
        }
        let (pos, size, scale) = (m.position(), m.size(), m.scale_factor());
        let url = format!("overlay.html?x={}&y={}&s={}", pos.x, pos.y, scale);
        let win = WebviewWindowBuilder::new(app, &label, WebviewUrl::App(url.into()))
            .title("Marker overlay")
            .transparent(true)
            .decorations(false)
            .shadow(false)
            .resizable(false)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .skip_taskbar(true)
            .focused(false)
            .build()?;
        win.set_position(PhysicalPosition::new(pos.x, pos.y))?;
        win.set_size(PhysicalSize::new(size.width, size.height))?;
        win.set_ignore_cursor_events(true)?;
    }
    Ok(())
}

pub fn send(app: &AppHandle, event: MarkerEvent) {
    let _ = app.emit("marker", event);
}
