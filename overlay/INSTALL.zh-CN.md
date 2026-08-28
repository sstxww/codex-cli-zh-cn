# 安装非官方简体中文 TUI

## 安装前

- 当前预编译目标是 Windows x64。
- 保留官方 `codex`；本项目新增的命令叫 `codex-zh`。
- 安装中文界面不会创建、复制或修改 `CODEX_HOME`。
- 使用前查看 Release 说明。

## 让 Agent 托管安装

把仓库链接发给 Agent，让它先读 [AGENT_START_HERE.md](AGENT_START_HERE.md)。你确认方案后，它可以审阅并运行：

```powershell
Get-Content .\localization\install-windows.ps1
.\localization\install-windows.ps1 -AddToPath
```

脚本会下载最新 Release 的 ZIP，并安装到 `%LOCALAPPDATA%\Programs\codex-cli-zh-cn\bin`。它不替换官方 `codex`，也不读取认证或 Provider 配置。

重新打开 PowerShell 后检查：

```powershell
codex --version
codex-zh --version
codex-zh
```

## 古法手工安装

1. 打开最新 [GitHub Release](https://github.com/sstxww/codex-cli-zh-cn/releases/latest)。
2. 下载 `codex-cli-zh-cn-windows-x64.zip`。
3. 解压到自己的目录；用完整路径运行，或把该目录加入用户 `PATH`。
4. 不要把它改名成 `codex.exe`，这样官方版本始终保留。

## 与 DeepSeek 等隔离目录一起使用

```powershell
$env:CODEX_HOME = "$HOME\.codex-deepseek"
codex-zh resume --last
```

如需固定快捷命令和启动身份提示，使用 [codex-cli-home-switcher](https://github.com/LH-03/codex-cli-home-switcher)。两者独立安装，只在启动时组合。

## 卸载

运行 `localization\uninstall-windows.ps1`，或只删除专用安装目录及它的用户 `PATH` 项。官方 Codex、两套 `CODEX_HOME`、认证、会话和日志不会被删除。
