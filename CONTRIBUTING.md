# Contributing to Risco

Thank you so much for taking the time to read this. It genuinely means a lot.
Risco is a small project, and the fact that you are here considering
contributing to it is something we do not take for granted.

This guide explains how to get a working build, what we expect from a change,
and what happens after you open a pull request. If anything here is unclear,
that is a bug in the guide: open a [question](https://github.com/CarlosCaoLopez/risco/issues/new?template=question.md)
and we will fix it.

- [Before you start](#before-you-start)
- [Setting up a development environment](#setting-up-a-development-environment)
- [Coding standards](#coding-standards)
- [Tests](#tests)
- [Commit conventions](#commit-conventions)
- [Branches](#branches)
- [Pull request process](#pull-request-process)
- [Review expectations](#review-expectations)
- [Definition of Done](#definition-of-done)

## Before you start

- **Small fixes** (typos, a clearer error message, a missing translation key):
  just open a pull request.
- **Anything bigger** (a new viewer, a new setting, a change to the sidecar
  format): please open a [feature request](https://github.com/CarlosCaoLopez/risco/issues/new?template=feature_request.md)
  first, so we can agree on the approach before you spend an evening on it.
- **Security problems** never go in a public issue. Follow [SECURITY.md](SECURITY.md).

Everyone taking part is expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Setting up a development environment

### 1. Fork and clone

Fork [CarlosCaoLopez/risco](https://github.com/CarlosCaoLopez/risco) on GitHub, then:

```sh
git clone https://github.com/<your-user>/risco.git
cd risco
git remote add upstream https://github.com/CarlosCaoLopez/risco.git
```

### 2. Install the toolchain

You need:

- **Rust** stable (install with [rustup](https://rustup.rs/)), with `rustfmt` and `clippy`.
- **Node.js 22** and npm.
- The [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your system.

On Debian/Ubuntu the system packages are:

```sh
sudo apt-get install -y libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev libxtst-dev libx11-dev libasound2-dev patchelf
```

`libxtst` is needed by the global input hook, and an appindicator library by the tray icon.

### 3. Create a descriptive branch

Branch from `develop` (see [Branches](#branches)) and name the branch after what it does:

```sh
git fetch upstream
git switch -c feature/typora-support upstream/develop
```

Good names: `feature/typora-support`, `fix/vscode-preview-repaint`, `docs/wayland-troubleshooting`.
Avoid names like `patch-1` or `my-changes`.

### 4. Build from a clean checkout

The Rust app embeds the bundled viewer plugins, so build them **before** any `cargo` command:

```sh
npm install
npm run build:plugins     # bundles plugins/ into plugins/dist/
cargo build --workspace
```

Then run the app with hot reload:

```sh
npm run tauri dev
```

### 5. Run the tests

```sh
cargo test --workspace    # all Rust tests
cargo test -p mdmark      # only the highlight engine, fast and with no Tauri deps
npm run build             # type-checks the frontend and both plugins
```

If all three pass on a clean checkout, your environment is ready.

## Coding standards

We follow the conventions each language's tooling already enforces, rather than
inventing our own:

- **Rust**: `cargo fmt` (configured in `rustfmt.toml`) and `cargo clippy --workspace -- -D warnings`
  must both be clean. 4-space indentation.
- **TypeScript, CSS, HTML, JSON**: 2-space indentation, double quotes, semicolons,
  matching the existing files. `tsc` must pass with the strict settings in `tsconfig.json`.
- [`.editorconfig`](.editorconfig) sets indentation and line endings for your editor.
- Every new source file starts with an SPDX header:

  ```rust
  // SPDX-FileCopyrightText: 2026 Your Name <you@example.com>
  //
  // SPDX-License-Identifier: GPL-3.0-or-later
  ```

  `uvx reuse annotate --copyright "Your Name <you@example.com>" --license GPL-3.0-or-later <file>`
  writes it for you, and `uvx reuse lint` checks the whole repo.

Comments explain *why*, not *what*. Keep them as sparse as the surrounding code.

### Project-specific rules

A few invariants span several files. CI does not catch all of them, so please check:

- **User-facing text** lives in `locales/<code>.json`. `en.json` is the reference;
  every other catalog must have the same keys and `{placeholders}` (`cargo test -p risco i18n::`
  checks this). Add a new key to all eleven files: an English placeholder is
  better than a missing key.
- **Highlight colors** are defined in `mdmark::COLORS`, `COLOR_KEYS` in `src-tauri/src/lib.rs`,
  the `color.*` locale keys, the `RGB` map in `src/overlay/overlay.ts` and `assets/liquid-highlight.css`.
- **The sidecar format** is `store::Record` in Rust and `HlRecord` in `plugins/shared/paint.ts`.
- **Settings**: a change to `Config` in `src-tauri/src/config.rs` needs the same change in
  the `Config` interface in `src/main.ts`.

## Tests

- Matching and marking logic belongs in `crates/mdmark`, with its tests next to it.
  It has no I/O, so tests are plain strings in and strings out:

  ```rust
  #[test]
  fn partial_strong_splits_marks() {
      assert_eq!(
          hl("uno **dos tres**", "uno dos"),
          r#"<mark class="hl hl-yellow">uno </mark>**<mark class="hl hl-yellow">dos</mark> tres**"#
      );
  }
  ```

- App modules (`locate`, `store`, `window`…) have unit tests at the bottom of each file.
  Run one module with `cargo test -p risco locate::`.
- A bug fix should come with a test that fails before the fix.
- There are no automated tests for the gesture, overlay or viewer plugins. If you
  touch those, say in the pull request how you tried it by hand (OS, viewer, steps).

## Commit conventions

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<optional scope>): <description>

<optional body>

<optional footer>
```

| Type | Use it for |
|:--|:--|
| `feat` | A new feature the user can see |
| `fix` | A bug fix the user can see |
| `refactor` | Restructuring with no change in behaviour |
| `perf` | A performance improvement |
| `style` | Formatting or whitespace only |
| `test` | Adding or fixing tests |
| `docs` | Documentation only |
| `build` | Build system, dependencies, version bumps |
| `ops` | CI/CD, release and deployment |
| `chore` | Housekeeping (`.gitignore`, repo setup…) |

Scopes name the part of the project you touched: `mdmark`, `app`, `ui`, `overlay`,
`orca`, `vscode`, `obsidian`, `plugins`, `sound`, `readme`. Never use an issue
number as a scope; put `Closes #12` in the footer instead.

Rules:

- The description is in the imperative, lower case and has no trailing period:
  `fix(vscode): paint highlights when the preview loads`, not `Fixed the preview.`
- The body says **why** the change was needed. The diff already says what changed.
- A breaking change gets a `!` and a footer:

  ```
  feat(app)!: store sidecars in a .risco folder

  BREAKING CHANGE: highlights saved by 0.1.x are migrated on first launch.
  ```

- Each logical change is its own commit. A pull request that adds a feature and
  reformats an unrelated file should be two commits.

### Sign-off (DCO)

Every commit must be signed off, certifying the [Developer Certificate of Origin](DCO):
that you wrote the change, or otherwise have the right to submit it under the
project's license.

```sh
git commit -s -m "fix(orca): read the active tab after a restart"
```

This adds `Signed-off-by: Your Name <you@example.com>` to the message. Forgot it?
`git commit --amend -s` fixes the last commit, and `git rebase --signoff develop`
fixes a whole branch.

We also encourage signing commits with GPG; [docs/GPG_KEY.md](docs/GPG_KEY.md) explains how.

## Branches

We use Git Flow:

| Branch | Purpose | Branches from | Merges into |
|:--|:--|:--|:--|
| `main` | Released code only. Every release is a tag such as `v0.2.0`. | — | — |
| `develop` | Integration branch for the next release. | `main` | — |
| `feature/*`, `fix/*`, `docs/*` | One change each. **Your pull requests go here.** | `develop` | `develop` |
| `release/*` | Final fixes and version bump before a release. | `develop` | `main` and `develop` |
| `hotfix/*` | Urgent fix for a released version. | `main` | `main` and `develop` |

Nobody pushes directly to `main` or `develop`; everything goes through a pull request.

## Pull request process

1. Rebase on the latest `develop` so the branch applies cleanly.
2. Run `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` and `npm run build`.
3. Open the pull request against **`develop`** and fill in the template.
4. CI (build, lint, tests, REUSE and security checks) must pass.
5. At least **one maintainer approval** is required to merge.
6. We usually squash or rebase-merge, keeping your commit messages, so please keep them tidy.

## Review expectations

Let's be honest about timing: Risco is maintained by one person, a university
student, in their spare time. That means:

- A first reply on an issue or pull request usually comes within **a week**, but
  during exam periods it can take **several weeks**. A friendly ping after two
  weeks is welcome and not rude.
- Small, focused pull requests are reviewed much faster than large ones.
- A review may ask for changes. That is about the code, never about you, and the
  goal is always to get your change merged.
- If a pull request goes quiet for more than a month after changes were
  requested, we may close it. You can reopen it whenever you have time.

## Definition of Done

A change is ready to merge when:

- [ ] It does what the issue or pull request description says, and nothing unrelated.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace -- -D warnings` and `cargo test --workspace` pass.
- [ ] `npm run build` passes (frontend and plugins type-check).
- [ ] New logic has tests, and a bug fix has a regression test.
- [ ] New user-facing text is in all `locales/*.json` files.
- [ ] New files carry SPDX headers and `uvx reuse lint` passes.
- [ ] Documentation (README, `CHANGELOG.md` under *Unreleased*) is updated when behaviour changes.
- [ ] Commits follow Conventional Commits and are signed off.
- [ ] A maintainer has approved it.

Thank you again for helping make Risco better.
