//! Read the text currently selected in whatever window has focus.

use std::time::Duration;

/// On X11 the selection lives in PRIMARY, so the clipboard is left alone.
#[cfg(target_os = "linux")]
pub fn selected_text() -> Option<String> {
    use arboard::{GetExtLinux, LinuxClipboardKind};
    let mut cb = arboard::Clipboard::new().ok()?;
    let text = cb.get().clipboard(LinuxClipboardKind::Primary).text().ok()?;
    non_empty(text)
}

/// Elsewhere: copy with the system shortcut, read, then restore the clipboard.
#[cfg(not(target_os = "linux"))]
pub fn selected_text() -> Option<String> {
    use enigo::{Direction, Enigo, Key, Keyboard, Settings};

    let mut cb = arboard::Clipboard::new().ok()?;
    let previous = cb.get_text().ok();
    let _ = cb.clear();

    let modifier = if cfg!(target_os = "macos") { Key::Meta } else { Key::Control };
    let mut enigo = Enigo::new(&Settings::default()).ok()?;
    let _ = enigo.key(modifier, Direction::Press);
    let _ = enigo.key(Key::Unicode('c'), Direction::Click);
    let _ = enigo.key(modifier, Direction::Release);
    std::thread::sleep(Duration::from_millis(120));

    let text = cb.get_text().ok();
    if let Some(prev) = previous {
        let _ = cb.set_text(prev);
    }
    non_empty(text?)
}

/// Give the app under the pointer time to publish its selection.
pub fn settle() {
    std::thread::sleep(Duration::from_millis(80));
}

fn non_empty(text: String) -> Option<String> {
    (!text.trim().is_empty()).then_some(text)
}
