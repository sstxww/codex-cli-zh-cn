---
name: setup-codex-cli-zh-cn
description: Safely inspect and install the unofficial Simplified Chinese Codex CLI/TUI, combine it with isolated CODEX_HOME profiles, or guide a private localization for another language. Use when a user supplies this repository, asks for a Chinese Codex interface, or wants agent-managed localization. Always ask the language and component choices and obtain explicit confirmation before writing.
---

# Set up Codex CLI zh-CN

Use this workflow to preserve the official CLI and the owner's Codex state while adding only the confirmed localization components.

## Workflow

1. Read repository-root `AGENT_START_HERE.md`, `README.zh-CN.md`, and `INSTALL.md`.
2. Communicate in the owner's language when possible.
3. Ask them to choose: Chinese UI only, isolated homes only, both, another-language private localization, or manual-only instructions.
4. If their language is neither English nor Chinese, ask whether they prefer official English or a private localization. Never assume Chinese.
5. Stop for their answer.
6. Inspect only OS/architecture, PowerShell, command versions/resolution, intended paths, and directory existence. Do not expose credentials or provider configuration.
7. Read `references/installation-and-localization.md`. Show exact actions, untouched state, verification, and rollback.
8. Obtain explicit confirmation before downloading, building, installing, editing `PATH`, or adding a home switcher.
9. Execute only the confirmed option.
10. Verify official `codex`, separate `codex-zh`, selected `CODEX_HOME`, and rollback. Label static, build, and live checks accurately.

Keep canonical commands, flags, paths, IDs, model text, and raw tool output unchanged. Never merge Codex homes or replace a known-good provider configuration.
