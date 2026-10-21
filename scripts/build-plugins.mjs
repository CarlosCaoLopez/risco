// SPDX-FileCopyrightText: 2026 Carlos Cao López <caolopezcarlos@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

// Bundle the VS Code extension and the Obsidian plugin into plugins/dist/,
// where the app embeds them for "Instalar".

import { createVSIX } from "@vscode/vsce";
import { build } from "esbuild";
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const css = "assets/liquid-highlight.css";
const out = "plugins/dist";
const common = { bundle: true, target: "es2022", logLevel: "warning" };

mkdirSync(`${out}/vscode`, { recursive: true });
mkdirSync(`${out}/obsidian`, { recursive: true });

await Promise.all([
  build({
    ...common,
    entryPoints: ["plugins/vscode/extension.ts"],
    outfile: `${out}/vscode/extension.js`,
    platform: "node",
    format: "cjs",
    external: ["vscode"],
  }),
  build({
    ...common,
    entryPoints: ["plugins/vscode/preview.ts"],
    outfile: `${out}/vscode/preview.js`,
    platform: "browser",
    format: "iife",
  }),
  build({
    ...common,
    entryPoints: ["plugins/obsidian/main.ts"],
    outfile: `${out}/obsidian/main.js`,
    platform: "node",
    format: "cjs",
    external: ["obsidian", "electron", "@codemirror/*"],
  }),
]);

// Both plugins take the app's version, so the app can tell when the
// installed copy is out of date.
const { version } = JSON.parse(readFileSync("src-tauri/tauri.conf.json", "utf8"));
const withVersion = (from, to) => {
  const json = JSON.parse(readFileSync(from, "utf8"));
  writeFileSync(to, JSON.stringify({ ...json, version }, null, 2) + "\n");
};

withVersion("plugins/vscode/package.json", `${out}/vscode/package.json`);
copyFileSync(css, `${out}/vscode/liquid-highlight.css`);
copyFileSync("LICENSE", `${out}/vscode/LICENSE`);
withVersion("plugins/obsidian/manifest.json", `${out}/obsidian/manifest.json`);
copyFileSync(css, `${out}/obsidian/styles.css`);

// The app installs the extension with `code --install-extension`.
await createVSIX({
  cwd: `${out}/vscode`,
  packagePath: resolve(`${out}/risco.vsix`),
  dependencies: false,
  allowMissingRepository: true,
});
