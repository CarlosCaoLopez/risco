<p align="center">
  <img src="src-tauri/icons/128x128@2x.png" width="128" alt="Risco icon">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  English · <a href="docs/README.es.md">Español</a> · <a href="docs/README.gl.md">Galego</a> · <a href="docs/README.fr.md">Français</a> · <a href="docs/README.pt-BR.md">Português (Brasil)</a> · <a href="docs/README.ja.md">日本語</a> · <a href="docs/README.ko.md">한국어</a> · <a href="docs/README.zh-CN.md">简体中文</a> · <a href="docs/README.tr.md">Türkçe</a> · <a href="docs/README.az.md">Azərbaycanca</a> · <a href="docs/README.hi.md">हिन्दी</a>
</p>

<h3 align="center">Highlight any Markdown. Without touching it.</h3>

<p align="center">
  <a href="https://api.reuse.software/info/github.com/CarlosCaoLopez/risco"><img src="https://api.reuse.software/badge/github.com/CarlosCaoLopez/risco" alt="REUSE status"></a>
  <a href="https://www.bestpractices.dev/projects/0000"><img src="https://www.bestpractices.dev/projects/0000/badge" alt="OpenSSF Best Practices"></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/CarlosCaoLopez/risco"><img src="https://api.scorecard.dev/projects/github.com/CarlosCaoLopez/risco/badge" alt="OpenSSF Scorecard"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0--or--later-blue" alt="License: GPL-3.0-or-later"></a>
</p>

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
| *Undo last* in the tray | Undo the last one. |

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

## How it works

```text
 you: hold Ctrl + drag in a viewer
            │
            ▼
 ┌─────────────────────┐  Gesture::{Start, Move, End}  ┌──────────────────────┐
 │ input.rs            │──────────────────────────────▶│ highlight.rs         │
 │ global rdev hook    │                               │ sound + overlay      │
 └─────────────────────┘                               └──────────┬───────────┘
                                                                  │ on End
          ┌───────────────────────────┬───────────────────────────┼────────────────────────┐
          ▼                           ▼                           ▼                        ▼
 ┌──────────────────┐   ┌───────────────────────────┐   ┌──────────────────┐   ┌──────────────────────┐
 │ selection.rs     │   │ window.rs (+ orca.rs)     │   │ locate.rs        │   │ store.rs + mdmark    │
 │ selected text    │   │ app, file hint, dirty?,   │   │ which .md on     │   │ toggle the highlight │
 │ (X11 PRIMARY or  │   │ reading view?             │   │ disk holds it    │   │ as a text quote      │
 │  Ctrl/Cmd+C)     │   └───────────────────────────┘   └──────────────────┘   └──────────┬───────────┘
 └──────────────────┘                                                                     │ atomic write
                                                                                          ▼
                     notes/lecture.md          (untouched, except in Orca: ++text++)
                     notes/.lecture.md.hl.jsonl (one JSON record per highlight)
                                                                                          │ file watcher
                                                                                          ▼
                                        ┌────────────────────────────────────────────────────────┐
                                        │ Obsidian plugin · VS Code extension (plugins/)          │
                                        │ find each quote on the rendered page and paint <mark>  │
                                        └────────────────────────────────────────────────────────┘
```

The core matching logic lives in [`crates/mdmark`](crates/mdmark), a small
pure-Rust crate with no I/O: it projects Markdown source to the plain text a
viewer renders and maps a selection back to source byte ranges.

<br>

## Configuration

Open **Settings…** from the tray icon:

| Section | What it does |
|:--|:--|
| **Marker** | Highlight colour. Also <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> from anywhere. |
| **Gesture** | The modifier to hold while dragging: Ctrl, Alt, Shift or Meta. |
| **Effects** | Marker animation and felt-tip sound, with volume. |
| **Viewers** | Install or update the Obsidian plugin and VS Code extension, and move old `<mark>` tags out of notes. |
| **Note folders** | Extra folders to search for notes, on top of the detected Obsidian vaults, VS Code folders and Orca worktrees. |
| **Start with the system** | Launch Risco at login. |
| **Language** | Follow the system, or pick one of 11 languages. |

Settings are stored as JSON in your OS config directory:
`~/.config/risco/config.json` on Linux, `~/Library/Application Support/risco/config.json`
on macOS, `%APPDATA%\risco\config.json` on Windows.

<br>

## Compatibility

| Platform | Status | Notes |
|:--|:--|:--|
| Linux, X11 (Ubuntu 22.04+, Fedora, Arch) | ✅ Tested | Reads the PRIMARY selection. |
| Linux, Wayland | ❌ Not supported | Global input hooks are not allowed on Wayland. Log in to an X11 session. |
| Windows 10 / 11 | ✅ Supported | Reads the selection by simulating <kbd>Ctrl</kbd>+<kbd>C</kbd>, then restores the clipboard. |
| macOS 11 Big Sur or later (Intel and Apple silicon) | ✅ Supported | Needs Accessibility permission. Default modifier is <kbd>⌥</kbd>. |

| Viewer | Status | Where to highlight |
|:--|:--|:--|
| Obsidian | ✅ Plugin installed automatically | Reading view (highlights also show in Live Preview) |
| VS Code | ✅ Extension installed automatically | Markdown preview |
| Orca | ✅ No plugin needed | Editor (writes `++text++`) |

<br>

## Troubleshooting

| Symptom | Likely cause and fix |
|:--|:--|
| Nothing happens at all | Risco is paused: check **Enabled** in the tray menu. On macOS, grant Accessibility in *System Settings › Privacy & Security › Accessibility*, then restart Risco. On Linux, check that `echo $XDG_SESSION_TYPE` prints `x11`. |
| The sound plays but no highlight appears | You are in the editor, not the rendered view. Switch to Obsidian's reading view or the VS Code preview (see [Read first](#read-first)). |
| *"The file has unsaved changes"* | Risco never writes over unsaved edits. Save the note and highlight again. |
| *"I couldn't find that text in any .md in your folders"* | The note is outside every known folder. Add its folder in **Settings › Note folders**. |
| Highlights are saved but not shown | The viewer plugin is missing or disabled. In Obsidian, enable *Risco* under *Community plugins*; in VS Code, check the *Risco* extension is installed. Then use **Settings › Viewers** to reinstall. |
| Windows SmartScreen blocks the installer | The installer is not signed yet. Choose *More info › Run anyway*. |
| Highlight lands on the wrong occurrence | The selected text appears several times in the note. Select a slightly longer, unique passage. |

To see what Risco is doing, start it from a terminal: it logs each highlight
to stderr (`risco` on Linux, `/Applications/Risco.app/Contents/MacOS/risco` on macOS).

<br>

## Build from source

```sh
npm install
npm run tauri build
```

On Linux you also need the usual Tauri/WebKitGTK packages, `libxtst` and an
appindicator library.

The app installs the viewer plugins itself the first time it runs.

<br>

## Support

- **Questions and help**: open a [question](https://github.com/CarlosCaoLopez/risco/issues/new?template=question.md).
- **Bugs**: open a [bug report](https://github.com/CarlosCaoLopez/risco/issues/new?template=bug_report.md).
- **Security problems**: report them privately, as explained in [SECURITY.md](SECURITY.md).

<br>

## Contributing

Contributions are very welcome, from a fixed typo to a new viewer. Start with
[CONTRIBUTING.md](CONTRIBUTING.md); everyone involved follows the
[Code of Conduct](CODE_OF_CONDUCT.md). How decisions are made is described in
[GOVERNANCE.md](GOVERNANCE.md), and notable changes are listed in
[CHANGELOG.md](CHANGELOG.md).

<br>

## License

Risco is free software: you can redistribute it and/or modify it under the
terms of the [GNU General Public License](LICENSE) as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See [docs/COMPONENTS_LICENSE.md](docs/COMPONENTS_LICENSE.md)
for why, and for the licenses of its dependencies.

Risco is distributed in the hope that it will be useful, but **without any
warranty**; without even the implied warranty of merchantability or fitness
for a particular purpose.
