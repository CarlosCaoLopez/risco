<p align="center">
  <img src="src-tauri/icons/128x128@2x.png" width="128" alt="Risco icon">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  English · <a href="docs/README.es.md">Español</a> · <a href="docs/README.gl.md">Galego</a> · <a href="docs/README.fr.md">Français</a> · <a href="docs/README.pt-BR.md">Português (Brasil)</a> · <a href="docs/README.ja.md">日本語</a> · <a href="docs/README.ko.md">한국어</a> · <a href="docs/README.zh-CN.md">简体中文</a> · <a href="docs/README.tr.md">Türkçe</a> · <a href="docs/README.az.md">Azərbaycanca</a> · <a href="docs/README.hi.md">हिन्दी</a>
</p>

<h3 align="center">Highlight any Markdown. Without touching it.</h3>

<p align="center">
  Free and open source. For Linux, Windows and macOS.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">Download&nbsp;&rsaquo;</a>
</p>

<br>

## Install

Download the installer for your system from the
[latest release](https://github.com/CarlosCaoLopez/risco/releases/latest):

| | |
|:--|:--|
| **macOS** | Open the `.dmg` and drag Risco to Applications. |
| **Windows** | Run the `-setup.exe`. |
| **Linux** | Install the `.deb` or `.rpm`, or run the `.AppImage`. |

Or install it with Homebrew:

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco is signed with a Developer ID and notarized by Apple, so it opens like
any other app. On first launch macOS asks for Accessibility access, which
Risco needs to see the gesture. Windows may show a SmartScreen warning
because the installer isn't signed yet: choose *More info › Run anyway*.

That's it. Risco lives in the tray, starts with your computer, and installs
its plugin in every Obsidian vault and in VS Code on its own.

<br>

## Hold. Drag. Highlighted.

Hold <kbd>Ctrl</kbd> (<kbd>⌥</kbd> on macOS) and drag over any text in
Obsidian's reading view, the VS Code preview or Orca. It sounds like a
felt-tip pen, and the highlight stays.

<!-- demo.gif -->

<br>

## Read first.

Highlights live on the rendered page, so switch to it before you drag:

| | |
|:--|:--|
| **Obsidian** | Reading view: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>), or the book icon at the top right of the note. |
| **VS Code** | Markdown preview: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>), or <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd> to open it beside the editor. Click the preview, then drag. |
| **Orca** | Nothing to switch: highlight right in the editor. |

In Obsidian's editing view or the VS Code editor, the gesture does nothing.

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

The app installs the viewer plugins itself the first time it runs.
