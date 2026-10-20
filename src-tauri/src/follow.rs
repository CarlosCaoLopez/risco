//! Watches the note folders so a note's sidecar follows it when the note is
//! moved outside its viewer (file manager, `mv`, git…). Viewer plugins only
//! see renames made inside the viewer.

use crate::config::Config;
use crate::{locate, store};
use notify::event::{ModifyKind, RenameMode};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// How often the set of roots is recomputed (new vaults, settings changes).
const ROOTS_EVERY: Duration = Duration::from_secs(60);
/// Quiet time before a burst of events is acted on. A move across file
/// systems is a copy then a delete, so both halves must be in.
const SETTLE: Duration = Duration::from_millis(800);

#[derive(Default)]
struct Moves {
    /// Sidecars whose note just went away.
    orphans: Vec<PathBuf>,
    /// Notes that just appeared.
    arrived: Vec<PathBuf>,
}

fn is_note(p: &Path) -> bool {
    locate::is_markdown(p) && !locate::skipped(p)
}

impl Moves {
    fn left(&mut self, note: &Path) {
        let s = store::sidecar(note);
        if s.exists() && !self.orphans.contains(&s) {
            self.orphans.push(s);
        }
    }

    fn record(&mut self, event: Event) {
        let paths = &event.paths;
        match event.kind {
            EventKind::Modify(ModifyKind::Name(RenameMode::Both)) if paths.len() == 2 => {
                let (from, to) = (&paths[0], &paths[1]);
                match (is_note(from), is_note(to)) {
                    (true, true) => {
                        if let Err(e) = store::relocate(from, to) {
                            eprintln!("no se pudo mover el subrayado de {}: {e}", from.display());
                        }
                    }
                    (true, false) => self.left(from),
                    (false, true) => self.arrived.push(to.clone()),
                    (false, false) => {}
                }
            }
            EventKind::Remove(_) | EventKind::Modify(ModifyKind::Name(RenameMode::From)) => {
                paths.iter().filter(|p| is_note(p)).for_each(|p| self.left(p));
            }
            EventKind::Create(_) | EventKind::Modify(ModifyKind::Name(RenameMode::To)) => {
                self.arrived.extend(paths.iter().filter(|p| is_note(p)).cloned());
            }
            // Platforms that don't say which end of a rename this is.
            EventKind::Modify(ModifyKind::Name(_)) => {
                for p in paths.iter().filter(|p| is_note(p)) {
                    if p.exists() {
                        self.arrived.push(p.clone());
                    } else {
                        self.left(p);
                    }
                }
            }
            _ => {}
        }
    }

    /// Hand each orphan to a note that arrived with its name and highlights.
    fn settle(&mut self) {
        self.orphans
            .retain(|s| s.exists() && store::note_of(s).is_some_and(|n| !n.exists()));
        for note in std::mem::take(&mut self.arrived) {
            if self.orphans.is_empty() {
                break;
            }
            if note.exists() {
                if let Some(from) = store::adopt_from(&note, &self.orphans) {
                    self.orphans.retain(|s| *s != from);
                }
            }
        }
    }
}

pub fn spawn(config: Arc<RwLock<Config>>) {
    std::thread::Builder::new()
        .name("follow".into())
        .spawn(move || {
            let (tx, rx) = mpsc::channel();
            let mut watcher = match notify::recommended_watcher(tx) {
                Ok(w) => w,
                Err(e) => return eprintln!("no se pueden vigilar las carpetas: {e}"),
            };
            let mut watched: Vec<PathBuf> = Vec::new();
            let mut checked = Instant::now() - ROOTS_EVERY;
            let mut moves = Moves::default();
            loop {
                if checked.elapsed() >= ROOTS_EVERY {
                    checked = Instant::now();
                    let configured = config.read().expect("config lock").roots.clone();
                    let roots = locate::roots(&configured);
                    if roots != watched {
                        for r in watched.iter().filter(|r| !roots.contains(r)) {
                            let _ = watcher.unwatch(r);
                        }
                        for r in roots.iter().filter(|r| !watched.contains(r)) {
                            if let Err(e) = watcher.watch(r, RecursiveMode::Recursive) {
                                eprintln!("no se puede vigilar {}: {e}", r.display());
                            }
                        }
                        watched = roots;
                        // Notes moved while the app was closed.
                        store::adopt_orphans(&watched);
                    }
                }
                match rx.recv_timeout(SETTLE) {
                    Ok(Ok(event)) => moves.record(event),
                    Ok(Err(e)) => eprintln!("vigilancia de carpetas: {e}"),
                    Err(RecvTimeoutError::Timeout) => moves.settle(),
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }
        })
        .expect("spawn follow thread");
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{CreateKind, RemoveKind};

    fn event(kind: EventKind, paths: &[&Path]) -> Event {
        paths.iter().fold(Event::new(kind), |e, p| e.add_path(p.to_path_buf()))
    }

    #[test]
    fn sidecar_follows_moves() {
        let root = std::env::temp_dir().join(format!("mh-follow-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("a")).unwrap();
        std::fs::create_dir_all(root.join("b")).unwrap();
        let a = root.join("a/nota.md");
        let b = root.join("b/nota.md");
        std::fs::write(&a, "Hola mundo").unwrap();
        std::fs::write(store::sidecar(&a), r#"{"id":"1","exact":"mundo","prefix":"Hola ","suffix":"","color":"blue"}"#).unwrap();

        // A rename the OS pairs up.
        let mut moves = Moves::default();
        std::fs::rename(&a, &b).unwrap();
        moves.record(event(EventKind::Modify(ModifyKind::Name(RenameMode::Both)), &[&a, &b]));
        assert!(store::sidecar(&b).exists() && !store::sidecar(&a).exists());

        // A copy to another disk, then the old note is deleted.
        std::fs::copy(&b, &a).unwrap();
        moves.record(event(EventKind::Create(CreateKind::File), &[&a]));
        std::fs::remove_file(&b).unwrap();
        moves.record(event(EventKind::Remove(RemoveKind::File), &[&b]));
        moves.settle();
        assert!(store::sidecar(&a).exists() && !store::sidecar(&b).exists());
        assert!(moves.orphans.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}
