# Codex CLI private-localization playbook

This document captures reusable experience from the zh-CN fork so people and AI agents can create a private localization for another language without replacing the official CLI.

## First conversation

Before writing code, ask the owner:

- target language and locale (for example `ja-JP`, `de-DE`, or `pt-BR`);
- whether they want a private build or a public repository;
- which TUI surfaces matter most;
- whether canonical commands should remain English (recommended);
- operating systems and terminal fonts to support;
- whether they also need isolated provider homes (a separate concern).

Do not infer the desired locale from this Chinese fork. If the owner only wants profile isolation, use the home switcher and leave the UI unchanged.

## Architecture that minimized risk

1. **Separate binary:** add a locale-specific launcher such as `codex-ja`, not a replacement `codex`.
2. **Explicit locale opt-in:** set one process-local locale variable before the runtime starts. Official invocations keep their original behavior.
3. **Small translation boundary:** route fixed TUI labels and descriptions through a localization module. Do not translate arbitrary dynamic text.
4. **Fallback to upstream English:** missing keys should display the original string, not fail the UI.
5. **No state migration:** reuse the selected `CODEX_HOME` as-is. UI language and provider state are independent.

## What should stay canonical

Keep these unchanged unless upstream itself changes them:

- slash command tokens such as `/model` and `/resume`;
- command-line flags and subcommand names;
- configuration keys and environment-variable names;
- file paths, URLs, model IDs, thread IDs, tool names, and connector IDs;
- model responses, shell output, diffs, logs, error payloads, and raw tool output.

Translated search aliases can help users discover an English slash command, but the inserted/executed token should stay canonical.

## Translation inventory

Classify candidate strings before translating:

| Class | Default decision |
| --- | --- |
| Fixed title, hint, button, menu description | Translate |
| Fixed onboarding and permission explanation | Translate carefully |
| Command token, flag, ID, path, model/tool output | Preserve |
| Mixed template with a dynamic value | Translate only the fixed fragments |
| Upstream diagnostic intended for copying/search | Usually preserve |

Start with a small, high-value surface and an inventory. Broad search-and-replace is fragile: the same English word can have different meanings, and dynamic strings can accidentally be corrupted.

## Unicode and terminal layout

Many terminals render CJK and some other scripts with widths that differ from byte or character counts. Use the project's display-width helpers (or `unicode-width`) for truncation, padding, cursor positions, and table columns. Test:

- narrow and wide terminals;
- punctuation and mixed ASCII/non-ASCII labels;
- command search and selection highlights;
- wrapping, ellipsis, and right-aligned shortcuts;
- common Windows Terminal and Unix terminal fonts for supported platforms.

## Tests that paid off

- unit tests for locale detection, fallback, exact templates, and command aliases;
- snapshots of core menus, permission prompts, status cards, and onboarding;
- checks that canonical command tokens remain unchanged;
- `git diff --check`, formatter, focused crate tests, and lint/fix;
- a real pseudo-terminal launch, not only `--help` or compilation;
- side-by-side smoke tests of official `codex` and the localized binary using the same test home.

Record whether each result is static, build-time, pseudo-terminal, or live-provider validation. Do not turn one category into a broader claim.

## Upstream maintenance

Keep the fork relationship and record the exact upstream commit. On every sync:

1. review upstream changes to translated files;
2. rebase or merge deliberately;
3. let new English strings fall back safely;
4. update the translation inventory instead of guessing blindly;
5. rerun width/snapshot/canonical-token tests;
6. publish a new separate binary;
7. keep the previous Release available for rollback.

## Privacy checklist for a public fork

- stage from a clean upstream clone;
- copy only intended source changes and public documentation;
- exclude local launchers containing usernames or drive paths;
- exclude configuration, authentication, sessions, logs, prompts, databases, build caches, and private handoff notes;
- scan staged content for home paths, email addresses, API-key patterns, access tokens, and unrelated project names;
- inspect the exact staged manifest and Release archive contents before pushing.

This playbook is guidance, not a promise that patches will apply to future Codex versions unchanged. Prefer small, reviewable patches and retain the official CLI as the rollback path.
