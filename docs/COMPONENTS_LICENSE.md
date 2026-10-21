# Licensing of Risco and its components

Risco is licensed under the **GNU General Public License, version 3 or any
later version** (SPDX: `GPL-3.0-or-later`). The full text is in
[`LICENSE`](../LICENSE) and [`LICENSES/GPL-3.0-or-later.txt`](../LICENSES/GPL-3.0-or-later.txt).

This document explains why we chose it, how it fits with the licenses of
everything Risco depends on, and what it means for you if you redistribute
or modify Risco.

> This is an explanation for humans, not legal advice. When in doubt, the
> license text is what counts.

## Why GPL-3.0-or-later

**What ships already demands it.** On Linux, Risco reads the focused window
through [`active-win-pos-rs`](https://crates.io/crates/active-win-pos-rs),
which links the [`hyprland`](https://crates.io/crates/hyprland) crate. That
crate is GPL-3.0-or-later. A Rust binary is statically linked, so the Linux
build of Risco is a combined work that may only be distributed under the GPL.
We considered keeping the source under a permissive license and documenting
the exception, but one license for all of Risco is simpler for everyone to
understand.

**It protects users' freedom.** Risco runs a system-wide input hook and reads
your notes. Copyleft guarantees that anyone who receives a build of Risco,
or of a fork, can get its source and check what it does with that access.
That matters more for a tool like this than for a typical library.

**It is the FSF's recommended license** for applications, and it is
OSI-approved. Version 3 adds an explicit patent license (section 11), protection
against "tivoization" (section 6) and better compatibility with Apache-2.0
than version 2.

**"Or later"** lets the project, and anyone downstream, move to a future GPL
version published by the FSF without having to contact every contributor.

## Compatibility with dependencies

Risco's own code may be combined with code under any license that is
GPL-3.0-compatible. Every dependency in `Cargo.lock` and `package-lock.json` was
checked; they fall into the groups below.

### Rust crates (compiled into the app)

| License | Examples | Compatible with GPL-3.0? |
|:--|:--|:--|
| `MIT`, `MIT OR Apache-2.0` (≈ 550 crates) | tauri, serde, pulldown-cmark, rodio, rdev, x11 | ✅ Yes |
| `Apache-2.0` | tao, cpal, kdotool | ✅ Yes, with GPL **v3** (not v2) |
| `Apache-2.0 WITH LLVM-exception` | target-lexicon, rustix (also MIT) | ✅ Yes |
| `BSD-2-Clause`, `BSD-3-Clause` | brotli, alloc-stdlib, bindgen (build only) | ✅ Yes |
| `ISC` | inotify, libloading | ✅ Yes |
| `Zlib`, `BSL-1.0` | foldhash, zlib-rs, clipboard-win | ✅ Yes |
| `Unicode-3.0` | icu_* crates (Unicode data) | ✅ Yes |
| `Unlicense OR MIT`, `CC0-1.0` | memchr, walkdir, notify | ✅ Yes (public-domain dedications) |
| `MPL-2.0` | cssparser, selectors, option-ext | ✅ Yes: MPL-2.0 section 3.3 allows combination with the GPL |
| `MIT OR Apache-2.0 OR LGPL-2.1-or-later` | r-efi | ✅ Yes (we use it under MIT) |
| `GPL-3.0-or-later` | hyprland, hyprland-macros (Linux only) | ✅ Same license |

The bundled SQLite (via `rusqlite`'s `bundled` feature) is in the public
domain and imposes no conditions.

### npm packages (build tools, not shipped)

The npm dependencies are used to build the frontend and the viewer plugins.
Only the runtime packages `@tauri-apps/api` and `@tauri-apps/plugin-dialog`
(`Apache-2.0 OR MIT`) end up in the bundles. `obsidian`, `vscode` and
`@codemirror/*` are provided by the host application at runtime and are not bundled.

Build-only tools (TypeScript, Vite, esbuild, `@vscode/vsce`) are under MIT,
Apache-2.0, ISC, BSD, 0BSD, BlueOak-1.0.0 and Artistic-2.0. They are never
distributed with Risco, so their licenses place no conditions on it.
`@vscode/vsce-sign` is a proprietary Microsoft binary used only to package
the `.vsix` on the developer's machine.

### Assets and documentation

- Icons, CSS and the liquid-glass stylesheet are original work under GPL-3.0-or-later.
- [`CODE_OF_CONDUCT.md`](../CODE_OF_CONDUCT.md) is the Contributor Covenant 2.1,
  under [CC-BY-4.0](../LICENSES/CC-BY-4.0.txt).
- [`DCO`](../DCO) is the Developer Certificate of Origin 1.1, which may be copied
  verbatim but not changed ([`LicenseRef-DCO`](../LICENSES/LicenseRef-DCO.txt)).

Files that cannot carry a license header (JSON, lockfiles, images) are covered
by [`.reuse/dep5`](../.reuse/dep5). Run `uvx reuse lint` to check the whole repository.

### Third-party platforms

Risco installs plugins into Obsidian and VS Code and reads Orca's state. These
applications are not linked into Risco and are not covered by its license.
The plugins only use the public plugin APIs, so they can be GPL-licensed
regardless of the host application's license.

## Additional terms

Risco adds **no additional terms** under GPL-3.0 section 7. In particular,
there is no extra patent clause beyond section 11, and no trademark grant:
the name "Risco" and its icon identify this project, and forks should use a
different name to avoid confusion.

## What this means for you

You may use Risco for any purpose, including at work, without restriction.

If you **redistribute** Risco, modified or not (for example in a Linux package):

- Keep the copyright notices and the license text.
- Provide the complete corresponding source code, or a written offer for it,
  under the same license (section 6).
- If you modified it, say so and date your changes (section 5).
- Do not add restrictions, such as DRM or a proprietary license, on top.

If you only **modify it for yourself**, you have no obligations at all.

## No warranty

Risco is provided **"as is", without warranty of any kind**, express or
implied, including the warranties of merchantability and fitness for a
particular purpose. The entire risk as to the quality and performance of the
program is with you. See sections 15 and 16 of the license.
