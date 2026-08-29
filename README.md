<p align="center">
  <img src="docs/assets/readme-banner.svg" alt="Codex CLI Simplified Chinese TUI" width="100%" />
</p>

<p align="center">
  <a href="https://github.com/sstxww/codex-cli-zh-cn/actions/workflows/build-windows-release.yml"><img src="https://img.shields.io/github/actions/workflow/status/sstxww/codex-cli-zh-cn/build-windows-release.yml?branch=main&style=flat-square&label=Windows%20build" alt="Windows build" /></a>
  <a href="https://github.com/sstxww/codex-cli-zh-cn/actions/workflows/build-windows-release.yml"><img src="https://img.shields.io/badge/artifact-workflow_build-2563EB?style=flat-square&logo=githubactions&logoColor=white" alt="Workflow build artifact" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20x64-0078D4?style=flat-square&logo=windows11&logoColor=white" alt="Windows x64" />
  <img src="https://img.shields.io/badge/Codex-0.150.0--alpha.8-111827?style=flat-square" alt="Codex compatibility baseline" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-3b82f6?style=flat-square" alt="Apache-2.0" /></a>
</p>

<p align="center">
  <a href="https://github.com/sstxww/codex-cli-zh-cn/releases/latest">Download</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="https://github.com/openai/codex">Official Codex</a> ·
  <a href="https://github.com/sstxww/codex-cli-zh-cn/actions/workflows/build-windows-release.yml">Build status</a>
</p>

# Codex CLI Simplified Chinese TUI

[简体中文](README.zh-CN.md)

This is an unofficial Simplified Chinese interactive terminal UI for OpenAI Codex CLI. It installs as the separate `codex-zh` command and does not replace the official `codex` command.

Compatibility baseline: official tag `rust-v0.150.0-alpha.8` (`codex-cli 0.150.0-alpha.8`).

## At a glance

| Item | Details |
| --- | --- |
| What it is | An unofficial Simplified Chinese interactive TUI for OpenAI Codex CLI |
| Command | `codex-zh` — installed alongside, not over, the official `codex` command |
| Compatibility | Official `rust-v0.150.0-alpha.8` / `codex-cli 0.150.0-alpha.8` |
| Distribution | Ready-to-run Windows x64 release plus an auditable source overlay |
| Translation boundary | Interactive UI only; canonical commands, paths, IDs, model output, and raw tool output stay unchanged |

> [!IMPORTANT]
> This repository is a community localization, not an official OpenAI distribution. Keep the official `codex` executable installed; `codex-zh` is a separate entry point.

## Fastest way to try it

1. Download `codex-cli-zh-cn-windows-x64.zip` from [the latest release](https://github.com/sstxww/codex-cli-zh-cn/releases/latest).
2. Extract it to a dedicated directory.
3. Run `codex-zh.exe`, or add it to `PATH` with:

```powershell
.\overlay\localization\install-windows.ps1 -AddToPath
```

## Scope

- Translates the interactive TUI, onboarding, pickers, settings, status views, and common prompts.
- Keeps canonical commands, flags, paths, IDs, model output, and raw tool output unchanged.
- Provides interactive `resume` and `fork` entry points through `codex-zh`.
- Leaves administrative and non-interactive commands to the official `codex` executable.

## Repository layout

`overlay/` contains every source file changed from the compatibility baseline. Keeping unchanged upstream files out of this repository makes the localization itself easier to audit. Copy the overlay onto a fresh checkout of the matching official tag before building.

## Windows x64 install

Download `codex-cli-zh-cn-windows-x64.zip` from [Releases](https://github.com/sstxww/codex-cli-zh-cn/releases/latest), extract it to a dedicated directory, and run `codex-zh.exe`. Keep the official Codex CLI installed.

The repository also contains an installer under `overlay/localization/install-windows.ps1`:

```powershell
.\overlay\localization\install-windows.ps1 -AddToPath
```

Open a new terminal after changing `PATH`.

## Build from source

```powershell
git clone https://github.com/openai/codex.git
Set-Location codex
git checkout rust-v0.150.0-alpha.8
Copy-Item -Path <localization-repository>\overlay\* -Destination . -Recurse -Force
Set-Location codex-rs
cargo build --release -p codex-tui --bin codex-zh
```

## Validation

The Windows x64 release passed formatting, focused TUI checks, a locked dependency check, a real terminal login-screen smoke test, packaging tests, and an isolated installer test. The full localized suite and the untouched official baseline produced the same result: 3,754 passed, 30 failed, and 10 skipped. The 30 failures are therefore existing Windows/upstream baseline failures, not localization regressions.

The release archive contains only `codex-zh.exe`, `codex-zh.cmd`, and `README.txt`. It does not include authentication, configuration, or session data. Local build paths were remapped before publication. The community binary is not code-signed, so Windows may show a trust prompt.

## Attribution and license

The original localization work comes from [LH-03/codex-cli-zh-cn](https://github.com/LH-03/codex-cli-zh-cn). Codex itself is maintained by OpenAI at [openai/codex](https://github.com/openai/codex). This repository follows the included Apache-2.0 license and notice.
