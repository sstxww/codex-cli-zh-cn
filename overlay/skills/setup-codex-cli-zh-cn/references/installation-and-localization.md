# Installation and localization reference

## Component mapping

| Owner choice | Action |
| --- | --- |
| Chinese UI only | Install the separate `codex-zh`; do not add profile switching |
| Isolated homes only | Use `LH-03/codex-cli-home-switcher`; keep official UI |
| Both | Install independently; combine at process launch |
| Another language | Read `LOCALIZATION_PLAYBOOK.md`; create a different binary/locale |
| Manual only | Give Release or source-build instructions; run nothing |

## Safe inspection

Check versions, command paths, OS/architecture, and existence of directories. Avoid reading API keys, tokens, full authentication/configuration, sessions, logs, or user prompts.

## Verification

- `codex --version` still works.
- `codex-zh --version` identifies the localized build.
- A live interactive TUI opens when practical.
- Canonical slash commands remain English.
- The expected inherited `CODEX_HOME` is used.
- No official binary or Codex home was copied, renamed, merged, or deleted.
- Report the exact uninstaller or directory/PATH rollback.
