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

pub fn load(note: &Path) -> Vec<Record> {
    std::fs::read_to_string(sidecar(note))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
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
