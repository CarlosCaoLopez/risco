// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Find text the user selected in a *rendered* Markdown view inside the
//! Markdown *source*, and wrap it in highlight marks: `<mark class="hl hl-COLOR">`
//! tags or Orca's `++underline++`.
//!
//! The source is projected to the plain text a viewer would show (no `**`,
//! `[]()`, `#`…), with every plain character remembering the byte span it
//! came from. Selections are searched in that projection, and tags are only
//! ever inserted at text boundaries, so emphasis, links and blocks keep
//! rendering. Selecting text that is already highlighted removes the
//! highlight instead.
//!
//! Highlights can also live outside the source, as text quotes (see [`Quote`]).

mod anchor;

pub use anchor::{marks, resolve, toggle_quotes, Mark, Quote, QuoteEdit};

use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};
use std::collections::BTreeMap;
use std::ops::Range;

pub const COLORS: [&str; 5] = ["yellow", "pink", "green", "blue", "purple"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Added,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub source: String,
    pub action: Action,
    /// The selection appeared more than once; the first suitable one was used.
    pub ambiguous: bool,
    /// Number of marks added or removed.
    pub marks: usize,
    /// Plain-text characters the highlight covers (added) or covered (removed).
    /// Marks never change the plain text, so these indices hold before and after.
    pub range: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EmptySelection,
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptySelection => f.write_str("la selección está vacía"),
            Error::NotFound => f.write_str("no se encontró el texto en el archivo"),
        }
    }
}

impl std::error::Error for Error {}

/// How marks are written into the source.
#[derive(Debug, Clone, Copy)]
pub enum Syntax<'a> {
    /// `<mark class="hl hl-COLOR">…</mark>`
    Html { color: &'a str },
    /// `++…++`, the only highlight-like markup Orca's editor renders.
    Plus,
}

impl Syntax<'_> {
    fn open(&self) -> String {
        match self {
            Syntax::Html { color } => format!(r#"<mark class="hl hl-{color}">"#),
            Syntax::Plus => PLUS.into(),
        }
    }

    fn close(&self) -> &'static str {
        match self {
            Syntax::Html { .. } => CLOSE_TAG,
            Syntax::Plus => PLUS,
        }
    }
}

const CLOSE_TAG: &str = "</mark>";
const PLUS: &str = "++";

/// Whether `selection` can be found in `source`, as rendered text or as raw source.
pub fn contains(source: &str, selection: &str) -> bool {
    let needle = normalize(selection);
    if needle.is_empty() {
        return false;
    }
    let proj = Projection::new(source);
    !proj.occurrences(source, &needle).is_empty()
}

/// Whether every occurrence of `selection` is already highlighted, so
/// [`toggle`] would remove it.
pub fn marked(source: &str, selection: &str) -> bool {
    let needle = normalize(selection);
    let proj = Projection::new(source);
    let occs = proj.occurrences(source, &needle);
    !needle.is_empty() && !occs.is_empty() && occs.iter().all(|r| proj.fully_marked(r.clone()))
}

/// Remove every `<mark class="hl">` tag, keeping `++` marks.
pub fn strip_html_marks(source: &str) -> String {
    let proj = Projection::new(source);
    let ids: Vec<usize> = (0..proj.marks.len()).filter(|&i| !proj.marks[i].plus).collect();
    remove_marks(source, &proj, &ids)
}

/// Highlight `selection` in `source`, or remove the highlight if it is
/// already fully highlighted.
pub fn toggle(source: &str, selection: &str, syntax: Syntax) -> Result<Outcome, Error> {
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

    match occs.iter().find(|r| !proj.fully_marked((*r).clone())) {
        Some(range) => {
            let (source, marks, range) = add(source, &proj, range.clone(), syntax)?;
            Ok(Outcome { source, action: Action::Added, ambiguous, marks, range })
        }
        None => {
            let touched = proj.touched_marks(occs[0].clone());
            let marks = touched.len();
            Ok(Outcome {
                source: remove_marks(source, &proj, &touched),
                action: Action::Removed,
                ambiguous,
                marks,
                range: occs[0].clone(),
            })
        }
    }
}

/// Collapse every whitespace run to one space and trim.
fn normalize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if is_space(ch) {
            if !out.is_empty() && !out.ends_with(' ') {
                out.push(' ');
            }
        } else {
            out.push(ch);
        }
    }
    out.truncate(out.trim_end().len());
    out
}

fn is_space(ch: char) -> bool {
    ch.is_whitespace() || ch == '\u{a0}'
}

fn add(
    source: &str,
    proj: &Projection,
    range: Range<usize>,
    syntax: Syntax,
) -> Result<(String, usize, Range<usize>), Error> {
    let touched = proj.touched_marks(range.clone());
    if touched.is_empty() {
        let groups = proj.wrap_groups(range.clone());
        return Ok((insert_marks(source, &groups, syntax), groups.len(), range));
    }

    // Overlaps an existing highlight: merge it into the new one.
    let mut range = range;
    for (i, c) in proj.chars.iter().enumerate() {
        if c.seg.is_some_and(|s| proj.segments[s].mark.is_some_and(|m| touched.contains(&m))) {
            range.start = range.start.min(i);
            range.end = range.end.max(i + 1);
        }
    }
    let stripped = remove_marks(source, proj, &touched);
    let proj2 = Projection::new(&stripped);
    if proj2.chars.len() != proj.chars.len() {
        return Err(Error::NotFound);
    }
    let groups = proj2.wrap_groups(range.clone());
    Ok((insert_marks(&stripped, &groups, syntax), groups.len(), range))
}

fn insert_marks(source: &str, groups: &[Range<usize>], syntax: Syntax) -> String {
    let open = syntax.open();
    let mut out = source.to_string();
    for g in groups.iter().rev() {
        out.insert_str(g.end, syntax.close());
        out.insert_str(g.start, &open);
    }
    out
}

fn remove_marks(source: &str, proj: &Projection, ids: &[usize]) -> String {
    let mut cuts: Vec<Range<usize>> = ids
        .iter()
        .flat_map(|&id| [proj.marks[id].open.clone(), proj.marks[id].close.clone()])
        .filter(|r| !r.is_empty())
        .collect();
    cuts.sort_by_key(|r| r.start);
    let mut out = source.to_string();
    for r in cuts.iter().rev() {
        out.replace_range(r.clone(), "");
    }
    out
}

fn is_hl_open(html: &str) -> bool {
    let h = html.trim_start();
    h.starts_with("<mark") && (h.contains(r#"class="hl "#) || h.contains(r#"class="hl""#))
}

/// The `COLOR` of `<mark class="hl hl-COLOR">`.
fn hl_color(html: &str) -> Option<String> {
    let rest = &html[html.find(" hl-")? + 4..];
    let end = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '-')).unwrap_or(rest.len());
    (end > 0).then(|| rest[..end].to_string())
}

/// Byte offsets of paired `++` delimiters, mapped to whether each one opens.
///
/// Like Orca's tokenizer, a `++` opens a mark when a later `++` in the same
/// block closes it; a lone `++` (as in "C++") stays text.
fn plus_delimiters(source: &str, opts: Options) -> BTreeMap<usize, bool> {
    let mut out = BTreeMap::new();
    let mut open: Option<usize> = None;
    let mut code = 0usize;
    for (event, range) in Parser::new_ext(source, opts).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => code += 1,
            Event::End(TagEnd::CodeBlock) => code = code.saturating_sub(1),
            Event::Start(
                Tag::Emphasis
                | Tag::Strong
                | Tag::Strikethrough
                | Tag::Link { .. }
                | Tag::Image { .. },
            )
            | Event::End(
                TagEnd::Emphasis
                | TagEnd::Strong
                | TagEnd::Strikethrough
                | TagEnd::Link
                | TagEnd::Image,
            ) => {}
            Event::Start(_) | Event::End(_) => open = None,
            Event::Text(text) if code == 0 && source.get(range.clone()) == Some(text.as_ref()) => {
                let mut from = 0;
                while let Some(i) = text[from..].find(PLUS) {
                    let at = range.start + from + i;
                    match open {
                        // An empty `++++` closes nothing.
                        Some(o) if at > o + PLUS.len() => {
                            out.insert(o, true);
                            out.insert(at, false);
                            open = None;
                        }
                        Some(_) => {}
                        None => open = Some(at),
                    }
                    from += i + PLUS.len();
                }
            }
            _ => {}
        }
    }
    out
}

fn is_mark_close(html: &str) -> bool {
    html.trim().eq_ignore_ascii_case(CLOSE_TAG)
}

/// One plain character of the projection.
#[derive(Debug, Clone)]
struct PChar {
    ch: char,
    /// `None` for block separators and text that must not be wrapped (code blocks).
    seg: Option<usize>,
    /// Source bytes this character stands for.
    span: Range<usize>,
}

/// A run of characters coming from one parser event.
#[derive(Debug, Clone)]
struct Segment {
    /// Enclosing inline containers, outermost first.
    chain: Vec<usize>,
    /// Innermost enclosing highlight.
    mark: Option<usize>,
}

/// An inline element (emphasis, link…) whose delimiters can be wrapped whole.
#[derive(Debug, Clone)]
struct Container {
    range: Range<usize>,
    first: usize,
    last: usize,
}

#[derive(Debug, Clone)]
struct MarkPair {
    open: Range<usize>,
    close: Range<usize>,
    /// `++…++` rather than a `<mark>` tag.
    plus: bool,
    /// The `hl-COLOR` of a `<mark>` tag.
    color: Option<String>,
}

#[derive(Debug, Default)]
struct Projection {
    chars: Vec<PChar>,
    segments: Vec<Segment>,
    containers: Vec<Container>,
    marks: Vec<MarkPair>,
}

impl Projection {
    fn new(source: &str) -> Self {
        let opts = Options::ENABLE_TABLES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_WIKILINKS
            | Options::ENABLE_MATH;
        let plus = plus_delimiters(source, opts);
        let mut p = Projection::default();
        let mut stack: Vec<usize> = Vec::new();
        let mut mark_stack: Vec<usize> = Vec::new();
        // Containers whose inside must be wrapped whole, e.g. `[[wikilinks]]`.
        let mut atomic: Option<(Range<usize>, usize)> = None;
        let mut image_depth = 0usize;
        let mut in_code_block = false;

        for (event, range) in Parser::new_ext(source, opts).into_offset_iter() {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link { .. } => {
                        if let Tag::Link { link_type, .. } = tag {
                            if atomic.is_none()
                                && matches!(
                                    link_type,
                                    LinkType::WikiLink { .. }
                                        | LinkType::Autolink
                                        | LinkType::Email
                                )
                            {
                                atomic = Some((range.clone(), stack.len()));
                            }
                        }
                        p.containers.push(Container { range, first: usize::MAX, last: 0 });
                        stack.push(p.containers.len() - 1);
                    }
                    Tag::Image { .. } => image_depth += 1,
                    Tag::CodeBlock(_) => {
                        in_code_block = true;
                        p.separator();
                    }
                    _ => p.separator(),
                },
                Event::End(tag) => match tag {
                    TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link => {
                        stack.pop();
                        if atomic.as_ref().is_some_and(|(_, depth)| *depth == stack.len()) {
                            atomic = None;
                        }
                    }
                    TagEnd::Image => image_depth = image_depth.saturating_sub(1),
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        p.separator();
                    }
                    _ => p.separator(),
                },
                Event::Text(text) => {
                    if image_depth > 0 {
                        continue;
                    }
                    if in_code_block {
                        for ch in text.chars() {
                            p.push(ch, None, range.clone());
                        }
                        continue;
                    }
                    let mut seg = p.segment(&stack, &mark_stack);
                    if let Some((ar, _)) = &atomic {
                        for ch in text.chars() {
                            p.push(ch, Some(seg), ar.clone());
                        }
                    } else if source.get(range.clone()) == Some(text.as_ref()) {
                        // An escaped char (`\*`) arrives without its backslash.
                        let escaped = text.starts_with(|c: char| c.is_ascii_punctuation())
                            && source[..range.start].ends_with('\\');
                        let mut skip_to = 0;
                        for (off, ch) in text.char_indices() {
                            let at = range.start + off;
                            if at < skip_to {
                                continue;
                            }
                            if let Some(&opens) = plus.get(&at) {
                                let delim = at..at + PLUS.len();
                                if opens {
                                    p.marks.push(MarkPair {
                                        open: delim,
                                        close: 0..0,
                                        plus: true,
                                        color: None,
                                    });
                                    mark_stack.push(p.marks.len() - 1);
                                } else if let Some(id) = mark_stack.pop() {
                                    p.marks[id].close = delim;
                                }
                                seg = p.segment(&stack, &mark_stack);
                                skip_to = at + PLUS.len();
                                continue;
                            }
                            let from = if off == 0 && escaped { at - 1 } else { at };
                            p.push(ch, Some(seg), from..at + ch.len_utf8());
                        }
                    } else {
                        // Escapes or entities: the text differs from its source.
                        for ch in text.chars() {
                            p.push(ch, Some(seg), range.clone());
                        }
                    }
                }
                Event::Code(text) | Event::InlineMath(text) => {
                    if image_depth > 0 {
                        continue;
                    }
                    let seg = p.segment(&stack, &mark_stack);
                    let span = atomic.as_ref().map_or(range.clone(), |(ar, _)| ar.clone());
                    for ch in text.chars() {
                        p.push(ch, Some(seg), span.clone());
                    }
                }
                Event::DisplayMath(text) => {
                    p.separator();
                    for ch in text.chars() {
                        p.push(ch, None, range.clone());
                    }
                    p.separator();
                }
                Event::SoftBreak | Event::HardBreak => {
                    let seg = p.segment(&stack, &mark_stack);
                    p.push(' ', Some(seg), range);
                }
                Event::InlineHtml(html) => {
                    if is_hl_open(&html) {
                        let color = hl_color(&html);
                        p.marks.push(MarkPair { open: range, close: 0..0, plus: false, color });
                        mark_stack.push(p.marks.len() - 1);
                    } else if is_mark_close(&html) {
                        if let Some(id) = mark_stack.pop() {
                            p.marks[id].close = range;
                        }
                    }
                }
                Event::Html(_) | Event::Rule => p.separator(),
                _ => {}
            }
        }

        while p.chars.last().is_some_and(|c| c.ch == ' ') {
            p.chars.pop();
        }
        for (i, c) in p.chars.iter().enumerate() {
            if let Some(s) = c.seg {
                for &cid in &p.segments[s].chain {
                    let cont = &mut p.containers[cid];
                    cont.first = cont.first.min(i);
                    cont.last = cont.last.max(i);
                }
            }
        }
        p
    }

    fn segment(&mut self, stack: &[usize], marks: &[usize]) -> usize {
        self.segments.push(Segment { chain: stack.to_vec(), mark: marks.last().copied() });
        self.segments.len() - 1
    }

    fn separator(&mut self) {
        self.push('\n', None, 0..0);
    }

    fn push(&mut self, ch: char, seg: Option<usize>, span: Range<usize>) {
        if !is_space(ch) {
            self.chars.push(PChar { ch, seg, span });
            return;
        }
        match self.chars.last_mut() {
            None => {}
            Some(last) if last.ch == ' ' => {
                if seg.is_none() {
                    last.seg = None;
                } else if last.seg == seg && last.span.end == span.start {
                    last.span.end = span.end;
                }
            }
            Some(_) => self.chars.push(PChar { ch: ' ', seg, span }),
        }
    }

    fn text(&self) -> (String, Vec<usize>) {
        let mut s = String::with_capacity(self.chars.len());
        let mut starts = Vec::with_capacity(self.chars.len());
        for c in &self.chars {
            starts.push(s.len());
            s.push(c.ch);
        }
        (s, starts)
    }

    /// Ranges of plain-character indices where `needle` (already normalized) occurs.
    fn occurrences(&self, source: &str, needle: &str) -> Vec<Range<usize>> {
        let (text, starts) = self.text();
        let len = needle.chars().count();
        let found: Vec<Range<usize>> = text
            .match_indices(needle)
            .filter_map(|(b, _)| starts.binary_search(&b).ok())
            .map(|i| i..i + len)
            .filter(|r| self.has_text(r.clone()))
            .collect();
        if !found.is_empty() {
            return found;
        }
        // Live-preview editors hand us raw source, `**` and all.
        self.raw_occurrences(source, needle)
    }

    fn raw_occurrences(&self, source: &str, needle: &str) -> Vec<Range<usize>> {
        let mut text = String::new();
        let mut bytes: Vec<usize> = Vec::new(); // source offset of each text byte
        for (at, ch) in source.char_indices() {
            if is_space(ch) {
                if text.is_empty() || text.ends_with(' ') {
                    continue;
                }
                text.push(' ');
                bytes.push(at);
            } else {
                text.push(ch);
                bytes.extend(std::iter::repeat_n(at, ch.len_utf8()));
            }
        }
        text.match_indices(needle)
            .filter_map(|(b, m)| {
                let start = bytes[b];
                let last = bytes[b + m.len() - 1];
                let end = last + source[last..].chars().next().map_or(1, char::len_utf8);
                let idx: Vec<usize> = self
                    .chars
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.seg.is_some() && c.span.start >= start && c.span.end <= end)
                    .map(|(i, _)| i)
                    .collect();
                Some(*idx.first()?..*idx.last()? + 1)
            })
            .filter(|r| self.has_text(r.clone()))
            .collect()
    }

    fn text_chars(&self, range: Range<usize>) -> impl Iterator<Item = &PChar> {
        self.chars[range].iter().filter(|c| c.seg.is_some() && c.ch != ' ')
    }

    fn has_text(&self, range: Range<usize>) -> bool {
        self.text_chars(range).next().is_some()
    }

    fn fully_marked(&self, range: Range<usize>) -> bool {
        self.text_chars(range).all(|c| self.segments[c.seg.unwrap()].mark.is_some())
    }

    fn touched_marks(&self, range: Range<usize>) -> Vec<usize> {
        let mut ids: Vec<usize> =
            self.text_chars(range).filter_map(|c| self.segments[c.seg.unwrap()].mark).collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    /// Source ranges to wrap so that plain characters `range` end up highlighted.
    fn wrap_groups(&self, range: Range<usize>) -> Vec<Range<usize>> {
        let (a, b) = (range.start, range.end);
        let mut groups: Vec<(usize, usize)> = Vec::new(); // first..=last plain index
        for i in a..b {
            let c = &self.chars[i];
            if c.seg.is_none() {
                continue;
            }
            match groups.last_mut() {
                Some((_, last))
                    if *last + 1 == i && {
                        let p = &self.chars[*last];
                        p.seg.is_some() && (p.span.end == c.span.start || p.span == c.span)
                    } =>
                {
                    *last = i
                }
                _ => groups.push((i, i)),
            }
        }

        let mut spans: Vec<Range<usize>> = groups
            .into_iter()
            .filter_map(|(first, last)| {
                // Spaces between groups stay inside so neighbours can merge;
                // a group that is only space adds nothing.
                if self.chars[first..=last].iter().all(|c| c.ch == ' ') {
                    return None;
                }
                let start =
                    self.expand(first, a, b).map_or(self.chars[first].span.start, |c| c.start);
                let end = self.expand(last, a, b).map_or(self.chars[last].span.end, |c| c.end);
                Some(start..end)
            })
            .collect();

        spans.sort_by_key(|r| r.start);
        let mut merged: Vec<Range<usize>> = Vec::new();
        for r in spans {
            match merged.last_mut() {
                Some(m) if r.start <= m.end => m.end = m.end.max(r.end),
                _ => merged.push(r),
            }
        }
        merged
    }

    /// Outermost inline container around char `i` that the selection covers whole.
    fn expand(&self, i: usize, a: usize, b: usize) -> Option<Range<usize>> {
        let seg = self.chars[i].seg?;
        self.segments[seg]
            .chain
            .iter()
            .map(|&cid| &self.containers[cid])
            .find(|c| c.first >= a && c.last < b)
            .map(|c| c.range.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const Y: Syntax = Syntax::Html { color: "yellow" };

    fn hl(src: &str, sel: &str) -> String {
        toggle(src, sel, Y).expect("toggle").source
    }

    #[test]
    fn plain_word() {
        assert_eq!(
            hl("Hola mundo feliz", "mundo"),
            r#"Hola <mark class="hl hl-yellow">mundo</mark> feliz"#
        );
    }

    #[test]
    fn whole_strong_is_wrapped_outside() {
        assert_eq!(
            hl("uno **dos** tres", "uno dos tres"),
            r#"<mark class="hl hl-yellow">uno **dos** tres</mark>"#
        );
    }

    #[test]
    fn partial_strong_splits_marks() {
        assert_eq!(
            hl("uno **dos tres**", "uno dos"),
            r#"<mark class="hl hl-yellow">uno </mark>**<mark class="hl hl-yellow">dos</mark> tres**"#
        );
    }

    #[test]
    fn link_text_and_url_untouched() {
        assert_eq!(
            hl("ver [la guía](https://x.org/a) ya", "la guía"),
            r#"ver <mark class="hl hl-yellow">[la guía](https://x.org/a)</mark> ya"#
        );
        assert_eq!(
            hl("ver [la guía](https://x.org/a) ya", "guía"),
            r#"ver [la <mark class="hl hl-yellow">guía</mark>](https://x.org/a) ya"#
        );
    }

    #[test]
    fn wikilink_is_atomic() {
        assert_eq!(
            hl("mira [[Mi nota]] hoy", "nota"),
            r#"mira <mark class="hl hl-yellow">[[Mi nota]]</mark> hoy"#
        );
    }

    #[test]
    fn across_paragraphs_one_mark_each() {
        let out = hl("# Título\n\nPrimer párrafo.\n\nSegundo párrafo.", "párrafo. Segundo");
        assert_eq!(
            out,
            "# Título\n\nPrimer <mark class=\"hl hl-yellow\">párrafo.</mark>\n\n<mark class=\"hl hl-yellow\">Segundo</mark> párrafo."
        );
    }

    #[test]
    fn soft_break_stays_in_one_mark() {
        assert_eq!(
            hl("línea uno\nlínea dos", "uno línea"),
            "línea <mark class=\"hl hl-yellow\">uno\nlínea</mark> dos"
        );
    }

    #[test]
    fn list_items() {
        assert_eq!(
            hl("- manzana roja\n- pera verde\n", "roja pera"),
            "- manzana <mark class=\"hl hl-yellow\">roja</mark>\n- <mark class=\"hl hl-yellow\">pera</mark> verde\n"
        );
    }

    #[test]
    fn code_block_is_never_wrapped() {
        let src = "antes\n\n```\nlet x = 1;\n```\n\ndespués";
        assert_eq!(toggle(src, "let x", Y), Err(Error::NotFound));
        assert_eq!(
            hl(src, "antes let x = 1; después"),
            "<mark class=\"hl hl-yellow\">antes</mark>\n\n```\nlet x = 1;\n```\n\n<mark class=\"hl hl-yellow\">después</mark>"
        );
    }

    #[test]
    fn inline_code_wrapped_whole() {
        assert_eq!(
            hl("usa `cargo test` ya", "cargo"),
            r#"usa <mark class="hl hl-yellow">`cargo test`</mark> ya"#
        );
    }

    #[test]
    fn toggle_removes_existing() {
        let once = hl("Hola mundo feliz", "mundo");
        let out = toggle(&once, "mundo", Y).unwrap();
        assert_eq!(out.action, Action::Removed);
        assert_eq!(out.source, "Hola mundo feliz");
    }

    #[test]
    fn overlapping_merges_and_recolors() {
        let once = hl("Hola mundo feliz", "mundo");
        let out = toggle(&once, "mundo feliz", Syntax::Html { color: "pink" }).unwrap();
        assert_eq!(out.action, Action::Added);
        assert_eq!(out.source, r#"Hola <mark class="hl hl-pink">mundo feliz</mark>"#);
    }

    #[test]
    fn duplicates_pick_first_unmarked() {
        let once = hl("gato y gato", "gato");
        assert_eq!(once, r#"<mark class="hl hl-yellow">gato</mark> y gato"#);
        let out = toggle(&once, "gato", Y).unwrap();
        assert!(out.ambiguous);
        assert_eq!(
            out.source,
            r#"<mark class="hl hl-yellow">gato</mark> y <mark class="hl hl-yellow">gato</mark>"#
        );
        assert_eq!(toggle(&out.source, "gato", Y).unwrap().action, Action::Removed);
    }

    #[test]
    fn raw_source_selection_from_live_preview() {
        assert_eq!(
            hl("uno **dos** tres", "**dos** tres"),
            r#"uno <mark class="hl hl-yellow">**dos** tres</mark>"#
        );
    }

    #[test]
    fn whitespace_and_unicode() {
        assert_eq!(
            hl("El  niño\tcomió   ñoquis", "niño comió"),
            "El  <mark class=\"hl hl-yellow\">niño\tcomió</mark>   ñoquis"
        );
    }

    #[test]
    fn escapes_wrap_whole_text_event() {
        let out = hl(r"precio 5\*3 hoy", "5*3");
        assert!(out.contains("<mark"), "{out}");
        assert!(out.contains(r"5\*3"), "{out}");
        assert_eq!(toggle(&out, "5*3", Y).unwrap().source, r"precio 5\*3 hoy");
    }

    fn plus(src: &str, sel: &str) -> String {
        toggle(src, sel, Syntax::Plus).expect("toggle").source
    }

    #[test]
    fn plus_marks() {
        assert_eq!(plus("Hola mundo feliz", "mundo"), "Hola ++mundo++ feliz");
        assert_eq!(plus("uno **dos tres**", "uno dos"), "++uno ++**++dos++ tres**");
        assert_eq!(plus("uno **dos** tres", "uno dos tres"), "++uno **dos** tres++");
    }

    #[test]
    fn plus_toggles_off() {
        let out = toggle("Hola ++mundo++ feliz", "mundo", Syntax::Plus).unwrap();
        assert_eq!(out.action, Action::Removed);
        assert_eq!(out.source, "Hola mundo feliz");
        assert!(marked("Hola ++mundo++ feliz", "mundo"));
        assert!(!marked("Hola ++mundo++ feliz", "Hola"));
    }

    #[test]
    fn plus_merges_with_existing() {
        assert_eq!(plus("Hola ++mundo++ feliz", "mundo feliz"), "Hola ++mundo feliz++");
    }

    #[test]
    fn lone_plus_is_text() {
        assert_eq!(plus("uso C++ a diario", "a diario"), "uso C++ ++a diario++");
        assert!(contains("uso C++ a diario", "C++ a"));
        let code = "```\nx ++ y ++ z\n```";
        assert!(marks(code).is_empty());
    }

    #[test]
    fn strip_keeps_plus() {
        let src = r#"<mark class="hl hl-blue">a</mark> ++b++"#;
        assert_eq!(strip_html_marks(src), "a ++b++");
    }

    #[test]
    fn not_found_and_empty() {
        assert_eq!(toggle("abc", "zzz", Y), Err(Error::NotFound));
        assert_eq!(toggle("abc", "  \n", Y), Err(Error::EmptySelection));
        assert!(contains("uno **dos**", "uno dos"));
    }

    #[test]
    fn table_cells() {
        let src = "| a | b |\n|---|---|\n| uno | dos |\n";
        assert_eq!(
            hl(src, "dos"),
            "| a | b |\n|---|---|\n| uno | <mark class=\"hl hl-yellow\">dos</mark> |\n"
        );
    }
}
