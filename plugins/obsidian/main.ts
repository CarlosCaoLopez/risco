// Shows Subrayador's highlights, stored beside each note in a hidden JSONL,
// in reading view and Live Preview. Also draws Orca's `++…++` marks in color.

import { StateEffect, type Range as CmRange } from "@codemirror/state";
import { Decoration, type DecorationSet, EditorView, ViewPlugin, type ViewUpdate } from "@codemirror/view";
import * as fs from "fs";
import { editorInfoField, editorLivePreviewField, FileSystemAdapter, MarkdownView, Plugin, TFile } from "obsidian";
import { paint, parseRecords, sidecarPath, unpaint, type HlRecord } from "../shared/paint";

const refresh = StateEffect.define<null>();
const DEFAULT_COLOR = "yellow";

export default class Subrayador extends Plugin {
  private records = new Map<string, HlRecord[]>();
  private watchers = new Map<string, fs.FSWatcher>();
  private repaintTimer: number | null = null;

  async onload() {
    this.registerMarkdownPostProcessor(() => this.scheduleRepaint());
    this.registerEditorExtension(this.livePreview());

    this.registerEvent(this.app.workspace.on("file-open", (file) => file && void this.track(file)));
    this.registerEvent(
      this.app.vault.on("rename", async (file, oldPath) => {
        if (!(file instanceof TFile) || file.extension !== "md") return;
        const adapter = this.app.vault.adapter;
        const from = sidecarPath(oldPath);
        if (await adapter.exists(from)) await adapter.rename(from, sidecarPath(file.path));
        this.records.delete(oldPath);
        await this.track(file);
      }),
    );
    this.app.workspace.onLayoutReady(() => {
      const file = this.app.workspace.getActiveFile();
      if (file) void this.track(file);
    });
  }

  onunload() {
    for (const w of this.watchers.values()) w.close();
    this.watchers.clear();
  }

  /** Load a note's records and watch its folder for sidecar changes. */
  private async track(file: TFile) {
    await this.loadRecords(file.path);
    const adapter = this.app.vault.adapter;
    if (!(adapter instanceof FileSystemAdapter)) return;
    const dir = file.parent?.path ?? "/";
    if (this.watchers.has(dir)) return;
    // Dotfiles never reach the vault's own events, so watch the disk.
    const watcher = fs.watch(adapter.getFullPath(dir === "/" ? "" : dir), (_, name) => {
      const match = name && /^\.(.+)\.hl\.jsonl$/.exec(name.toString());
      if (!match) return;
      const note = dir === "/" ? match[1] : `${dir}/${match[1]}`;
      if (this.records.has(note)) void this.loadRecords(note);
    });
    this.watchers.set(dir, watcher);
  }

  private async loadRecords(note: string) {
    let text = "";
    try {
      text = await this.app.vault.adapter.read(sidecarPath(note));
    } catch {
      // No sidecar yet: nothing highlighted.
    }
    this.records.set(note, parseRecords(text));
    this.scheduleRepaint();
    this.app.workspace.iterateAllLeaves((leaf) => {
      const view = leaf.view;
      if (!(view instanceof MarkdownView) || view.file?.path !== note) return;
      const cm = (view.editor as unknown as { cm?: EditorView }).cm;
      cm?.dispatch({ effects: refresh.of(null) });
    });
  }

  private scheduleRepaint() {
    if (this.repaintTimer !== null) window.clearTimeout(this.repaintTimer);
    this.repaintTimer = window.setTimeout(() => {
      this.repaintTimer = null;
      this.app.workspace.iterateAllLeaves((leaf) => {
        const view = leaf.view;
        if (!(view instanceof MarkdownView) || view.getMode() !== "preview" || !view.file) return;
        const root = view.previewMode.containerEl;
        unpaint(root);
        paint(root, this.records.get(view.file.path) ?? []);
      });
    }, 60);
  }

  /** Live Preview: color records and `++` marks in the source, hiding the `++`. */
  private livePreview() {
    const plugin = this;
    return ViewPlugin.fromClass(
      class {
        decorations: DecorationSet;
        constructor(view: EditorView) {
          this.decorations = this.build(view);
        }
        update(u: ViewUpdate) {
          if (u.docChanged || u.selectionSet || u.viewportChanged || u.transactions.some((t) => t.effects.some((e) => e.is(refresh)))) {
            this.decorations = this.build(u.view);
          }
        }
        build(view: EditorView): DecorationSet {
          const state = view.state;
          if (!state.field(editorLivePreviewField, false)) return Decoration.none;
          const file = state.field(editorInfoField, false)?.file;
          if (!file) return Decoration.none;
          const records = plugin.records.get(file.path) ?? [];
          const doc = state.doc.toString();
          const sel = state.selection.main;
          const out: CmRange<Decoration>[] = [];
          const mark = (color: string) => Decoration.mark({ class: `hl hl-${color}` });

          const inline = records.filter((r) => r.inline);
          for (const m of doc.matchAll(/\+\+((?:(?!\n\s*\n)[^])+?)\+\+/g)) {
            const from = m.index!;
            const to = from + m[0].length;
            const key = m[1].replace(/\s+/g, " ").trim();
            const color = inline.find((r) => r.exact.trim() === key)?.color ?? DEFAULT_COLOR;
            out.push(mark(color).range(from + 2, to - 2));
            // Show the `++` while the cursor is on the mark, like other syntax.
            if (sel.to < from || sel.from > to) {
              out.push(Decoration.replace({}).range(from, from + 2), Decoration.replace({}).range(to - 2, to));
            }
          }
          for (const r of records) {
            if (r.inline) continue;
            const at = locate(doc, r);
            if (at) out.push(mark(r.color).range(at[0], at[1]));
          }
          return Decoration.set(out, true);
        }
      },
      { decorations: (v) => v.decorations },
    );
  }
}

/** Find a record in raw source, letting whitespace and Markdown syntax vary a little. */
function locate(doc: string, r: HlRecord): [number, number] | null {
  const words = r.exact.split(/\s+/).filter(Boolean).map((w) => w.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
  if (!words.length) return null;
  const re = new RegExp(words.join("[\\s*_~`]+"), "g");
  let best: [number, number] | null = null;
  let bestScore = -1;
  for (const m of doc.matchAll(re)) {
    const from = m.index!;
    const to = from + m[0].length;
    const before = doc.slice(Math.max(0, from - r.prefix.length), from);
    const after = doc.slice(to, to + r.suffix.length);
    let score = 0;
    while (score < before.length && before[before.length - 1 - score] === r.prefix[r.prefix.length - 1 - score]) score++;
    let right = 0;
    while (right < after.length && after[right] === r.suffix[right]) right++;
    if (score + right > bestScore) {
      bestScore = score + right;
      best = [from, to];
    }
  }
  return best;
}
