// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Paint sidecar highlights onto a rendered Markdown page.
//
// The page text is flattened the way `mdmark` projects a source (whitespace
// runs and block edges become one space), so the quotes it stores can be found
// again here. `++…++` pairs inside one block, the marks Orca writes, are drawn
// as highlights too and lose their `++`.

/** One line of a `.NOTE.md.hl.jsonl` sidecar (`store::Record`). */
export interface HlRecord {
  id: string;
  exact: string;
  prefix: string;
  suffix: string;
  color: string;
  inline?: boolean;
}

const BLOCKS = new Set(["P", "LI", "H1", "H2", "H3", "H4", "H5", "H6", "TD", "TH", "BLOCKQUOTE", "PRE", "TR", "DIV", "UL", "OL", "TABLE"]);
const SKIP = new Set(["SCRIPT", "STYLE", "TEXTAREA"]);
const PLUS = "++";
const DEFAULT_COLOR = "yellow";

/** A flattened character: where it lives, or null for a synthetic space. */
interface Slot {
  node: Text | null;
  offset: number;
  block: number;
  code: boolean;
}

interface Flat {
  text: string;
  slots: Slot[];
}

function flatten(root: HTMLElement): Flat {
  const slots: Slot[] = [];
  let text = "";
  let block = 0;
  const space = () => {
    if (text.length && !text.endsWith(" ")) {
      text += " ";
      slots.push({ node: null, offset: 0, block, code: false });
    }
  };
  const walk = (el: Node, code: boolean) => {
    for (const child of Array.from(el.childNodes)) {
      if (child.nodeType === Node.TEXT_NODE) {
        const t = child as Text;
        const data = t.data;
        for (let i = 0; i < data.length; i++) {
          const ch = data[i];
          if (/\s/.test(ch)) {
            if (text.length && !text.endsWith(" ")) {
              text += " ";
              slots.push({ node: t, offset: i, block, code });
            }
          } else {
            text += ch;
            slots.push({ node: t, offset: i, block, code });
          }
        }
      } else if (child.nodeType === Node.ELEMENT_NODE) {
        const e = child as HTMLElement;
        if (SKIP.has(e.tagName) || e.hidden) continue;
        if (e.tagName === "BR") {
          space();
          continue;
        }
        const isBlock = BLOCKS.has(e.tagName);
        if (isBlock) {
          space();
          block++;
        }
        walk(e, code || e.tagName === "PRE");
        if (isBlock) {
          space();
          block++;
        }
      }
    }
  };
  walk(root, false);
  return { text, slots };
}

/** `[open, close]` flat offsets of paired `++`, like `mdmark::plus_delimiters`. */
function plusPairs(flat: Flat): [number, number][] {
  const pairs: [number, number][] = [];
  let open: number | null = null;
  let from = 0;
  for (;;) {
    const at = flat.text.indexOf(PLUS, from);
    if (at < 0) break;
    from = at + PLUS.length;
    const s = flat.slots[at];
    const s2 = flat.slots[at + 1];
    // Both `+` must be real text from one node, outside code.
    if (!s.node || s.code || s2.node !== s.node || s2.offset !== s.offset + 1) continue;
    if (open !== null && flat.slots[open].block !== s.block) open = null;
    if (open === null) open = at;
    else if (at > open + PLUS.length) {
      pairs.push([open, at]);
      open = null;
    }
  }
  return pairs;
}

/** Flat offsets whose characters are left out of quotes (the `++` themselves). */
function cleanText(flat: Flat, pairs: [number, number][]): { text: string; map: number[] } {
  const drop = new Set<number>();
  for (const [o, c] of pairs) [o, o + 1, c, c + 1].forEach((i) => drop.add(i));
  let text = "";
  const map: number[] = [];
  for (let i = 0; i < flat.text.length; i++) {
    if (drop.has(i)) continue;
    text += flat.text[i];
    map.push(i);
  }
  return { text, map };
}

/** Best occurrence of `rec.exact` in `text`, scored by matching context. */
function resolve(text: string, rec: HlRecord): [number, number] | null {
  if (!rec.exact) return null;
  let best: [number, number] | null = null;
  let bestScore = -1;
  for (let at = text.indexOf(rec.exact); at >= 0; at = text.indexOf(rec.exact, at + 1)) {
    const end = at + rec.exact.length;
    let left = 0;
    while (left < rec.prefix.length && at - left - 1 >= 0 && text[at - left - 1] === rec.prefix[rec.prefix.length - left - 1]) left++;
    let right = 0;
    while (right < rec.suffix.length && end + right < text.length && text[end + right] === rec.suffix[right]) right++;
    if (left + right > bestScore) {
      bestScore = left + right;
      best = [at, end];
    }
  }
  return best;
}

type Op = { kind: "wrap"; start: number; end: number; color: string; key: string | null } | { kind: "drop"; start: number; end: number };

/** Character ranges of `[start, end)` grouped by text node, last first. */
function pieces(flat: Flat, start: number, end: number): { node: Text; from: number; to: number }[] {
  const out: { node: Text; from: number; to: number }[] = [];
  for (let i = start; i < end; i++) {
    const s = flat.slots[i];
    if (!s.node || s.code) continue;
    const last = out[out.length - 1];
    if (last && last.node === s.node) last.to = s.offset + 1;
    else out.push({ node: s.node, from: s.offset, to: s.offset + 1 });
  }
  return out.reverse();
}

function wrap(node: Text, from: number, to: number, color: string, key: string | null) {
  if (to < node.length) node.splitText(to);
  const mid = from > 0 ? node.splitText(from) : node;
  if (!mid.data.trim()) return;
  const mark = document.createElement("mark");
  mark.className = `hl hl-${color}`;
  mark.dataset.hl = key === null ? "record" : "plus";
  if (key !== null) mark.dataset.hlKey = key;
  mid.parentNode!.insertBefore(mark, mid);
  mark.appendChild(mid);
}

/** Highlight `records` and any `++` marks under `root`. */
export function paint(root: HTMLElement, records: HlRecord[]) {
  const flat = flatten(root);
  const pairs = plusPairs(flat);
  const clean = cleanText(flat, pairs);
  const inline = records.filter((r) => r.inline);
  const colorOf = (key: string) => inline.find((r) => r.exact.trim() === key)?.color ?? DEFAULT_COLOR;
  const ops: Op[] = [];

  // `++` marks painted earlier: their colors may have changed.
  for (const m of Array.from(root.querySelectorAll<HTMLElement>("mark[data-hl=plus]"))) {
    m.className = `hl hl-${colorOf(m.dataset.hlKey ?? "")}`;
  }
  for (const [o, c] of pairs) {
    const key = flat.text.slice(o + PLUS.length, c).trim();
    ops.push({ kind: "wrap", start: o + PLUS.length, end: c, color: colorOf(key), key });
    ops.push({ kind: "drop", start: o, end: o + PLUS.length }, { kind: "drop", start: c, end: c + PLUS.length });
  }
  for (const rec of records) {
    if (rec.inline) continue;
    const at = resolve(clean.text, rec);
    if (!at) continue;
    ops.push({ kind: "wrap", start: clean.map[at[0]], end: clean.map[at[1] - 1] + 1, color: rec.color, key: null });
  }

  // Last first, so earlier offsets stay valid as nodes split.
  ops.sort((a, b) => b.start - a.start);
  for (const op of ops) {
    for (const p of pieces(flat, op.start, op.end)) {
      if (op.kind === "drop") p.node.deleteData(p.from, p.to - p.from);
      else wrap(p.node, p.from, p.to, op.color, op.key);
    }
  }
}

/** Undo `paint` for sidecar records. `++` marks stay, but get `records`' colors again. */
export function unpaint(root: HTMLElement) {
  for (const mark of Array.from(root.querySelectorAll<HTMLElement>("mark[data-hl=record]"))) {
    mark.replaceWith(...Array.from(mark.childNodes));
  }
  root.normalize();
}

/** Parse a sidecar's JSONL, skipping bad lines. */
export function parseRecords(jsonl: string): HlRecord[] {
  const out: HlRecord[] = [];
  for (const line of jsonl.split("\n")) {
    if (!line.trim()) continue;
    try {
      out.push(JSON.parse(line) as HlRecord);
    } catch {
      // A half-written or hand-edited line: ignore it.
    }
  }
  return out;
}

/** `dir/.name.md.hl.jsonl` for `dir/name.md` (`store::sidecar`). */
export function sidecarPath(note: string): string {
  const slash = Math.max(note.lastIndexOf("/"), note.lastIndexOf("\\"));
  return `${note.slice(0, slash + 1)}.${note.slice(slash + 1)}.hl.jsonl`;
}
