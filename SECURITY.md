# Security Policy

Risco runs a global keyboard and mouse hook and writes files next to your
notes, so we take security reports seriously, even though this is a small,
volunteer-run project.

## Supported versions

Only the latest release gets security fixes.

| Version | Supported |
|:--|:--|
| 0.1.x (latest) | ✅ |
| Older pre-releases (`-rc`) | ❌ |

## Reporting a vulnerability

**Please do not open a public issue, discussion or pull request for a
security problem.** Report it privately in one of these ways:

1. **GitHub Private Vulnerability Reporting** (preferred): go to
   [Security › Report a vulnerability](https://github.com/CarlosCaoLopez/risco/security/advisories/new).
2. **Email**: caolopezcarlos@gmail.com, with `[risco security]` in the subject.
   If you want to encrypt the report, ask for a GPG key in a first, short email.

A good report includes:

- The Risco version (the release you installed) and your OS.
- What an attacker can do, and what they need first (local access, a crafted note, a malicious vault…).
- Steps to reproduce, or a proof of concept.
- Any fix you have in mind.

For example:

> Risco 0.1.1 on Ubuntu 24.04. A note whose sidecar `.nota.md.hl.jsonl` contains
> a record with `"exact": "<img src=x onerror=…>"` runs script in the Obsidian
> reading view when the plugin paints it. Steps: …

## What happens next

| When | What |
|:--|:--|
| Within **72 hours** | We acknowledge your report. |
| Within **14 days** | We send a first assessment: whether we can reproduce it, how severe we think it is, and a plan. |
| Until it is fixed | We keep you updated at least every 14 days. |
| After the fix ships | We publish a GitHub Security Advisory and credit you, unless you prefer to stay anonymous. |

The maintainer is a student; the times above are commitments, but a fix for a
complex issue may take longer than the first reply. We will tell you if it does.

## Disclosure policy

We follow **coordinated disclosure**:

- Please give us a reasonable time to release a fix (we aim for 90 days at most)
  before you publish details.
- We will not open a public issue or describe the problem in commit messages
  until a fixed release is available.
- When the fix is out, we publish an advisory with a CVE if one applies, and
  note it in the *Security* section of [CHANGELOG.md](CHANGELOG.md).

## Scope and known limitations

Some of what Risco does is sensitive by design. These are known and documented,
not vulnerabilities in themselves, but bugs that make them worse are in scope:

- **Global input hook.** Risco watches keyboard and mouse events system-wide to
  recognise the modifier + drag gesture. On macOS this needs the Accessibility
  permission. It never records or sends keystrokes.
- **Clipboard.** On Windows and macOS, Risco reads the selection by simulating
  <kbd>Ctrl</kbd>/<kbd>⌘</kbd>+<kbd>C</kbd> and then restoring the clipboard.
  Clipboard managers may still see the copied text.
- **Reading other apps' state.** To find the open note, Risco reads window titles,
  Orca's SQLite profile, VS Code's recent folders and Obsidian's vault list.
- **Writing files.** Risco writes hidden sidecar files (`.note.md.hl.jsonl`) beside
  your notes, edits notes opened in Orca, and installs plugins into Obsidian vaults
  and VS Code.
- **No network.** Risco makes no network requests. A report showing that it does is in scope.
- **Unsigned Windows installer.** Windows builds are not code-signed yet.
- **Wayland** is not supported on Linux; Risco needs X11.

Out of scope: attacks that already require full control of the user's account,
and vulnerabilities in Obsidian, VS Code, Orca or the OS itself (report those upstream).

## EU Cyber Resilience Act

Risco is free and open source software, developed and distributed without
charge and outside of any commercial activity. Under the EU Cyber Resilience
Act (Regulation (EU) 2024/2847) it is therefore not placed on the market by a
*manufacturer*, and the project is not a commercial *open-source software
steward*. We still follow the practices the Act expects: this documented
vulnerability handling process, coordinated disclosure, security fixes for the
latest release, and automated dependency and code scanning in CI.

Risco is provided **without any warranty**, as stated in sections 15 and 16 of
the [GNU General Public License v3](LICENSE).
