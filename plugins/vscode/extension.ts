// Feeds each preview the highlights stored beside its note, and refreshes
// the preview when Risco rewrites them.

import * as fs from "fs";
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

  return {
    extendMarkdownIt(md: MarkdownIt) {
      // A trailing token that carries the note's records into the preview.
      md.core.ruler.push("risco", (state) => {
        const env = state.env as { currentDocument?: vscode.Uri } | undefined;
        const token = new state.Token("risco_data", "", 0);
        token.content = attr(records(env?.currentDocument));
        state.tokens.push(token);
      });
      md.renderer.rules.risco_data = (tokens, idx) =>
        `<div class="risco-data" hidden data-records="${tokens[idx].content}"></div>`;
      return md;
    },
  };
}

export function deactivate() {}
