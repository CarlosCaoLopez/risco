# Changelog

All notable changes to Risco are documented here. Each entry explains why a
change was made, not just what moved.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The interface, tray menu, notifications and error messages are translated
  into 11 languages, and Settings has a language picker. Risco follows the
  system language by default, because most people who highlight notes do not
  write them in English.
- The README is translated into the same 10 languages, so people can find out
  what Risco does before installing it.
- Governance documents (contributing guide, security policy, code of conduct,
  governance, DCO), issue and pull request templates, and an architecture
  decision log, so outside contributors know how the project works and how
  to report problems safely.
- Continuous integration for every push and pull request: build, lint
  (`rustfmt`, `clippy`, `tsc`, REUSE), tests, CodeQL and OpenSSF Scorecard.
  Until now only releases were built, so breakage was found at release time.
- Dependabot keeps Cargo, npm and GitHub Actions dependencies up to date.

### Changed

- Risco is now licensed under the **GNU General Public License v3.0 or later**.
  The repository had no license before, which meant nobody could legally reuse
  or fork it. The Linux build already linked a GPL-3.0 dependency (`hyprland`,
  through `active-win-pos-rs`), so GPL is the honest license for what ships.
- Every source file carries an SPDX header and the repository is
  [REUSE](https://reuse.software/) compliant.
- Rust code is formatted with `rustfmt`, so style is checked by a tool rather
  than by review.
- GitHub Actions are pinned to full commit SHAs, so a moved tag in a
  third-party action cannot change what our release pipeline runs.

## [0.1.1] - 2026-10-21

### Added

- Each highlight is logged to stderr, which makes "nothing happened" reports
  much easier to debug.

### Changed

- Gestures only highlight when the viewer shows the rendered note (Obsidian's
  reading view, the VS Code Markdown preview). In an editor the selection and
  the rendered text do not line up, so highlighting there produced marks in
  the wrong place. The viewer plugins now report which view is active.

### Fixed

- The VS Code preview paints existing highlights as soon as it opens. VS Code
  fills the preview after extension scripts run, so the first paint used to
  find an empty page.

## [0.1.0-rc1] - 2026-10-21

First public pre-release.

### Added

- Hold a modifier (<kbd>Ctrl</kbd>, or <kbd>⌥</kbd> on macOS) and drag over
  text in Obsidian, the VS Code preview or Orca to highlight it; drag over a
  highlight to remove it. Five colours, picked with <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>1</kbd>–<kbd>5</kbd>.
- Highlights are stored as text quotes in a hidden sidecar next to each note
  (`.note.md.hl.jsonl`), so the Markdown itself stays untouched and other
  tools see clean files.
- In Orca, which cannot load plugins, highlights are written as `++text++`,
  the only markup its editor renders.
- An Obsidian plugin and a VS Code extension paint the sidecar highlights with
  a liquid-glass style. Risco installs them automatically on first launch.
- The sidecar follows its note when the note is renamed or moved.
- A felt-tip pen sound and an on-screen marker animation while dragging, so the
  gesture feels like using a real highlighter.
- A settings window, a tray menu with single-level undo, and autostart.
- Signed and notarized macOS builds, Windows and Linux installers, and a
  Homebrew cask.

### Changed

- The project was renamed from *Markdown Highlighter* to **Risco**. Settings
  from the old name are still read.

### Fixed

- Remapped modifier keys are honoured on X11.
- Overlays appear on the monitor where the drag happens.
- macOS asks for the Accessibility permission instead of silently ignoring gestures.

[Unreleased]: https://github.com/CarlosCaoLopez/risco/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/CarlosCaoLopez/risco/compare/v0.1.0-rc1...v0.1.1
[0.1.0-rc1]: https://github.com/CarlosCaoLopez/risco/releases/tag/v0.1.0-rc1
