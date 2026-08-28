# Maintainer guide for the zh-CN fork

## Provenance

- Upstream: `https://github.com/openai/codex.git`
- Current upstream tag: `rust-v0.150.0-alpha.8`
- Original localization source: `https://github.com/LH-03/codex-cli-zh-cn`
- Locale entry point: `codex-rs/tui/src/i18n.rs`
- Separate binary: `codex-rs/tui/src/bin/codex-zh.rs`

The numbered upstream tag above is the compatibility baseline.

## Update workflow

```powershell
git remote add upstream https://github.com/openai/codex.git
git fetch upstream
git log --oneline --decorate --graph --max-count 20 upstream/main main
```

Create a maintenance branch. Review translated upstream files before rebasing or merging; do not resolve localization conflicts wholesale.

## Validation

From `codex-rs`, follow upstream `AGENTS.md` and run the focused sequence for TUI changes:

```powershell
just fmt
just test -p codex-tui
just fix -p codex-tui
cargo build --release -p codex-tui --bin codex-zh
```

Also run:

```powershell
git diff --check
cargo check -p codex-tui --bin codex-zh
```

Then perform a pseudo-terminal or live-terminal smoke test of `codex-zh`, `resume --last`, command search, CJK-width layouts, and the untouched official `codex`. Confirm that `codex-zh --version` still matches the declared upstream compatibility baseline for each public build.

## Release

Use `localization/package-windows.ps1` with a freshly validated Windows x64 release binary. Inspect the ZIP manifest before uploading. Release notes must include the upstream commit, test categories, known limitations, and rollback path.

Never commit a built executable to the Git repository. Publish it only as a GitHub Release asset.
