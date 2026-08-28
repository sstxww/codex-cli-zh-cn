# Codex CLI 简体中文界面（非官方）

这是基于 [OpenAI Codex](https://github.com/openai/codex) 的社区本地化分支，为交互式 CLI/TUI 增加独立命令 `codex-zh`。

> 把仓库链接直接发给 AI Agent 时，让它先读 [AGENT_START_HERE.md](AGENT_START_HERE.md)。它必须先问清你只要中文界面、只要配置隔离、两者都要、其他语言本地化，还是只看手工教程；得到确认后才能安装。

## 核心原则

- **不替换官方 `codex`**：中文版本使用新命令 `codex-zh`，可随时回到官方版本。
- **最小翻译边界**：翻译固定的界面文字和说明；保留 `/model`、`/resume` 等命令名、所有 flags、路径、ID、模型回答和原始工具输出。
- **不碰 Provider 配置**：继续使用当前进程选中的 `CODEX_HOME`，不读取或改写认证、Provider、会话和日志。
- **可与隔离方案自由组合**：中文界面与多 `CODEX_HOME` 是两个独立维度。

## 组合方式

| 需求 | 使用方式 |
| --- | --- |
| 只要官方英文界面 | `codex` |
| 只要中文界面 | `codex-zh` |
| 只要 OpenAI / DeepSeek 等配置隔离 | 安装 [codex-cli-home-switcher](https://github.com/LH-03/codex-cli-home-switcher) |
| 中文界面 + OpenAI 配置 | `codex-zh-openai` |
| 中文界面 + DeepSeek 配置 | `codex-zh-deepseek` |

## 当前范围

- 提供 Windows x64 预编译版本与完整 Rust 源码。
- `codex-zh` 负责交互界面以及 `resume` / `fork` 会话入口。
- 管理类或非交互式子命令继续使用官方 `codex`。
- 中文命令搜索别名只帮助查找，实际命令名仍保持英文。
- 当前版本精确对应官方标签 `rust-v0.150.0-alpha.8`；上游快速变化时，发布页会注明兼容基线。

## 安装

请读 [INSTALL.zh-CN.md](INSTALL.zh-CN.md)。Agent 可以在你确认后运行安装脚本；你也可以下载 Release 并手工放置。

## 其他语言

我们没有替所有语言预制构建，但把踩坑经验整理在 [LOCALIZATION_PLAYBOOK.md](LOCALIZATION_PLAYBOOK.md)。其他语言使用者可以让自己的 Agent 先沟通翻译范围，再做一份不覆盖官方命令的私人本地化。

## 来源与许可

本仓库继承上游 Apache-2.0 许可，汉化工作源自 [LH-03/codex-cli-zh-cn](https://github.com/LH-03/codex-cli-zh-cn)，并移植到当前官方版本。它是非官方社区项目，与 OpenAI 无隶属或背书关系。DeepSeek 仅作为可隔离的第三方 Provider 示例，也不代表其背书。
