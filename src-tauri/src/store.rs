//! Highlights kept beside each note in a hidden JSONL file, so the Markdown
//! stays clean. `notas/clase.md` keeps them in `notas/.clase.md.hl.jsonl`, one
//! text quote per line; the viewer plugins read it and paint the marks.
//!
//! Orca can't load plugins, so highlights made from Orca are also written into
//! the note as `++…++` and flagged `inline` here, which keeps their color.

use crate::highlight::Change;
use mdmark::{Action, Quote, Syntax};
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub exact: String,
    pub prefix: String,
    pub suffix: String,
    pub color: String,
    /// Also written into the note as `++…++`.
    #[serde(default)]
    pub inline: bool,
    /// Milliseconds since the Unix epoch.
    #[serde(default)]
    pub created: u64,
}

impl Record {
    fn new(quote: Quote, color: &str, inline: bool) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
        Self {
            id: format!("{:x}", now.as_nanos()),
            exact: quote.exact,
            prefix: quote.prefix,
            suffix: quote.suffix,
            color: color.to_string(),
            inline,
            created: now.as_millis() as u64,
        }
    }

    fn quote(&self) -> Quote {
        Quote { exact: self.exact.clone(), prefix: self.prefix.clone(), suffix: self.suffix.clone() }
    }
}

/// `dir/.name.md.hl.jsonl` for `dir/name.md`.
pub fn sidecar(note: &Path) -> PathBuf {
    let name = note.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    note.with_file_name(format!(".{name}.hl.jsonl"))
}

/// `dir/name.md` for `dir/.name.md.hl.jsonl`, the inverse of `sidecar`.
pub fn note_of(sidecar: &Path) -> Option<PathBuf> {
    let name = sidecar.file_name()?.to_str()?;
    let note = name.strip_prefix('.')?.strip_suffix(".hl.jsonl")?;
    (!note.is_empty()).then(|| sidecar.with_file_name(note))
}

pub fn load(note: &Path) -> Vec<Record> {
    read(&sidecar(note))
}

fn read(sidecar: &Path) -> Vec<Record> {
    std::fs::read_to_string(sidecar)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Move `from`'s sidecar beside `to`, for a note that was moved or renamed.
/// Does nothing when there is none or `to` already has one (a viewer plugin
/// got there first).
pub fn relocate(from: &Path, to: &Path) -> std::io::Result<bool> {
    let (src, dst) = (sidecar(from), sidecar(to));
    if !src.exists() || dst.exists() {
        return Ok(false);
    }
    move_file(&src, &dst)?;
    Ok(true)
}

fn move_file(src: &Path, dst: &Path) -> std::io::Result<()> {
    if std::fs::rename(src, dst).is_err() {
        // Renaming fails across file systems: copy, then remove.
        std::fs::copy(src, dst)?;
        std::fs::remove_file(src)?;
    }
    Ok(())
}

/// Sidecars under `roots` whose note is gone.
fn orphans(roots: &[PathBuf]) -> Vec<PathBuf> {
    crate::locate::files(roots, |p| note_of(p).is_some_and(|n| !n.exists()))
}

/// Give `note` the sidecar it left behind when it was moved without its
/// viewer knowing: an orphan with the same file name whose highlights are
/// found in the note. Returns where it came from.
pub fn adopt(note: &Path, roots: &[PathBuf]) -> Option<PathBuf> {
    if sidecar(note).exists() {
        return None;
    }
    adopt_from(note, &orphans(roots))
}

/// Run `adopt` for every note that could take an orphan, e.g. after notes
/// were moved while the app was closed.
pub fn adopt_orphans(roots: &[PathBuf]) -> Vec<PathBuf> {
    let orphans = orphans(roots);
    if orphans.is_empty() {
        return Vec::new();
    }
    let names: Vec<_> = orphans.iter().filter_map(|o| o.file_name()).collect();
    crate::locate::markdown_files(roots)
        .into_iter()
        .filter(|n| names.contains(&sidecar(n).file_name().unwrap_or_default()))
        .filter(|n| adopt_from(n, &orphans).is_some())
        .collect()
}

/// `adopt` with a known list of orphan sidecars.
pub fn adopt_from(note: &Path, orphans: &[PathBuf]) -> Option<PathBuf> {
    let target = sidecar(note);
    if target.exists() {
        return None;
    }
    let source = std::fs::read_to_string(note).ok()?;
    let found = |o: &PathBuf| read(o).iter().filter(|r| mdmark::resolve(&source, &r.quote()).is_some()).count();
    let (best, _) = orphans
        .iter()
        .filter(|o| o.file_name() == target.file_name())
        .map(|o| (o, found(o)))
        .filter(|&(_, n)| n > 0)
        .max_by_key(|&(_, n)| n)?;
    move_file(best, &target).ok()?;
    Some(best.clone())
}

/// The sidecar contents, or `None` to delete it once it is empty.
fn render(records: &[Record]) -> Option<String> {
    if records.is_empty() {
        return None;
    }
    let lines: Vec<String> = records.iter().filter_map(|r| serde_json::to_string(r).ok()).collect();
    Some(lines.join("\n") + "\n")
}

fn overlaps(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

/// What a gesture changes: the note, its sidecar, or both.
pub struct Edit {
    pub action: Action,
    pub changes: Vec<Change>,
}

/// Toggle `selection` in `note`. With `plus` (Orca), or when the selection is
/// already inside a mark in the note, the note itself is edited; otherwise only
/// the sidecar changes.
pub fn toggle(note: &Path, source: &str, selection: &str, color: &str, plus: bool) -> Result<Edit, String> {
    let records = load(note);
    if plus || mdmark::marked(source, selection) {
        let out = mdmark::toggle(source, selection, Syntax::Plus).map_err(|e| e.to_string())?;
        let records = sync_inline(source, &out, records, color);
        return Ok(Edit {
            action: out.action,
            changes: vec![
                Change { path: note.to_path_buf(), contents: Some(out.source) },
                Change { path: sidecar(note), contents: render(&records) },
            ],
        });
    }

    let (inline, mut plain): (Vec<Record>, Vec<Record>) = records.into_iter().partition(|r| r.inline);
    let quotes: Vec<Quote> = plain.iter().map(Record::quote).collect();
    let edit = mdmark::toggle_quotes(source, &quotes, selection).map_err(|e| e.to_string())?;
    let mut i = 0;
    plain.retain(|_| {
        i += 1;
        !edit.remove.contains(&(i - 1))
    });
    plain.extend(edit.add.map(|q| Record::new(q, color, false)));
    let records: Vec<Record> = inline.into_iter().chain(plain).collect();
    Ok(Edit { action: edit.action, changes: vec![Change { path: sidecar(note), contents: render(&records) }] })
}

/// Rebuild the `inline` records from the `++` marks in the edited note. Marks
/// never change the plain text, so ranges from before and after the edit line up.
fn sync_inline(before: &str, out: &mdmark::Outcome, records: Vec<Record>, color: &str) -> Vec<Record> {
    let (inline, plain): (Vec<Record>, Vec<Record>) = records.into_iter().partition(|r| r.inline);
    let located: Vec<(Record, Option<Range<usize>>)> = inline
        .into_iter()
        .map(|r| {
            let at = mdmark::resolve(before, &r.quote());
            (r, at)
        })
        .collect();
    let added = out.action == Action::Added;

    let mut next: Vec<Record> = plain
        .into_iter()
        // A sidecar highlight now drawn in the note would show twice.
        .filter(|r| !(added && mdmark::resolve(before, &r.quote()).is_some_and(|q| overlaps(&q, &out.range))))
        .collect();
    for mark in mdmark::marks(&out.source).into_iter().filter(|m| m.plus) {
        let old = located.iter().find(|(_, q)| q.as_ref().is_some_and(|q| overlaps(q, &mark.range)));
        let record = match old {
            Some((r, _)) if !(added && overlaps(&mark.range, &out.range)) => {
                Record { exact: mark.quote.exact, prefix: mark.quote.prefix, suffix: mark.quote.suffix, ..r.clone() }
            }
            _ => Record::new(mark.quote, color, true),
        };
        next.push(record);
    }
    next
}

/// Move `<mark class="hl">` tags out of `note` into its sidecar.
pub fn migrate(note: &Path) -> Option<Vec<Change>> {
    let source = std::fs::read_to_string(note).ok()?;
    let html: Vec<mdmark::Mark> = mdmark::marks(&source).into_iter().filter(|m| !m.plus).collect();
    if html.is_empty() {
        return None;
    }
    let mut records = load(note);
    for m in html {
        let color = m.color.unwrap_or_else(|| mdmark::COLORS[0].to_string());
        records.push(Record::new(m.quote, &color, false));
    }
    Some(vec![
        Change { path: note.to_path_buf(), contents: Some(mdmark::strip_html_marks(&source)) },
        Change { path: sidecar(note), contents: render(&records) },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(name: &str, text: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("mh-store-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("nota.md");
        std::fs::write(&p, text).unwrap();
        let _ = std::fs::remove_file(sidecar(&p));
        p
    }

    fn apply(changes: &[Change]) {
        for c in changes {
            crate::highlight::apply_change(c).unwrap();
        }
    }

    #[test]
    fn sidecar_name() {
        assert_eq!(sidecar(Path::new("/a/b/clase.md")), PathBuf::from("/a/b/.clase.md.hl.jsonl"));
    }

    #[test]
    fn note_of_inverts_sidecar() {
        let p = Path::new("/a/b/clase.md");
        assert_eq!(note_of(&sidecar(p)).as_deref(), Some(p));
        assert_eq!(note_of(Path::new("/a/clase.md")), None);
        assert_eq!(note_of(Path::new("/a/..hl.jsonl")), None);
    }

    #[test]
    fn relocate_moves_sidecar() {
        let p = note("relocate", "Hola mundo");
        apply(&toggle(&p, "Hola mundo", "mundo", "pink", false).unwrap().changes);
        let to = p.with_file_name("otra.md");
        assert!(relocate(&p, &to).unwrap());
        assert!(!sidecar(&p).exists());
        assert_eq!(load(&to)[0].exact, "mundo");
        // Nothing left to move.
        assert!(!relocate(&p, &to).unwrap());
        std::fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }

    #[test]
    fn adopts_the_matching_orphan() {
        let root = std::env::temp_dir().join(format!("mh-store-adopt-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for d in ["a", "b", "c", "nueva"] {
            std::fs::create_dir_all(root.join(d)).unwrap();
        }
        // Two notes named nota.md were moved away; a third still has its note.
        let a = root.join("a/nota.md");
        let b = root.join("b/nota.md");
        let c = root.join("c/nota.md");
        for (p, text, sel) in [(&a, "Hola mundo", "mundo"), (&b, "Adiós luna", "luna"), (&c, "Adiós luna", "luna")] {
            std::fs::write(p, text).unwrap();
            apply(&toggle(p, text, sel, "blue", false).unwrap().changes);
        }
        std::fs::remove_file(&a).unwrap();
        std::fs::remove_file(&b).unwrap();
        let moved = root.join("nueva/nota.md");
        std::fs::write(&moved, "Adiós luna").unwrap();

        let roots = vec![root.clone()];
        assert_eq!(adopt(&moved, &roots), Some(sidecar(&b)));
        assert_eq!(load(&moved)[0].exact, "luna");
        assert!(sidecar(&a).exists() && sidecar(&c).exists());
        // Already has one now.
        assert_eq!(adopt(&moved, &roots), None);

        let other = root.join("nueva/otra.md");
        std::fs::rename(&moved, &other).unwrap();
        relocate(&moved, &other).unwrap();
        std::fs::write(&moved, "Hola mundo").unwrap();
        assert_eq!(adopt_orphans(&roots), vec![moved.clone()]);
        assert_eq!(load(&moved)[0].exact, "mundo");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn viewer_highlights_leave_note_alone() {
        let p = note("viewer", "Hola mundo feliz");
        let e = toggle(&p, "Hola mundo feliz", "mundo", "pink", false).unwrap();
        assert_eq!(e.action, Action::Added);
        assert_eq!(e.changes.len(), 1);
        apply(&e.changes);
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "Hola mundo feliz");
        let r = load(&p);
        assert_eq!((r[0].exact.as_str(), r[0].color.as_str(), r[0].inline), ("mundo", "pink", false));

        let e = toggle(&p, "Hola mundo feliz", "mundo", "pink", false).unwrap();
        assert_eq!(e.action, Action::Removed);
        apply(&e.changes);
        assert!(!sidecar(&p).exists());
        std::fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }

    #[test]
    fn orca_writes_plus_and_keeps_color() {
        let p = note("orca", "Hola mundo feliz");
        let e = toggle(&p, "Hola mundo feliz", "mundo", "green", true).unwrap();
        apply(&e.changes);
        let src = std::fs::read_to_string(&p).unwrap();
        assert_eq!(src, "Hola ++mundo++ feliz");
        let r = load(&p);
        assert_eq!((r[0].exact.as_str(), r[0].color.as_str(), r[0].inline), ("mundo", "green", true));

        // Another mark keeps the first one's color.
        let e = toggle(&p, &src, "Hola", "blue", true).unwrap();
        apply(&e.changes);
        let mut colors: Vec<String> = load(&p).into_iter().map(|r| r.color).collect();
        colors.sort();
        assert_eq!(colors, ["blue", "green"]);

        // Removing it from a viewer clears the `++` too.
        let src = std::fs::read_to_string(&p).unwrap();
        let e = toggle(&p, &src, "mundo", "blue", false).unwrap();
        assert_eq!(e.action, Action::Removed);
        apply(&e.changes);
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "++Hola++ mundo feliz");
        assert_eq!(load(&p).len(), 1);
        std::fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }

    #[test]
    fn orca_replaces_sidecar_highlight() {
        let p = note("both", "Hola mundo feliz");
        apply(&toggle(&p, "Hola mundo feliz", "mundo", "pink", false).unwrap().changes);
        apply(&toggle(&p, "Hola mundo feliz", "mundo", "pink", true).unwrap().changes);
        let r = load(&p);
        assert_eq!(r.len(), 1);
        assert!(r[0].inline);
        std::fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }

    #[test]
    fn migrates_html_marks() {
        let p = note("migrate", r#"a <mark class="hl hl-blue" style="x">b</mark> c"#);
        apply(&migrate(&p).unwrap());
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "a b c");
        let r = load(&p);
        assert_eq!((r[0].exact.as_str(), r[0].color.as_str()), ("b", "blue"));
        assert!(migrate(&p).is_none());
        std::fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }
}
