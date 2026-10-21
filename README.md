<p align="center">
  <img src="src-tauri/icons/128x128@2x.png" width="128" alt="Risco icon">
</p>

<h1 align="center">Risco</h1>

<h3 align="center">Highlight any Markdown. Without touching it.</h3>

<p align="center">
  Free and open source. For Linux, Windows and macOS.
  <br>
  <a href="#build-from-source">Build from source&nbsp;&rsaquo;</a>
</p>

<br>

## Hold. Drag. Highlighted.

Hold <kbd>Ctrl</kbd> (<kbd>⌥</kbd> on macOS) and drag over any text in Obsidian,
the VS Code preview or Orca. It sounds like a felt-tip pen,
and the highlight stays.

<!-- demo.gif -->

<br>

## A gesture for everything.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + drag | Highlight it. |
| <kbd>Ctrl</kbd> + drag over a highlight | Take it off. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | Yellow, pink, green, blue or purple. |
| *Deshacer último* in the tray | Undo the last one. |

<br>

## Your Markdown. Untouched.

Highlights live in a hidden file next to each note, not in the note itself.
The Obsidian plugin and the VS Code extension paint them in place.

Orca can't load plugins, so there the highlight goes into the note as
`++text++`, the one markup it renders.

If the editor has unsaved changes, nothing is written.

<br>

## Private by design.

No account. No network. No analytics.
Risco runs entirely on your computer, and your notes never leave it.

<br>

## Tech Specs

| | |
|:--|:--|
| **Platforms** | Linux (X11), Windows, macOS |
| **Viewers** | Obsidian, VS Code, Orca |
| **Built with** | Rust, Tauri 2 and TypeScript |
| **Network access** | None |
| **Price** | Free |

<br>

## Build from source

```sh
npm install
npm run tauri build
```

On Linux you also need the usual Tauri/WebKitGTK packages, `libxtst` and an
appindicator library.

Then open *Ajustes…* from the tray and install the plugin once per Obsidian
vault and once in VS Code.
