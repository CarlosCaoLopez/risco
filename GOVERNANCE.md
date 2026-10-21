# Governance

This document explains who makes decisions about Risco and how. It is meant to
be short and honest about the project's size: today Risco has a single
maintainer, and the rules below are written so they keep working when more
people join.

## Contact

| Role | Name | Contact |
|:--|:--|:--|
| Maintainer | Carlos Cao López ([@CarlosCaoLopez](https://github.com/CarlosCaoLopez)) | caolopezcarlos@gmail.com |

For security issues use [SECURITY.md](SECURITY.md); for conduct issues, the
private contact in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Roles

### Contributors

Anyone who opens an issue, reviews a pull request, improves a translation or
sends code. Contributors do not need any permission; they need to follow the
[Code of Conduct](CODE_OF_CONDUCT.md) and the [contribution guide](CONTRIBUTING.md).

### Maintainers

Maintainers have write access to the repository. They are responsible for:

- Reviewing and merging pull requests into `develop`.
- Triaging issues: labelling, asking for details, closing duplicates.
- Cutting releases: version bump, `CHANGELOG.md`, tag on `main`, signed installers.
- Handling security reports within the times promised in [SECURITY.md](SECURITY.md).
- Enforcing the Code of Conduct.
- Keeping CI, dependencies and the release secrets (Apple signing, Homebrew tap token) healthy.

Carlos Cao López currently holds all of these responsibilities.

## How decisions are made

### Day-to-day decisions

Any single maintainer may act alone on everyday work: merging a reviewed pull
request, fixing a bug, updating a dependency within its current major version,
improving docs or translations, triaging issues.

### Significant decisions

These need the agreement of **at least two maintainers**:

- Adding a new runtime dependency, or a new major version of one.
- Changing the sidecar format (`.note.md.hl.jsonl`) or the `++text++` convention,
  since viewer plugins and existing notes depend on them.
- Breaking changes to settings, behaviour or the plugins.
- Publishing a new release.
- Changing the license, this document or the Code of Conduct.

**While there is only one maintainer**, a significant decision is announced in
a GitHub issue labelled `decision` and stays open for comments for at least
**7 days** before it is carried out (security fixes are exempt). Releases follow
the normal Git Flow process instead.

### Consensus first, then a vote

Maintainers try to reach consensus in the open: in the issue or pull request
where the question came up. If consensus is not reached in reasonable time,
the decision is taken by **simple majority** of the maintainers. With an even
split, the status quo stays.

## Adding maintainers

New maintainers are added when the project needs them, for example when reviews
regularly wait too long, or a new viewer needs someone who knows it well. A
candidate is usually someone who has contributed good changes and reviews for
a while. Adding a maintainer requires a **unanimous** vote of the current
maintainers.

A maintainer who is inactive for a year, or who asks to step down, becomes an
emeritus maintainer and loses write access. They are welcome back.

## Conflict resolution

1. Discuss it in the relevant issue or pull request, focusing on the technical
   trade-offs. Most disagreements end here.
2. If that stalls, any participant can ask the maintainers to decide. The
   maintainers explain the decision and its reasoning in writing.
3. Conflicts about behaviour, rather than code, are handled privately under the
   [Code of Conduct](CODE_OF_CONDUCT.md).

## Communication

Project work is **open and transparent by default**. Plans, design discussions
and decisions happen in public GitHub issues and pull requests, so anyone can
follow and take part. The only private channels are for security reports and
Code of Conduct reports, which protect the people involved.
