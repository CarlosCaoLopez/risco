// Feeds each preview the highlights stored beside its note, and refreshes
// the preview when Risco rewrites them.

import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import type MarkdownIt from "markdown-it";
import * as vscode from "vscode";
import { parseRecords, sidecarPath } from "../shared/paint";

/** HTML attribute-safe JSON. */
function attr(value: unknown): string {
  return JSON.stringify(value).replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;");
}

function records(doc: vscode.Uri | undefined) {
  if (!doc || doc.scheme !== "file") return [];
  try {
    return parseRecords(fs.readFileSync(sidecarPath(doc.fsPath), "utf8"));
  } catch {
    return [];
  }
}

export function activate(ctx: vscode.ExtensionContext) {
  const refresh = () => vscode.commands.executeCommand("markdown.preview.refresh");
  const watcher = vscode.workspace.createFileSystemWatcher("**/.*.md.hl.jsonl");
  watcher.onDidChange(refresh);
  watcher.onDidCreate(refresh);
  watcher.onDidDelete(refresh);

  // Keep a note's highlights with it when it is renamed or moved.
  const rename = vscode.workspace.onDidRenameFiles((e) => {
    for (const { oldUri, newUri } of e.files) {
      if (oldUri.scheme !== "file" || !/\.(md|markdown)$/i.test(oldUri.fsPath)) continue;
      const from = sidecarPath(oldUri.fsPath);
      if (fs.existsSync(from)) fs.renameSync(from, sidecarPath(newUri.fsPath));
    }
  });
  ctx.subscriptions.push(watcher, rename);

  // Tell the app whether the focused window shows a Markdown preview, the
  // only place highlights show. The title doesn't say in every language.
  const reportMode = () => {
    if (!vscode.window.state.focused) return;
    const input = vscode.window.tabGroups.activeTabGroup.activeTab?.input;
    // A preview panel (`markdown.preview`) or the preview editor
    // (`vscode.markdown.preview.editor`).
    const reading =
      (input instanceof vscode.TabInputWebview || input instanceof vscode.TabInputCustom) &&
      input.viewType.includes("markdown.preview");
    try {
      fs.writeFileSync(path.join(os.tmpdir(), "risco-vscode.json"), JSON.stringify({ reading }));
    } catch {
      // The app then lets every gesture through.
    }
  };
  reportMode();
  ctx.subscriptions.push(
    vscode.window.onDidChangeWindowState(reportMode),
    vscode.window.tabGroups.onDidChangeTabs(reportMode),
    vscode.window.tabGroups.onDidChangeTabGroups(reportMode),
  );

  return {
    extendMarkdownIt(md: MarkdownIt) {
      // A trailing token where the note's records go into the preview.
      md.core.ruler.push("risco", (state) => {
        state.tokens.push(new state.Token("risco_data", "", 0));
      });
      // VS Code caches tokens until the note itself changes, so the sidecar
      // is read here, on every render, not when tokenizing.
      md.renderer.rules.risco_data = (_tokens, _idx, _options, env: { currentDocument?: vscode.Uri } | undefined) =>
        `<div class="risco-data" hidden data-records="${attr(records(env?.currentDocument))}"></div>`;
      return md;
    },
  };
}

export function deactivate() {}
