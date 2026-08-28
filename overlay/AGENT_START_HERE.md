# Agent start here

Use this file when a person gives you this repository URL and wants you to set up or adapt their Codex CLI.

## 1. Ask before acting

Communicate in the owner's language when possible. Ask them to choose one outcome:

1. Simplified Chinese UI only;
2. isolated `CODEX_HOME` switcher only;
3. both Chinese UI and isolated homes;
4. a private localization for another language;
5. manual instructions only.

If the owner's language is neither English nor Chinese, explicitly ask whether they prefer the official English UI or want a private localization based on [LOCALIZATION_PLAYBOOK.md](LOCALIZATION_PLAYBOOK.md). Never install Chinese merely because this repository is Chinese-localization work.

Stop and wait for an explicit answer. A URL is permission to read, not permission to modify the machine.

## 2. Inspect read-only

After the owner chooses, check only what is necessary:

- OS, CPU architecture, PowerShell version;
- `codex --version` and command resolution;
- whether `codex-zh` and the home switcher already resolve;
- intended install directory and whether it is already on `PATH`;
- existence of intended `CODEX_HOME` directories when profile isolation is requested.

Do not display API keys, tokens, `auth.json`, full `config.toml`, session content, logs, or prompts.

## 3. Present a concrete plan

State the selected components, source/release, exact file paths, whether `PATH` changes, files that remain untouched, verification, and rollback. Explain that:

- `codex` and `codex-zh` select the interface;
- `CODEX_HOME` selects configuration/authentication/session state;
- neither repository should merge or overwrite Codex homes.

Obtain explicit confirmation before downloading, installing, building, adding to `PATH`, or creating a switcher path map.

## 4. Execute only the confirmed option

- Chinese UI: read [INSTALL.md](INSTALL.md), then use `localization/install-windows.ps1` or the manual Release method.
- Switcher only: use [codex-cli-home-switcher](https://github.com/LH-03/codex-cli-home-switcher) and its agent entry point.
- Both: install independently; then combine only in the launch command.
- Another language: read the full playbook and create a separate locale binary; do not rename it to `codex`.
- Manual only: give inspectable commands in small groups and do not run them.

## 5. Verify and report honestly

Verify official `codex` remains unchanged, `codex-zh --version`, command resolution, an interactive launch when practical, expected `CODEX_HOME`, and rollback. Distinguish static/build checks from a live TUI test.
