//! Global mouse/keyboard hook that recognises "modifier + drag".

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

pub fn spawn(config: Arc<RwLock<Config>>, tx: Sender<Gesture>) {
    std::thread::Builder::new()
        .name("input".into())
        .spawn(move || {
            let mut held = false;
            let mut armed = false; // left button went down while the modifier was held
            let mut active = false;
            let mut pos = (0.0, 0.0);
            let mut origin = (0.0, 0.0);

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
