<!-- Thanks for contributing! Pull requests go to `develop`, not `main`. -->

## Description

<!-- What does this change and why? Link the issue it closes.
Example: Paint highlights in the VS Code preview after it finishes loading.
VS Code fills the preview after extension scripts run, so the first paint
found an empty page. Closes #42. -->

## Type of change

- [ ] `feat`: new feature
- [ ] `fix`: bug fix
- [ ] `refactor` / `perf`: no change in behaviour
- [ ] `docs`: documentation only
- [ ] `build` / `ops` / `chore`: build, CI or housekeeping
- [ ] Breaking change (`!`): existing notes, sidecars, settings or plugins behave differently

## Testing done

<!-- What did you run, and how did you try it by hand?
Example:
- `cargo test --workspace` passes; added `vscode_preview_repaint` test.
- Manually on Ubuntu 24.04 (X11) with VS Code 1.98: opened the preview of
  examples/prueba.md, existing highlights now appear immediately.
-->

## Checklist

- [ ] My commits are signed off (`git commit -s`) per the [DCO](https://github.com/CarlosCaoLopez/risco/blob/main/DCO)
- [ ] Commit messages follow [Conventional Commits](https://github.com/CarlosCaoLopez/risco/blob/main/CONTRIBUTING.md#commit-conventions)
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace -- -D warnings` and `cargo test --workspace` pass
- [ ] `npm run build` passes
- [ ] New user-facing text is in every `locales/*.json`
- [ ] New files have SPDX headers (`uvx reuse lint` passes)
- [ ] Docs and `CHANGELOG.md` (*Unreleased*) are updated if behaviour changed
