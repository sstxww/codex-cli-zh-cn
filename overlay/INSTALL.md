# Install the unofficial zh-CN TUI

## Before installing

- Supported prebuilt target: Windows x64.
- Keep the official `codex` installed; this project adds `codex-zh`.
- Installing this UI does not create, copy, or edit a `CODEX_HOME`.
- Review the Release notes before use.

## Agent-managed or scripted install

Clone or download this repository, inspect the script, then run it only after the owner confirms:

```powershell
Get-Content .\localization\install-windows.ps1
.\localization\install-windows.ps1 -AddToPath
```

The script downloads the latest stable-named Release archive, then installs it to `%LOCALAPPDATA%\Programs\codex-cli-zh-cn\bin`. It does not replace `codex`.

Open a new terminal:

```powershell
codex --version
codex-zh --version
codex-zh
```

## Manual Release install

1. Open the latest [GitHub Release](https://github.com/sstxww/codex-cli-zh-cn/releases/latest).
2. Download `codex-cli-zh-cn-windows-x64.zip`.
3. Extract it to a user-owned directory and add that directory to your user `PATH`, or invoke `codex-zh.exe` by its full path.
4. Do not rename it to `codex.exe`.

## Use with an isolated home

The UI automatically uses the `CODEX_HOME` inherited by its process:

```powershell
$env:CODEX_HOME = "$HOME\.codex-deepseek"
codex-zh resume --last
```

For named launchers and visible identity, use [codex-cli-home-switcher](https://github.com/LH-03/codex-cli-home-switcher).

## Build from source

Follow upstream build prerequisites, then from `codex-rs`:

```powershell
cargo build --release -p codex-tui --bin codex-zh
```

The executable is under `target\release\codex-zh.exe`. Run the checks in [MAINTAINING.md](MAINTAINING.md) before trusting or publishing it.

## Uninstall

Run `localization\uninstall-windows.ps1` from a repository checkout, or remove only the dedicated install directory and its user-`PATH` entry. Codex homes and official `codex` are not removed.
