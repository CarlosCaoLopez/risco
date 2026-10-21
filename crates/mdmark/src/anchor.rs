// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Highlights stored outside the source, anchored by the text they cover.
//!
//! A [`Quote`] is the highlighted plain text plus a little context on either
//! side, like a W3C TextQuoteSelector. The context picks the right occurrence
//! when the text repeats and lets the anchor survive edits elsewhere in the
//! file. Indices returned here are plain-text characters of the projection.

use crate::{normalize, Action, Error, Projection};
use std::ops::Range;

/// Characters of context kept on each side of a quote.
const CONTEXT: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    pub exact: String,
    pub prefix: String,
    pub suffix: String,
}

/// What a gesture does to a list of stored quotes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteEdit {
    pub action: Action,
    /// Indices into the existing quotes to drop.
    pub remove: Vec<usize>,
    /// The quote to store, when adding. Overlapped quotes are merged into it.
    pub add: Option<Quote>,
    /// The selection appeared more than once; the first suitable one was used.
    pub ambiguous: bool,
}

/// A highlight written into the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    pub quote: Quote,
    pub range: Range<usize>,
    /// `++…++` rather than a `<mark>` tag.
    pub plus: bool,
    /// The `hl-COLOR` of a `<mark>` tag.
    pub color: Option<String>,
}

/// Where `quote` sits in `source` now, if it is still there.
pub fn resolve(source: &str, quote: &Quote) -> Option<Range<usize>> {
    Projection::new(source).resolve(quote)
}

/// Every highlight mark in `source`, with the text it covers.
pub fn marks(source: &str) -> Vec<Mark> {
    let proj = Projection::new(source);
    let mut out = Vec::new();
    for (id, pair) in proj.marks.iter().enumerate() {
        let mut idx = proj
            .chars
            .iter()
            .enumerate()
            .filter(|(_, c)| c.seg.is_some_and(|s| proj.segments[s].mark == Some(id)))
            .map(|(i, _)| i);
        let Some(first) = idx.next() else { continue };
        let last = idx.next_back().unwrap_or(first);
        out.push(Mark {
            quote: proj.quote(first..last + 1),
            range: first..last + 1,
            plus: pair.plus,
            color: pair.color.clone(),
        });
    }
    out
}

/// Highlight `selection` as a new quote, or remove the quotes covering it if
/// it is already fully highlighted. Mirrors [`crate::toggle`] without touching
/// the source.
pub fn toggle_quotes(source: &str, existing: &[Quote], selection: &str) -> Result<QuoteEdit, Error> {
    let needle = normalize(selection);
    if needle.is_empty() {
        return Err(Error::EmptySelection);
    }
    let proj = Projection::new(source);
    let occs = proj.occurrences(source, &needle);
    if occs.is_empty() {
        return Err(Error::NotFound);
    }
    let ambiguous = occs.len() > 1;
    let ranges: Vec<Option<Range<usize>>> = existing.iter().map(|q| proj.resolve(q)).collect();
    let touching = |r: &Range<usize>| -> Vec<usize> {
        (0..existing.len())
            .filter(|&i| ranges[i].as_ref().is_some_and(|e| e.start < r.end && r.start < e.end))
            .collect()
    };
    let covered = |r: &Range<usize>| {
        proj.chars[r.clone()].iter().enumerate().all(|(off, c)| {
            let i = r.start + off;
            c.seg.is_none() || c.ch == ' ' || ranges.iter().flatten().any(|e| e.contains(&i))
        })
    };

    match occs.iter().find(|r| !covered(r)) {
        Some(r) => {
            let remove = touching(r);
            let mut range = r.clone();
            for e in remove.iter().filter_map(|&i| ranges[i].as_ref()) {
                range.start = range.start.min(e.start);
                range.end = range.end.max(e.end);
            }
            Ok(QuoteEdit { action: Action::Added, remove, add: Some(proj.quote(range)), ambiguous })
        }
        None => Ok(QuoteEdit { action: Action::Removed, remove: touching(&occs[0]), add: None, ambiguous }),
    }
}

impl Projection {
    fn quote(&self, range: Range<usize>) -> Quote {
        let collect = |r: Range<usize>| self.chars[r].iter().map(|c| c.ch).collect::<String>();
        Quote {
            exact: collect(range.clone()),
            prefix: collect(range.start.saturating_sub(CONTEXT)..range.start),
            suffix: collect(range.end..(range.end + CONTEXT).min(self.chars.len())),
        }
    }

    /// The occurrence of `quote.exact` whose surroundings match best.
    fn resolve(&self, quote: &Quote) -> Option<Range<usize>> {
        if quote.exact.is_empty() {
            return None;
        }
        let (text, starts) = self.text();
        let len = quote.exact.chars().count();
        let before: Vec<char> = quote.prefix.chars().rev().collect();
        let after: Vec<char> = quote.suffix.chars().collect();
        let score = |r: &Range<usize>| {
            let left = self.chars[..r.start].iter().rev().zip(&before).take_while(|(c, q)| c.ch == **q).count();
            let right = self.chars[r.end..].iter().zip(&after).take_while(|(c, q)| c.ch == **q).count();
            left + right
        };
        let mut best: Option<(usize, Range<usize>)> = None;
        for r in text
            .match_indices(quote.exact.as_str())
            .filter_map(|(b, _)| starts.binary_search(&b).ok())
            .map(|i| i..i + len)
        {
            let s = score(&r);
            if best.as_ref().is_none_or(|(b, _)| s > *b) {
                best = Some((s, r));
            }
        }
        best.map(|(_, r)| r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{toggle, Syntax};

    fn add(src: &str, quotes: &mut Vec<Quote>, sel: &str) -> QuoteEdit {
        let edit = toggle_quotes(src, quotes, sel).expect("toggle");
        let mut i = 0;
        quotes.retain(|_| {
            i += 1;
            !edit.remove.contains(&(i - 1))
        });
        quotes.extend(edit.add.clone());
        edit
    }

    #[test]
    fn quote_has_context() {
        let src = "Uno **dos** tres cuatro";
        let mut q = Vec::new();
        add(src, &mut q, "dos tres");
        assert_eq!(q, [Quote { exact: "dos tres".into(), prefix: "Uno ".into(), suffix: " cuatro".into() }]);
        assert_eq!(resolve(src, &q[0]), Some(4..12));
    }

    #[test]
    fn toggles_like_marks() {
        let src = "Hola mundo feliz";
        let mut q = Vec::new();
        assert_eq!(add(src, &mut q, "mundo").action, Action::Added);
        let edit = add(src, &mut q, "mundo");
        assert_eq!(edit.action, Action::Removed);
        assert!(q.is_empty());
    }

    #[test]
    fn overlap_merges() {
        let src = "Hola mundo feliz";
        let mut q = Vec::new();
        add(src, &mut q, "mundo");
        let edit = add(src, &mut q, "mundo feliz");
        assert_eq!(edit.action, Action::Added);
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].exact, "mundo feliz");
    }

    #[test]
    fn duplicates_use_context() {
        let src = "gato y gato";
        let mut q = Vec::new();
        add(src, &mut q, "gato");
        let edit = add(src, &mut q, "gato");
        assert!(edit.ambiguous);
        assert_eq!(edit.action, Action::Added);
        assert_eq!(resolve(src, &q[0]), Some(0..4));
        assert_eq!(resolve(src, &q[1]), Some(7..11));
        assert_eq!(add(src, &mut q, "gato").action, Action::Removed);
    }

    #[test]
    fn survives_edits_elsewhere() {
        let mut q = Vec::new();
        add("Hola mundo feliz", &mut q, "mundo");
        assert_eq!(resolve("# Nuevo\n\nHola mundo feliz", &q[0]), Some(11..16));
        assert_eq!(resolve("Adiós", &q[0]), None);
    }

    #[test]
    fn marks_lists_both_syntaxes() {
        let src = r#"a <mark class="hl hl-pink">b</mark> c ++d e++ f"#;
        let m = marks(src);
        assert_eq!(m.len(), 2);
        assert_eq!((m[0].quote.exact.as_str(), m[0].plus, m[0].color.as_deref()), ("b", false, Some("pink")));
        assert_eq!((m[1].quote.exact.as_str(), m[1].plus), ("d e", true));
    }

    #[test]
    fn plain_indices_survive_marking() {
        let src = "Hola mundo feliz";
        let out = toggle(src, "mundo", Syntax::Plus).unwrap();
        assert_eq!(out.range, 5..10);
        assert_eq!(marks(&out.source)[0].range, out.range);
        assert_eq!(resolve(&out.source, &marks(&out.source)[0].quote), Some(5..10));
    }
}
