// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Global mouse/keyboard hook that recognises "modifier + drag".
//!
//! rdev reports physical keys, so a remapped modifier (`ctrl:nocaps`, xmodmap)
//! looks like some other key. On X11 the modifier is therefore read from the
//! server's logical state when the button goes down; key tracking is the fallback.

use crate::config::{Config, Modifier};
use rdev::{Button, EventType, Key};
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

/// Pointer travel before a press with the modifier counts as a stroke.
const DRAG_THRESHOLD: f64 = 4.0;

#[derive(Debug, Clone, Copy)]
pub enum Gesture {
    Start { x: f64, y: f64 },
    Move { x: f64, y: f64 },
    End { x: f64, y: f64 },
}

fn is_modifier(key: Key, m: Modifier) -> bool {
    match m {
        Modifier::Ctrl => matches!(key, Key::ControlLeft | Key::ControlRight),
        Modifier::Alt => matches!(key, Key::Alt | Key::AltGr),
        Modifier::Shift => matches!(key, Key::ShiftLeft | Key::ShiftRight),
        Modifier::Meta => matches!(key, Key::MetaLeft | Key::MetaRight),
    }
}

#[cfg(target_os = "linux")]
mod x11_state {
    use super::Modifier;
    use std::os::raw::{c_int, c_uint};
    use x11::xlib;

    pub struct Display(*mut xlib::Display);

    impl Display {
        pub fn open() -> Option<Self> {
            let d = unsafe { xlib::XOpenDisplay(std::ptr::null()) };
            (!d.is_null()).then_some(Self(d))
        }

        pub fn modifier_held(&self, m: Modifier) -> bool {
            let want = match m {
                Modifier::Ctrl => xlib::ControlMask,
                Modifier::Alt => xlib::Mod1Mask | xlib::Mod5Mask,
                Modifier::Shift => xlib::ShiftMask,
                Modifier::Meta => xlib::Mod4Mask,
            };
            let (mut root, mut child) = (0, 0);
            let (mut rx, mut ry, mut wx, mut wy): (c_int, c_int, c_int, c_int) = (0, 0, 0, 0);
            let mut mask: c_uint = 0;
            let ok = unsafe {
                xlib::XQueryPointer(
                    self.0,
                    xlib::XDefaultRootWindow(self.0),
                    &mut root,
                    &mut child,
                    &mut rx,
                    &mut ry,
                    &mut wx,
                    &mut wy,
                    &mut mask,
                )
            };
            ok != 0 && mask & want != 0
        }
    }

    impl Drop for Display {
        fn drop(&mut self) {
            unsafe { xlib::XCloseDisplay(self.0) };
        }
    }
}

pub fn spawn(config: Arc<RwLock<Config>>, tx: Sender<Gesture>) {
    std::thread::Builder::new()
        .name("input".into())
        .spawn(move || {
            let mut held = false;
            let mut armed = false; // left button went down while the modifier was held
            let mut active = false;
            let mut pos = (0.0, 0.0);
            let mut origin = (0.0, 0.0);
            #[cfg(target_os = "linux")]
            let display = x11_state::Display::open();

            let result = rdev::listen(move |event| {
                let (enabled, modifier) = {
                    let c = config.read().expect("config lock");
                    (c.enabled, c.modifier)
                };
                match event.event_type {
                    EventType::KeyPress(k) if is_modifier(k, modifier) => held = true,
                    EventType::KeyRelease(k) if is_modifier(k, modifier) => held = false,
                    EventType::MouseMove { x, y } => {
                        pos = (x, y);
                        if armed && !active {
                            let (dx, dy) = (x - origin.0, y - origin.1);
                            if (dx * dx + dy * dy).sqrt() > DRAG_THRESHOLD {
                                active = true;
                                let _ = tx.send(Gesture::Start { x: origin.0, y: origin.1 });
                            }
                        }
                        if active {
                            let _ = tx.send(Gesture::Move { x, y });
                        }
                    }
                    EventType::ButtonPress(Button::Left) => {
                        #[cfg(target_os = "linux")]
                        let held = display.as_ref().map_or(held, |d| d.modifier_held(modifier));
                        armed = enabled && held;
                        origin = pos;
                    }
                    EventType::ButtonRelease(Button::Left) => {
                        if active {
                            let _ = tx.send(Gesture::End { x: pos.0, y: pos.1 });
                        }
                        armed = false;
                        active = false;
                    }
                    _ => {}
                }
            });
            if let Err(e) = result {
                eprintln!("no se pudo escuchar el ratón/teclado: {e:?}");
            }
        })
        .expect("spawn input thread");
}
