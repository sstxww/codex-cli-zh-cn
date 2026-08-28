//! Conservative localization helpers for Codex-owned TUI text.
//!
//! The localization boundary intentionally lives in `codex-tui`: protocol
//! values, slash-command names, configuration keys, model/provider IDs, file
//! paths, and model/tool output never pass through this module.  The Chinese
//! launcher opts in with `CODEX_UI_LOCALE=zh-CN`; every other invocation keeps
//! the upstream English UI.

use std::borrow::Cow;
use std::sync::OnceLock;

use ratatui::text::Line;

pub(crate) const UI_LOCALE_ENV: &str = "CODEX_UI_LOCALE";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum UiLocale {
    #[default]
    EnUs,
    ZhCn,
}

impl UiLocale {
    fn from_env_value(value: Option<&str>) -> Self {
        let Some(value) = value else {
            return Self::EnUs;
        };
        match value.trim().to_ascii_lowercase().replace('_', "-").as_str() {
            "zh" | "zh-cn" | "zh-hans" | "zh-hans-cn" => Self::ZhCn,
            _ => Self::EnUs,
        }
    }
}

pub(crate) fn current_locale() -> UiLocale {
    static LOCALE: OnceLock<UiLocale> = OnceLock::new();
    *LOCALE.get_or_init(|| UiLocale::from_env_value(std::env::var(UI_LOCALE_ENV).ok().as_deref()))
}

pub(crate) fn is_zh_cn() -> bool {
    current_locale() == UiLocale::ZhCn
}

/// Translate a fixed static UI label while retaining a static return value.
pub(crate) fn tr_static(text: &'static str) -> &'static str {
    if is_zh_cn() {
        exact_zh_cn(text).unwrap_or(text)
    } else {
        text
    }
}

/// Translate Codex-owned UI text. Unknown text is returned unchanged.
pub(crate) fn tr(text: &str) -> Cow<'_, str> {
    translate_for(current_locale(), text)
}

pub(crate) fn tr_owned(text: impl AsRef<str>) -> String {
    tr(text.as_ref()).into_owned()
}

/// Translate the textual spans in a Codex-owned hint line, preserving styles.
pub(crate) fn tr_line<'a>(mut line: Line<'a>) -> Line<'a> {
    if !is_zh_cn() {
        return line;
    }
    for span in &mut line.spans {
        span.content = Cow::Owned(tr(span.content.as_ref()).into_owned());
    }
    line
}

fn translate_for(locale: UiLocale, text: &str) -> Cow<'_, str> {
    if locale != UiLocale::ZhCn {
        return Cow::Borrowed(text);
    }
    if let Some(translated) = exact_zh_cn(text) {
        return Cow::Borrowed(translated);
    }

    // Dynamic labels are handled only when their fixed boundary is known.
    // The interpolated value is preserved byte-for-byte.
    if let Some(value) = text.strip_suffix(" (current)") {
        return Cow::Owned(format!("{value}（当前）"));
    }
    if let Some(value) = text.strip_suffix(" (default)") {
        return Cow::Owned(format!("{value}（默认）"));
    }
    if let Some(value) = text.strip_suffix(" (custom)") {
        return Cow::Owned(format!("{value}（自定义）"));
    }
    if let Some(value) = text.strip_suffix(" (key chord)") {
        return Cow::Owned(format!("{value}（组合键）"));
    }
    if let Some(value) = text.strip_prefix("Customized (")
        && let Some(count) = value.strip_suffix(')')
    {
        return Cow::Owned(format!("已自定义（{count}）"));
    }
    if let Some(value) = text.strip_prefix("Unbound (")
        && let Some(count) = value.strip_suffix(')')
    {
        return Cow::Owned(format!("未绑定（{count}）"));
    }
    if let Some(value) = text.strip_suffix(" actions.") {
        return Cow::Owned(format!("{value} 个操作。"));
    }
    if let Some((total, rest)) = text.split_once(" actions, ")
        && let Some((custom, unbound)) = rest.split_once(" customized, ")
        && let Some(unbound) = unbound.strip_suffix(" unbound.")
    {
        return Cow::Owned(format!(
            "共 {total} 个操作，{custom} 个已自定义，{unbound} 个未绑定。"
        ));
    }
    if let Some(value) = text.strip_suffix(" (non-admin sandbox)") {
        return Cow::Owned(format!("{}（非管理员沙箱）", tr_owned(value)));
    }
    if let Some(value) = text
        .strip_prefix("Yes, and don't ask again for commands that start with `")
        .and_then(|value| value.strip_suffix('`'))
    {
        return Cow::Owned(format!("是，以后不再询问以 `{value}` 开头的命令"));
    }
    if let Some(value) = text.strip_prefix("Select Reasoning Level for ") {
        return Cow::Owned(format!("选择 {value} 的推理强度"));
    }
    if let Some(value) = text.strip_prefix("Goal: ") {
        return Cow::Owned(format!("目标：{value}"));
    }
    if let Some(value) = text.strip_prefix("Permissions updated to ") {
        return Cow::Owned(format!("权限已更新为 {value}"));
    }
    if let Some(value) = text.strip_suffix(" needs your approval.") {
        return Cow::Owned(format!("{value} 需要你的批准。"));
    }
    if let Some(value) = text
        .strip_prefix("Do you want to approve network access to \"")
        .and_then(|value| value.strip_suffix("\"?"))
    {
        return Cow::Owned(format!("是否允许访问网络地址“{value}”？"));
    }
    if let Some(value) = text.strip_prefix("Current model (")
        && let Some(model) = value
            .strip_suffix(") doesn't support personalities. Try /model to pick a different model.")
    {
        return Cow::Owned(format!(
            "当前模型（{model}）不支持个性风格。请用 /model 选择其他模型。"
        ));
    }
    if let Some(value) = text.strip_prefix("Warning: OpenAI base URL is overridden to ")
        && let Some(base_url) =
            value.strip_suffix(". Selecting models may not be supported or work properly.")
    {
        return Cow::Owned(format!(
            "警告：OpenAI 基础 URL 已改为 {base_url}。模型选择可能不受支持或无法正常工作。"
        ));
    }
    if let Some(value) = text.strip_prefix("Choose a specific model and reasoning level (current: ")
        && let Some(current) = value.strip_suffix(')')
    {
        return Cow::Owned(format!("选择具体模型和推理强度（当前：{current}）"));
    }
    if let Some(value) = text.strip_prefix("Custom .tmTheme files can be added to the ")
        && let Some(path) = value.strip_suffix(" directory.")
    {
        return Cow::Owned(format!("可将自定义 .tmTheme 文件放入 {path} 目录。"));
    }
    if let Some(value) = text.strip_prefix("Installed ")
        && let Some(counts) = value.strip_suffix(" available apps.")
    {
        return Cow::Owned(format!("可用应用中已安装 {counts}。"));
    }
    if let Some(event) = text.strip_suffix(" hooks") {
        return Cow::Owned(format!("{event} 钩子"));
    }
    if let Some(count) = text
        .strip_suffix(" hooks need review before they can run.")
        .filter(|count| !count.is_empty())
    {
        return Cow::Owned(format!("{count} 个钩子需要审查后才能运行。"));
    }
    if let Some(limit) = text
        .strip_prefix("limit: ")
        .and_then(|value| value.strip_suffix(" approximate tokens"))
    {
        return Cow::Owned(format!("限制：约 {limit} Token"));
    }
    if let Some(count) = text
        .strip_prefix("Installed (")
        .and_then(|value| value.strip_suffix(')'))
    {
        return Cow::Owned(format!("已安装（{count}）"));
    }
    if let Some(value) = text.strip_prefix("Installed ")
        && let Some((installed, total)) = value
            .strip_suffix(" available plugins.")
            .and_then(|value| value.split_once(" of "))
    {
        return Cow::Owned(format!("已安装 {installed} / 共 {total} 个可用插件。"));
    }
    if let Some(count) = text
        .strip_prefix("Showing ")
        .and_then(|value| value.strip_suffix(" installed plugins."))
    {
        return Cow::Owned(format!("正在显示 {count} 个已安装插件。"));
    }
    if let Some(value) = text.strip_prefix("Installed ")
        && let Some((installed, rest)) = value.split_once(" of ")
        && let Some((total, label)) = rest.split_once(' ')
        && let Some(label) = label.strip_suffix(" plugins.")
    {
        return Cow::Owned(format!(
            "{label} 插件：已安装 {installed} / 共 {total} 个。"
        ));
    }
    if let Some(name) = text
        .strip_prefix("Remove ")
        .and_then(|value| value.strip_suffix(" marketplace?"))
    {
        return Cow::Owned(format!("移除市场源 {name}？"));
    }
    if let Some(name) = text
        .strip_prefix("Removing ")
        .and_then(|value| value.strip_suffix("..."))
    {
        return Cow::Owned(format!("正在移除 {name}…"));
    }
    if let Some(name) = text
        .strip_prefix("Upgrading ")
        .and_then(|value| value.strip_suffix(" marketplace..."))
    {
        return Cow::Owned(format!("正在升级市场源 {name}…"));
    }
    if let Some(label) = text
        .strip_prefix("Loading ")
        .and_then(|value| value.strip_suffix(" plugins..."))
    {
        return Cow::Owned(format!("正在加载 {label} 插件…"));
    }
    if let Some(name) = text
        .strip_prefix("Installing ")
        .and_then(|value| value.strip_suffix("..."))
    {
        return Cow::Owned(format!("正在安装 {name}…"));
    }
    if let Some(name) = text
        .strip_prefix("Uninstalling ")
        .and_then(|value| value.strip_suffix("..."))
    {
        return Cow::Owned(format!("正在卸载 {name}…"));
    }
    if let Some(name) = text
        .strip_prefix("Loading details for ")
        .and_then(|value| value.strip_suffix("..."))
    {
        return Cow::Owned(format!("正在加载 {name} 的详情…"));
    }
    if let Some(name) = text.strip_suffix(" installed successfully.") {
        return Cow::Owned(format!("{name} 已成功安装。"));
    }
    if let Some(name) = text.strip_suffix(" plugin installed.") {
        return Cow::Owned(format!("插件 {name} 已安装。"));
    }
    if let Some(rest) = text.strip_prefix("App setup ")
        && let Some((progress, app_name)) = rest.split_once(": ")
    {
        return Cow::Owned(format!("应用设置 {progress}：{app_name}"));
    }
    if let Some((status, instruction)) = text.split_once("   ")
        && let Some(instruction) = exact_zh_cn(instruction)
    {
        return Cow::Owned(format!("{}   {instruction}", tr_owned(status.trim_end())));
    }
    if let Some(value) = text.strip_prefix("Type a goal objective and press ") {
        return Cow::Owned(format!("输入目标内容，然后按 {value}"));
    }
    if let Some(value) = text.strip_prefix("Tip: press ")
        && let Some(key) = value.strip_suffix(" to open this list directly.")
    {
        return Cow::Owned(format!("提示：按 {key} 可直接打开此列表。"));
    }
    if let Some(value) = text.strip_suffix("% context left") {
        return Cow::Owned(format!("剩余 {value}% 上下文"));
    }
    if let Some(value) = text.strip_suffix(" used") {
        return Cow::Owned(format!("已用 {value}"));
    }
    if let Some(value) = text.strip_prefix("Plan mode") {
        return Cow::Owned(format!("计划模式{value}"));
    }
    if let Some(value) = text.strip_prefix("Pursuing goal") {
        return Cow::Owned(format!("正在推进目标{value}"));
    }
    if let Some(value) = text.strip_prefix(
        "Note: You’re in a subdirectory of a Git project. Trusting will apply to the repository root: ",
    ) {
        return Cow::Owned(format!(
            "注意：你当前位于 Git 项目的子目录中。信任操作将应用到仓库根目录：{value}"
        ));
    }

    Cow::Borrowed(text)
}

/// Chinese search aliases for the slash-command popup. Command execution still
/// uses the canonical English command returned by `SlashCommand::command()`.
pub(crate) fn command_matches_zh_alias(command: &str, filter: &str) -> bool {
    if !is_zh_cn() || filter.is_empty() {
        return false;
    }
    let aliases = match command {
        "model" => "模型 推理 强度",
        "ide" => "编辑器 上下文 选中 文件",
        "permissions" => "权限 批准 沙箱",
        "keymap" => "快捷键 键位",
        "vim" => "编辑模式",
        "setup-default-sandbox" => "设置 默认 沙箱 管理员",
        "sandbox-add-read-dir" => "沙箱 读取 目录",
        "experimental" => "实验 功能",
        "approve" => "批准 重试 自动审查",
        "memories" => "记忆",
        "skills" => "技能",
        "import" => "导入 Claude",
        "hooks" => "钩子 生命周期",
        "review" => "审查 代码 问题",
        "rename" => "重命名 任务 会话",
        "new" => "新建 对话",
        "archive" => "归档 退出",
        "delete" => "删除 会话",
        "resume" => "恢复 继续 会话",
        "fork" => "分叉 复制 会话",
        "app" => "桌面 应用",
        "init" => "初始化 AGENTS 指令",
        "compact" => "压缩 总结 上下文",
        "plan" => "计划 模式",
        "goal" => "目标 长任务",
        "agent" | "subagents" => "代理 线程 子代理",
        "side" | "btw" => "旁聊 临时 对话",
        "copy" => "复制 回复",
        "export" => "导出 对话",
        "raw" => "原始 回滚 复制",
        "diff" => "差异 修改 git",
        "mention" => "引用 文件",
        "status" => "状态 配置 用量",
        "usage" => "用量 限额 重置",
        "debug-config" => "调试 配置 来源",
        "title" => "标题 终端",
        "statusline" => "状态栏",
        "theme" => "主题 语法 颜色",
        "pets" => "宠物 动画",
        "mcp" => "工具 服务器",
        "apps" => "应用",
        "plugins" => "插件",
        "logout" => "退出 登录",
        "quit" | "exit" => "退出",
        "feedback" => "反馈 日志",
        "rollout" => "记录 文件 路径",
        "ps" => "后台 终端 进程",
        "stop" => "停止 后台 终端",
        "clear" => "清屏 新对话",
        "personality" => "个性 风格 语气",
        "test-approval" => "测试 批准",
        _ => "",
    };
    aliases.contains(filter)
}

/// Presentation-only Chinese labels for stable `tui.keymap.*` action IDs.
/// The IDs persisted to `config.toml` are never translated.
pub(crate) fn keymap_action_label(action: &str) -> Option<String> {
    if !is_zh_cn() {
        return None;
    }
    let exact = match action {
        "open_transcript" => "打开对话记录",
        "open_external_editor" => "在外部编辑器中打开草稿",
        "copy" => "复制上一条代理回复",
        "clear_terminal" => "清空终端界面",
        "toggle_vim_mode" => "切换 Vim 模式",
        "toggle_fast_mode" => "切换快速模式",
        "toggle_raw_output" => "切换原始回滚模式",
        "toggle_side_conversation" => "切换旁聊与主对话",
        "interrupt_turn" => "中断当前回合",
        "decrease_reasoning_effort" => "降低推理强度",
        "increase_reasoning_effort" => "提高推理强度",
        "edit_queued_message" => "编辑最近加入队列的消息",
        "submit" => "提交当前草稿",
        "queue" => "将草稿加入队列",
        "toggle_shortcuts" => "显示或隐藏快捷键",
        "history_search_previous" => "搜索历史中的上一个结果",
        "history_search_next" => "搜索历史中的下一个结果",
        "insert_newline" => "插入换行",
        "open_fullscreen" => "全屏查看详情",
        "open_thread" => "打开来源线程",
        "approve" => "批准主要选项",
        "approve_for_session" => "批准本次会话",
        "approve_for_prefix" => "批准命令前缀",
        "deny" => "明确拒绝",
        "decline" => "拒绝并提供修改建议",
        _ => "",
    };
    if !exact.is_empty() {
        return Some(exact.to_string());
    }

    let words = action
        .split('_')
        .map(|word| match word {
            "open" => "打开",
            "close" => "关闭",
            "move" => "移动",
            "scroll" => "滚动",
            "jump" => "跳转",
            "page" => "一页",
            "half" => "半页",
            "left" => "向左",
            "right" => "向右",
            "up" => "向上",
            "down" => "向下",
            "word" => "单词",
            "line" => "行",
            "start" => "开头",
            "end" => "结尾",
            "delete" | "kill" => "删除",
            "backward" => "前一个",
            "forward" => "下一个",
            "whole" => "整行",
            "yank" => "复制",
            "paste" => "粘贴",
            "enter" => "进入",
            "insert" => "插入",
            "append" => "追加",
            "after" => "之后",
            "cursor" => "光标",
            "char" => "字符",
            "substitute" | "change" => "修改",
            "operator" => "操作符",
            "motion" => "动作",
            "select" => "选择",
            "inner" => "内部",
            "around" => "包围",
            "text" => "文本",
            "object" => "对象",
            "cancel" => "取消",
            "parentheses" => "圆括号",
            "brackets" => "方括号",
            "braces" => "花括号",
            "double" => "双",
            "single" => "单",
            "quote" => "引号",
            "backtick" => "反引号",
            "list" => "列表",
            "accept" => "确认",
            "transcript" => "对话记录",
            other => other,
        })
        .collect::<Vec<_>>()
        .join(" ");
    Some(words)
}

fn exact_zh_cn(text: &str) -> Option<&'static str> {
    Some(match text {
        // Slash-command descriptions. Canonical command names stay English.
        "send logs to maintainers" => "向维护者发送日志",
        "start a new chat during a conversation" => "在当前对话中开始新会话",
        "create an AGENTS.md file with instructions for Codex" => {
            "创建包含 Codex 项目指令的 AGENTS.md"
        }
        "summarize conversation to prevent hitting the context limit" => {
            "总结对话，避免达到上下文上限"
        }
        "review my current changes and find issues" => "审查当前修改并查找问题",
        "rename the current thread" => "重命名当前任务",
        "resume a saved chat" => "恢复已保存的会话",
        "archive this session and exit" => "归档当前会话并退出",
        "permanently delete this session and exit" => "永久删除当前会话并退出",
        "clear the terminal and start a new chat" => "清空终端并开始新会话",
        "fork the current chat" => "从当前会话创建分支",
        "continue this session in the Desktop app" => "在 Codex 桌面应用中继续此会话",
        "exit Codex" => "退出 Codex",
        "copy last response as markdown" => "以 Markdown 复制上一条回复",
        "export the conversation as markdown" => "将对话导出为 Markdown",
        "toggle raw scrollback mode for copy-friendly terminal selection" => {
            "切换原始回滚模式，便于在终端中选择复制"
        }
        "show git diff (including untracked files)" => "显示 Git 差异（包括未跟踪文件）",
        "mention a file" => "在提示词中引用文件",
        "use skills to improve how Codex performs specific tasks" => {
            "使用技能改进 Codex 执行特定任务的方式"
        }
        "import setup, this project, and recent chats from Claude Code" => {
            "从 Claude Code 导入设置、当前项目和最近会话"
        }
        "view and manage lifecycle hooks" => "查看和管理生命周期钩子",
        "show current session configuration and token usage" => "显示当前会话配置和 Token 用量",
        "view account usage or use a usage limit reset" => "查看账户用量或使用限额重置",
        "show config layers and requirement sources for debugging" => {
            "显示配置层和要求来源，用于调试"
        }
        "configure which items appear in the terminal title" => "配置终端标题显示哪些项目",
        "configure which items appear in the status line" => "配置状态栏显示哪些项目",
        "choose a syntax highlighting theme" => "选择语法高亮主题",
        "choose or hide the terminal pet" => "选择或隐藏终端宠物",
        "list background terminals" => "列出后台终端",
        "stop all background terminals" => "停止所有后台终端",
        "DO NOT USE" => "请勿使用",
        "choose what model and reasoning effort to use" => "选择模型和推理强度",
        "include current selection, open files, and other context from your IDE" => {
            "加入 IDE 中的当前选区、已打开文件和其他上下文"
        }
        "choose a communication style for Codex" => "选择 Codex 的沟通风格",
        "switch to Plan mode" => "切换到计划模式",
        "set or view the goal for a long-running task" => "设置或查看长时间任务的目标",
        "switch the active agent thread" => "切换当前代理线程",
        "start a side conversation in an ephemeral fork" => "在临时分支中开始旁聊",
        "choose what Codex is allowed to do" => "选择允许 Codex 执行的操作",
        "remap TUI shortcuts" => "重新设置终端界面快捷键",
        "toggle Vim mode for the composer" => "切换输入框的 Vim 模式",
        "set up elevated agent sandbox" => "设置增强保护的代理沙箱",
        "let sandbox read a directory: /sandbox-add-read-dir <absolute_path>" => {
            "允许沙箱读取目录：/sandbox-add-read-dir <absolute_path>"
        }
        "toggle experimental features" => "开关实验性功能",
        "approve one retry of a recent auto-review denial" => {
            "批准重试一次最近被自动审查拒绝的操作"
        }
        "configure memory use and generation" => "配置记忆的使用和生成",
        "list configured MCP tools; use /mcp verbose for details" => {
            "列出已配置的 MCP 工具；用 /mcp verbose 查看详情"
        }
        "manage apps" => "管理应用",
        "browse plugins" => "浏览插件",
        "log out of Codex" => "退出 Codex 登录",
        "print the rollout file path" => "显示会话记录文件路径",
        "test approval request" => "测试批准请求",

        // Generic picker chrome and status markers.
        "no matches" => "没有匹配项",
        "Type to search" => "输入文字搜索",
        "Type to search apps" => "输入文字搜索应用",
        "Type to filter themes..." => "输入文字筛选主题…",
        "Type to filter pets..." => "输入文字筛选宠物…",
        " (current)" => "（当前）",
        " (default)" => "（默认）",
        "Press " => "按 ",
        " to toggle" => " 切换开关",
        " to move" => " 调整顺序",
        " to confirm and close" => " 确认并关闭",
        " to close" => " 关闭",
        " to trust all; " => " 信任全部；",
        " to review hooks; " => " 审查钩子；",
        " to view hooks; " => " 查看钩子；",
        " to trust; " => " 信任；",
        " to toggle; " => " 切换开关；",
        "to confirm" => "确认",
        "to cancel" => "取消",
        " or " => " 或 ",
        " to open thread" => " 打开线程",
        "Current" => "当前",
        "Default" => "默认",
        "Enabled" => "已启用",
        "Disabled" => "已禁用",
        "Installed" => "已安装",
        "Installed · Disabled" => "已安装 · 已禁用",
        "Can be installed" => "可安装",
        "On" => "开",
        "Off" => "关",
        "Yes" => "是",
        "No" => "否",
        "Cancel" => "取消",
        "Close" => "关闭",
        "Back" => "返回",
        "Try again" => "重试",
        "Loading..." => "正在加载…",
        "Refreshing..." => "正在刷新…",
        "Command" => "命令",
        "Rationale" => "理由",
        "Search" => "搜索",
        "None" => "无",
        "Minimal" => "最少",
        "Low" => "低",
        "Medium" => "中",
        "High" => "高",
        "Extra high" => "很高",
        "Max" => "最高",
        "Ultra" => "极高",

        // Memories settings. Memory files and config keys stay untouched.
        "Memories" => "记忆设置",
        "Use memories" => "使用记忆",
        "Use memories in the following threads. Applied at next thread." => {
            "在后续任务中使用记忆；从下一个任务开始生效。"
        }
        "Generate memories" => "生成记忆",
        "Generate memories from the following threads. Current thread included." => {
            "从后续任务生成记忆；包括当前任务。"
        }
        "Reset all memories" => "重置全部记忆",
        "Clear local memory files and summaries. Existing threads stay intact." => {
            "清除本地记忆文件和摘要；现有任务保持不变。"
        }
        "Learn more: " => "了解更多：",
        "Choose how Codex uses and creates memories. Changes are saved to config.toml" => {
            "选择 Codex 如何使用和生成记忆。修改会保存到 config.toml"
        }
        "Reset all memories?" => "重置全部记忆？",
        "This clears local memory files and rollout summaries for the current Codex home." => {
            "这会清除当前 CODEX_HOME 中的本地记忆文件和会话摘要。"
        }
        "Go back" => "返回",
        "Delete local memory files and rollout summaries." => "删除本地记忆文件和会话摘要。",
        "Return to memory settings." => "返回记忆设置。",
        "  No memory settings available" => "  没有可用的记忆设置",
        " to save or select" => " 保存或选择",

        // Lifecycle hooks browser. Hook event IDs, matchers, commands, and paths
        // intentionally remain canonical.
        "Hooks" => "钩子",
        "Lifecycle hooks from config and enabled plugins." => {
            "来自配置和已启用插件的生命周期钩子。"
        }
        "Turn hooks on or off. Your changes are saved automatically." => {
            "开启或关闭钩子；修改会自动保存。"
        }
        "Event" => "事件",
        "Review" => "待审查",
        "Description" => "说明",
        "Handler" => "处理器",
        "Prompt" => "提示词",
        "Agent" => "代理",
        "MCP Server" => "MCP 服务器",
        "MCP Tool" => "MCP 工具",
        "Issues" => "问题",
        "modified" => "已修改",
        "No hooks installed for this event." => "此事件没有已安装的钩子。",
        "Matcher" => "匹配条件",
        "Source" => "来源",
        "Mode" => "模式",
        "Timeout" => "超时",
        "Context" => "上下文",
        "Trust" => "信任状态",
        "Sync" => "同步",
        "Async" => "异步",
        "unlimited" => "无限制",
        "1 hook needs review before it can run." => "1 个钩子需要审查后才能运行。",
        "Managed" => "管理员托管",
        "Trusted" => "已信任",
        "New hook - review required" => "新钩子 - 需要审查",
        "Modified since last trusted - review required" => "信任后已修改 - 需要重新审查",
        "Before a tool executes" => "工具执行前",
        "When permission is requested" => "请求权限时",
        "After a tool executes" => "工具执行后",
        "Before context compaction" => "压缩上下文前",
        "After context compaction" => "压缩上下文后",
        "When a new session starts" => "新会话开始时",
        "Right before a session ends" => "会话结束前",
        "When the user submits a prompt" => "用户提交提示词时",
        "When a subagent is created" => "创建子代理时",
        "Right before a subagent ends its turn" => "子代理结束本轮前",
        "Right before Codex ends its turn" => "Codex 结束本轮前",
        "Plugin" => "插件",
        "Admin config" => "管理员配置",
        "User config" => "用户配置",
        "Project config" => "项目配置",
        "Session flags" => "会话参数",
        "Cloud-managed config" => "云端托管配置",
        "Unknown source" => "未知来源",
        "Managed hooks are always on; press " => "管理员托管的钩子始终开启；按 ",

        // /status card. Model/provider/account values, paths, IDs, and URLs are
        // never translated; only Codex-owned labels and explanations are.
        "Model" => "模型",
        "Directory" => "目录",
        "Permissions" => "权限",
        "Agents.md" => "Agents.md",
        "Model provider" => "模型 Provider",
        "Account" => "账户",
        "Thread name" => "任务名称",
        "Session" => "会话",
        "Forked from" => "分支来源",
        "Collaboration mode" => "协作模式",
        "Token usage" => "Token 用量",
        "Context window" => "上下文窗口",
        "Remote" => "远程连接",
        "Limits" => "用量限额",
        "Warning" => "警告",
        "API key configured (run codex login to use ChatGPT)" => {
            "API Key 已配置（运行 codex login 可改用 ChatGPT）"
        }
        "Visit " => "访问 ",
        " for up-to-date" => " 查看最新的",
        "information on rate limits and credits" => "用量限额和额度信息",

        // Composer footer and shortcut overlay.
        "shift+tab to cycle" => "Shift+Tab 切换模式",
        " for shortcuts" => " 查看快捷键",
        " to queue message" => " 将消息加入队列",
        " to queue" => " 加入队列",
        " again to quit" => " 再按一次退出",
        " again to edit previous message" => " 再按一次编辑上一条消息",
        " to edit previous message" => " 编辑上一条消息",
        " to submit message" => " 提交消息",
        " to interrupt" => " 中断",
        " to exit" => " 退出",
        " for commands" => " 查看命令",
        " for shell commands" => " 输入 Shell 命令",
        " for newline" => " 换行",
        " for file paths" => " 插入文件路径",
        " to paste images" => " 粘贴图片",
        " to edit in external editor" => " 在外部编辑器中编辑",
        " search history" => " 搜索历史",
        " to view transcript" => " 查看对话记录",
        " to change mode" => " 切换模式",
        " reasoning down" => " 降低推理强度",
        " reasoning up" => " 提高推理强度",
        "reverse-i-search: " => "反向搜索历史：",
        "IDE context" => "IDE 上下文",
        "Side" => "旁聊",
        "Goal paused (/goal resume)" => "目标已暂停（/goal resume 可恢复）",
        "Goal stalled (/goal resume)" => "目标已停滞（/goal resume 可恢复）",
        "Goal hit usage limits (/goal resume)" => "目标达到用量限制（/goal resume 可恢复）",
        "Goal abandoned" => "目标未完成",
        "Goal achieved" => "目标已达成",
        "customize shortcuts with " => "使用以下命令自定义快捷键：",

        // Onboarding and sign-in.
        "Welcome to " => "欢迎使用 ",
        ", OpenAI's command-line coding agent" => "，OpenAI 的命令行编程代理",
        "You are in " => "当前目录：",
        "Do you trust the contents of this directory? Working with untrusted contents comes with higher risk of prompt injection. Trusting the directory allows project-local config, hooks, and exec policies to load." => {
            "你信任此目录中的内容吗？处理不受信任的内容会增加提示注入风险。信任此目录后，将允许加载项目本地配置、钩子和执行策略。"
        }
        "Yes, continue" => "是，继续",
        "No, quit" => "否，退出",
        " to continue and create a sandbox..." => " 继续并创建沙箱…",
        " to continue" => " 继续",
        "Sign in with ChatGPT to use Codex as part of your paid plan" => {
            "登录 ChatGPT，将 Codex 作为付费方案的一部分使用"
        }
        "or connect an API key for usage-based billing" => "或者连接 API Key，按用量计费",
        "ChatGPT login is disabled" => "ChatGPT 登录已禁用",
        "Usage included with Plus, Pro, Business, and Enterprise plans" => {
            "Plus、Pro、Business 和 Enterprise 方案包含用量"
        }
        "Sign in from another device with a one-time code" => "使用一次性代码从其他设备登录",
        "Sign in with ChatGPT" => "使用 ChatGPT 登录",
        "Sign in with Device Code" => "使用设备代码登录",
        "Provide your own API key" => "提供自己的 API Key",
        "Pay for what you use" => "按实际用量付费",
        "Finish signing in via your browser" => "请在浏览器中完成登录",
        "  If the link doesn't open automatically, open the following link to authenticate:" => {
            "  如果链接未自动打开，请打开以下链接完成验证："
        }
        "  On a remote or headless machine? Press " => "  正在远程或无界面设备上使用？请按 ",
        " and choose " => "，然后选择 ",
        " to cancel" => " 取消",
        "✓ Signed in with your ChatGPT account" => "✓ 已登录 ChatGPT 账户",
        "✓ API key configured" => "✓ API Key 已配置",
        "  Codex will use usage-based billing with your API key." => {
            "  Codex 将使用你的 API Key 并按用量计费。"
        }
        "Use your own OpenAI API key for usage-based billing" => {
            "使用自己的 OpenAI API Key，按用量计费"
        }
        "  Paste or type your API key below. It will be stored locally in auth.json." => {
            "  在下方粘贴或输入 API Key。它将保存在本机 auth.json 中。"
        }
        "  Detected OPENAI_API_KEY environment variable." => "  检测到 OPENAI_API_KEY 环境变量。",
        "  Paste a different key if you prefer to use another account." => {
            "  如果想使用其他账户，请粘贴不同的 Key。"
        }
        "Paste or type your API key" => "粘贴或输入 API Key",
        "API key" => "API Key",
        " to save" => " 保存",
        " to go back" => " 返回",

        // Session resume/fork picker chrome. Session titles, paths, models,
        // transcript text, and provider IDs are intentionally untouched.
        "Resume a previous session" => "恢复以前的会话",
        "Fork a previous session" => "从以前的会话创建分支",
        "resume" => "恢复",
        "fork" => "分支",
        "Created" => "创建时间",
        "Updated" => "更新时间",
        "Search: " => "搜索：",
        "Active" => "活动",
        "Archived" => "已归档",
        "Status: " => "状态：",
        "Sort:" => "排序：",
        "Sort: " => "排序：",
        "Filter:" => "筛选：",
        "Filter: " => "筛选：",
        "Cwd" => "当前目录",
        "All" => "全部",
        "Loading transcript…" => "正在加载对话记录…",
        "Searching…" => "正在搜索…",
        "No results for your search" => "没有符合搜索条件的结果",
        "Loading sessions…" => "正在加载会话…",
        "Loading older sessions…" => "正在加载更早的会话…",
        "No sessions yet" => "还没有会话",
        "transcript" => "对话记录",
        "quit" => "退出",
        "restore" => "恢复归档",
        "start new" => "开始新会话",
        "new" => "新建",
        "exit" => "退出",
        "clear search" => "清除搜索",
        "clear" => "清除",
        "dense view" => "紧凑视图",
        "comfortable view" => "舒适视图",
        "dense" => "紧凑",
        "comfy" => "舒适",
        "archive" => "归档",
        "focus sort/filter" => "聚焦排序/筛选",
        "focus" => "聚焦",
        "change option" => "更改选项",
        "option" => "选项",
        "preview" => "预览",
        "expand" => "展开",
        "exp" => "展开",
        "browse" => "浏览",

        // Startup/session status card.
        "  To get started, describe a task or try one of these commands:" => {
            "  开始使用：描述一个任务，或尝试以下命令："
        }
        " - create an AGENTS.md file with instructions for Codex" => {
            " - 创建包含 Codex 项目指令的 AGENTS.md"
        }
        " - show current session configuration" => " - 显示当前会话配置",
        " - choose what Codex is allowed to do" => " - 选择允许 Codex 执行的操作",
        " - choose what model and reasoning effort to use" => " - 选择模型和推理强度",
        " - review any changes and find issues" => " - 审查修改并查找问题",
        "model changed:" => "模型已更改：",
        "requested: " => "请求的模型：",
        "used: " => "实际使用：",
        "model:" => "模型：",
        "directory:" => "目录：",
        "permissions:" => "权限：",
        " to change" => " 可更改",
        "fast" => "快速",
        "YOLO mode" => "完全访问模式",
        "none" => "无",
        "minimal" => "最少",
        "low" => "低",
        "medium" => "中",
        "high" => "高",
        "xhigh" => "很高",
        "max" => "最高",
        "ultra" => "极高",

        // Keymap picker and action editor.
        "Keymap" => "快捷键设置",
        "All configurable shortcuts." => "全部可配置快捷键。",
        "Common" => "常用",
        "Frequently customized shortcuts." => "经常自定义的快捷键。",
        "Root-level shortcut overrides." => "根配置中的快捷键覆盖项。",
        "Actions without an active shortcut." => "当前没有有效快捷键的操作。",
        "App" => "应用",
        "Composer" => "输入框",
        "Editor" => "编辑器",
        "Navigation" => "导航",
        "Approval" => "批准",
        "Global" => "全局",
        "Chat" => "对话",
        "Vim normal" => "Vim 普通模式",
        "Vim operator" => "Vim 操作符",
        "Vim text object" => "Vim 文本对象",
        "Pager" => "分页器",
        "List" => "列表",
        "Global and chat-level shortcuts." => "全局和对话级快捷键。",
        "Composer submission and queue shortcuts." => "输入框提交和队列快捷键。",
        "Inline editor movement and editing shortcuts." => "行内编辑器的移动和编辑快捷键。",
        "Vim normal-mode and operator shortcuts." => "Vim 普通模式和操作符快捷键。",
        "Pager and selection-list navigation shortcuts." => "分页器和选择列表的导航快捷键。",
        "Approval prompt shortcuts." => "批准提示框快捷键。",
        "No shortcuts available" => "没有可用快捷键",
        "No configurable shortcuts are available." => "没有可配置的快捷键。",
        "No common shortcuts" => "没有常用快捷键",
        "No common shortcut actions are available." => "没有可用的常用快捷键操作。",
        "No customized shortcuts" => "没有已自定义的快捷键",
        "No root-level keymap overrides have been configured." => "尚未配置根级快捷键覆盖项。",
        "No unbound shortcuts" => "没有未绑定的快捷键",
        "Every configurable action currently has a shortcut." => "每个可配置操作当前都有快捷键。",
        "No shortcuts in this group" => "此分组中没有快捷键",
        "No configurable actions are available in this group." => "此分组中没有可配置操作。",
        "Type to search shortcuts" => "输入文字搜索快捷键",
        "Debug" => "调试",
        "Inspect keypresses from your terminal." => "检查终端报告的按键。",
        "See the key Codex detects and any shortcuts assigned to it." => {
            "查看 Codex 检测到的按键，以及分配给它的快捷键。"
        }
        "Inspect keypresses" => "检查按键",
        "Press Enter to start. Then press any key to inspect it; Ctrl+C exits." => {
            "按 Enter 开始，然后按任意键检查；Ctrl+C 退出。"
        }
        "Open a live inspector that shows the detected key, config key, and matching actions." => {
            "打开实时检查器，显示检测到的按键、配置键和匹配操作。"
        }
        "left/right" => "左/右",
        " group · " => " 切换分组 · ",
        "enter" => "Enter",
        " edit shortcut · " => " 编辑快捷键 · ",
        " custom · " => " 已自定义 · ",
        " unbound · " => " 未绑定 · ",
        "esc" => "Esc",
        " close" => " 关闭",
        " start inspector · " => " 启动检查器 · ",
        " select · " => " 选择 · ",
        " back" => " 返回",
        "unbound" => "未绑定",
        "Custom" => "自定义",
        "Custom root override" => "根配置自定义覆盖",
        "Default keymap" => "默认快捷键",
        "Edit Shortcut" => "编辑快捷键",
        "Current " => "当前：",
        "Config " => "配置项：",
        "Configure this shortcut." => "配置此快捷键。",
        "Set key" => "设置按键",
        "Capture a key for this unbound action." => "为此未绑定操作捕获一个按键。",
        "Capture one key and bind this action." => "捕获一个按键并绑定此操作。",
        "Replace binding" => "替换绑定",
        "Add binding" => "添加绑定",
        "Remove custom binding" => "删除自定义绑定",
        "Restore the default keymap binding." => "恢复默认快捷键绑定。",
        "Back to shortcuts" => "返回快捷键列表",
        "Return to the shortcut list." => "返回快捷键列表。",
        "Replace this binding." => "替换此绑定。",
        "Replace this binding with a key chord." => "用组合键替换此绑定。",
        "Shortcut Conflict" => "快捷键冲突",
        "Pick another key" => "选择其他按键",
        "Return to key capture for this action." => "返回并为此操作重新捕获按键。",
        "Leave keymap unchanged." => "保持快捷键设置不变。",

        // Experimental settings.
        "Experimental features" => "实验性功能",
        "Toggle experimental features. Changes are saved to config.toml." => {
            "开关实验性功能。修改会保存到 config.toml。"
        }
        "  No experimental features available for now" => "  当前没有可用的实验性功能",
        " to select" => " 选择",
        " to save for next conversation" => " 保存并在下次对话生效",
        "Network proxy" => "网络代理",
        "Apply network proxy restrictions to sandboxed sessions that already have network access." => {
            "对已拥有网络访问权的沙箱会话应用网络代理限制。"
        }
        "Prevent sleep while running" => "运行时防止休眠",
        "Keep your computer awake while Codex is running a thread." => {
            "Codex 运行任务时让电脑保持唤醒。"
        }

        // Model and personality settings.
        "Model selection is disabled until startup completes." => "启动完成前无法选择模型。",
        "Models are being updated; please try /model again in a moment." => {
            "模型列表正在更新，请稍后再试 /model。"
        }
        "All models" => "全部模型",
        "Select Model" => "选择模型",
        "Pick a quick auto mode or browse all models." => "选择快速自动模式，或浏览全部模型。",
        "No additional models are available right now." => "当前没有其他可用模型。",
        "Select Model and Effort" => "选择模型和推理强度",
        "Access legacy models by running codex -m <model_name> or in your config.toml" => {
            "旧模型可通过 codex -m <model_name> 或 config.toml 使用"
        }
        "More reasoning…" => "更高推理强度…",
        "Advanced Reasoning" => "高级推理",
        "⚠ Consumes usage limits faster" => "⚠ 会更快消耗用量限额",
        "For difficult problems when quality matters more than speed · higher usage" => {
            "适合质量比速度更重要的难题 · 用量较高"
        }
        "For demanding work using multiple agents · highest usage" => {
            "适合使用多个代理的高要求任务 · 用量最高"
        }
        "Select Personality" => "选择个性风格",
        "Choose a communication style for Codex." => "选择 Codex 的沟通风格。",
        "Personality selection is disabled until startup completes." => {
            "启动完成前无法选择个性风格。"
        }
        "Friendly" => "友好",
        "Pragmatic" => "务实",
        "No personality instructions." => "不添加个性风格指令。",
        "Warm, collaborative, and helpful." => "温和、协作并乐于提供帮助。",
        "Concise, task-focused, and direct." => "简洁、专注任务并直截了当。",

        // Permissions and approvals.
        "Update Model Permissions" => "更新模型权限",
        "Configured permission profile." => "已配置的权限方案。",
        "Disabled by requirements." => "因管理要求而禁用。",
        "Ask for approval" => "需要时询问批准",
        "Approve for me" => "由自动审查批准",
        "Only ask for actions detected as potentially unsafe." => {
            "仅对检测为可能不安全的操作发起询问。"
        }
        "Read Only" => "只读",
        "Full Access" => "完全访问",
        "Codex can read files in the current workspace. Approval is required to edit files or access the internet." => {
            "Codex 可以读取当前工作区文件；编辑文件或访问互联网需要批准。"
        }
        "Codex can read and edit files in the current workspace, and run commands. Approval is required to access the internet or edit other files." => {
            "Codex 可以读取和编辑当前工作区文件并运行命令；访问互联网或编辑其他文件需要批准。"
        }
        "Codex can edit files outside this workspace and access the internet without asking for approval. Exercise caution when using." => {
            "Codex 可编辑工作区外的文件并直接访问互联网，无需询问批准；使用时请谨慎。"
        }
        "Would you like to run the following command?" => "是否运行以下命令？",
        "Would you like to grant these permissions?" => "是否授予这些权限？",
        "Would you like to make the following edits?" => "是否执行以下修改？",
        "Thread: " => "线程：",
        "Description: " => "说明：",
        "Apply proposed file edits" => "应用建议的文件修改",
        "Destination: " => "目标位置：",
        "unavailable" => "不可用",
        "Environment: " => "环境：",
        "Reason: " => "原因：",
        "Permission rule: " => "权限规则：",
        "Server: " => "服务器：",
        "Yes, just this once" => "是，仅此一次",
        "Yes, proceed" => "是，继续",
        "Yes, and allow this host for this conversation" => "是，并在本次对话中允许此主机",
        "Yes, and allow these permissions for this session" => "是，并在本次会话中允许这些权限",
        "Yes, and don't ask again for this command in this session" => "是，本次会话不再询问此命令",
        "Yes, and allow this host in the future" => "是，以后也允许此主机",
        "No, and block this host in the future" => "否，并在以后阻止此主机",
        "No, continue without running it" => "否，不运行并继续",
        "No, and tell Codex what to do differently" => "否，并告诉 Codex 应如何调整",
        "Yes, and don't ask again for these files" => "是，这些文件不再询问",
        "Yes, grant these permissions for this turn" => "是，仅本轮授予这些权限",
        "Yes, grant for this turn with strict auto review" => "是，本轮授予并启用严格自动审查",
        "Yes, grant these permissions for this session" => "是，本次会话授予这些权限",
        "No, continue without permissions" => "否，不授予权限并继续",
        "Yes, provide the requested info" => "是，提供所请求的信息",
        "No, but continue without it" => "否，不提供但继续",
        "Cancel this request" => "取消此请求",
        "Enable full access?" => "启用完全访问权限？",
        "We strongly recommend selecting \"Approve for me\" instead, and customizing the reviewer policy for your use case." => {
            "强烈建议改选“由自动审查批准”，并按使用场景自定义审查策略。"
        }
        "We strongly recommend selecting \"Ask for approval\" instead." => {
            "强烈建议改选“需要时询问批准”。"
        }
        "When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval." => {
            "启用完全访问后，Codex 无需你的批准即可编辑电脑上的任何文件，并运行可访问网络的命令。"
        }
        "When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval. " => {
            "启用完全访问后，Codex 无需你的批准即可编辑电脑上的任何文件，并运行可访问网络的命令。"
        }
        "Cyber models carry a higher risk of dangerous actions." => {
            "安全研究模型执行危险操作的风险更高。"
        }
        " Ensure proper safeguards are in place before granting full access. " => {
            " 授予完全访问权限前，请确保已采取适当保护措施。 "
        }
        "Exercise caution when enabling full access. This significantly increases the risk of data loss, leaks, or unexpected behavior." => {
            "启用完全访问权限时请谨慎；这会显著增加数据丢失、泄露或意外行为的风险。"
        }
        "Yes, continue anyway" => "是，仍然继续",
        "Apply full access for this session" => "本次会话使用完全访问权限",
        "Go back without enabling full access" => "返回且不启用完全访问权限",
        "Auto-review Denials" => "自动审查拒绝记录",
        "Select a denied action to approve." => "选择一个被拒绝的操作并批准重试。",
        "No recent auto-review denials in this thread." => "此线程中没有最近的自动审查拒绝记录。",
        "Denials are recorded after auto-review rejects an action." => {
            "自动审查拒绝操作后会在这里记录。"
        }

        // Goal and usage menus.
        "Edit goal" => "编辑目标",
        "Resume paused goal?" => "恢复已暂停的目标？",
        "Resume goal" => "恢复目标",
        "Mark it active and continue when idle" => "设为活动状态，并在空闲时继续",
        "Leave paused" => "保持暂停",
        "Keep it paused; use /goal resume later" => "保持暂停；稍后用 /goal resume 恢复",
        "Goal" => "目标",
        "Objective: " => "内容：",
        "Time used: " => "已用时间：",
        "Tokens used: " => "已用 Token：",
        "Token budget: " => "Token 预算：",
        "active" => "进行中",
        "paused" => "已暂停",
        "stalled" => "已停滞",
        "usage limited" => "受用量限制",
        "limited by budget" => "受预算限制",
        "complete" => "已完成",
        "Usage" => "用量",
        "View account usage or redeem an earned reset." => "查看账户用量，或兑换已获得的限额重置。",
        "Show usage" => "显示用量",
        "View recent account token usage." => "查看最近的账户 Token 用量。",
        "Redeem usage limit reset" => "兑换用量限额重置",
        "Check reset availability." => "检查可用的重置次数。",
        "No usage limit resets available." => "没有可用的用量限额重置。",
        "Usage limit resets" => "用量限额重置",
        "Checking your available resets..." => "正在检查可用的重置次数…",
        "You don't have any usage limit resets available." => "你没有可用的用量限额重置。",
        "Use this reset?" => "使用这次重置？",
        "Yes, use reset" => "是，使用重置",
        "No, go back" => "否，返回",
        "Choose a different reset." => "选择其他重置。",
        "Resetting your usage..." => "正在重置用量…",
        "Using a reset..." => "正在使用重置…",
        "Couldn't reset usage. Please try again." => "无法重置用量，请重试。",

        // Skills, apps, themes, and pets.
        "Skills" => "技能",
        "Choose an action" => "选择操作",
        "List skills" => "列出技能",
        "Enable/Disable Skills" => "启用或禁用技能",
        "Enable or disable skills." => "启用或禁用技能。",
        "No skills available." => "没有可用技能。",
        "Apps" => "应用",
        "Apps are disabled." => "应用功能已禁用。",
        "Enable the apps feature to use $ or /apps." => "启用应用功能后才能使用 $ 或 /apps。",
        "No apps available." => "没有可用应用。",
        "Loading installed and available apps..." => "正在加载已安装和可用的应用…",
        "Loading apps..." => "正在加载应用…",
        "This updates when the full list is ready." => "完整列表准备好后会自动更新。",
        "Use $ to insert an installed app into your prompt." => "使用 $ 将已安装应用插入提示词。",
        "Manage this app in your browser." => "在浏览器中管理此应用。",
        "Install this app in your browser, then reload Codex." => {
            "在浏览器中安装此应用，然后重新加载 Codex。"
        }
        "Plugins" => "插件",
        "Plugins are disabled." => "插件功能已禁用。",
        "Loading available plugins..." => "正在加载可用插件…",
        "Loading plugins..." => "正在加载插件…",
        "This updates when the marketplace list is ready." => "市场源列表准备好后会自动更新。",
        "Adding marketplace..." => "正在添加市场源…",
        "This updates when marketplace installation completes." => "市场源安装完成后会自动更新。",
        "This removes the configured marketplace from Codex." => {
            "这会从 Codex 中移除已配置的市场源。"
        }
        "Remove marketplace" => "移除市场源",
        "Remove this marketplace from the available plugin list." => {
            "从可用插件列表中移除此市场源。"
        }
        "Back to plugins" => "返回插件列表",
        "Keep this marketplace installed." => "保留此市场源。",
        "Removing marketplace..." => "正在移除市场源…",
        "This updates when marketplace removal completes." => "市场源移除完成后会自动更新。",
        "Upgrading marketplaces..." => "正在升级市场源…",
        "This updates when marketplace upgrade completes." => "市场源升级完成后会自动更新。",
        "Loading plugin details..." => "正在加载插件详情…",
        "This updates when plugin details load." => "插件详情加载完成后会自动更新。",
        "Installing plugin..." => "正在安装插件…",
        "This updates when plugin installation completes." => "插件安装完成后会自动更新。",
        "Uninstalling plugin..." => "正在卸载插件…",
        "This updates when the plugin removal completes." => "插件卸载完成后会自动更新。",
        "Failed to load plugins." => "插件加载失败。",
        "Plugin marketplace unavailable" => "插件市场源不可用",
        "Failed to add marketplace." => "添加市场源失败。",
        "Marketplace add failed" => "市场源添加失败",
        "Failed to add marketplace from the provided source." => "无法从提供的来源添加市场源。",
        "Enter a marketplace source." => "输入市场源。",
        "Failed to remove marketplace." => "移除市场源失败。",
        "Marketplace removal failed" => "市场源移除失败",
        "Failed to remove the selected marketplace." => "无法移除所选市场源。",
        "Review the confirmation prompt again." => "重新查看确认提示。",
        "Failed to load plugin details." => "插件详情加载失败。",
        "Plugin detail unavailable" => "插件详情不可用",
        "Return to the plugin list." => "返回插件列表。",
        "All Plugins" => "全部插件",
        "Browse plugins from available marketplaces." => "浏览可用市场源中的插件。",
        "Installed plugins." => "已安装插件。",
        "No marketplace plugins available" => "没有可用的市场插件",
        "No plugins are available in the discovered marketplaces." => {
            "已发现的市场源中没有可用插件。"
        }
        "No installed plugins" => "没有已安装插件",
        "No installed plugins." => "没有已安装插件。",
        "OpenAI Curated" => "OpenAI 精选",
        "OpenAI Curated marketplace." => "OpenAI 精选市场源。",
        "Loading OpenAI Curated plugins..." => "正在加载 OpenAI 精选插件…",
        "OpenAI Curated unavailable" => "OpenAI 精选不可用",
        "No OpenAI Curated plugins available" => "没有可用的 OpenAI 精选插件",
        "No OpenAI Curated plugins available." => "没有可用的 OpenAI 精选插件。",
        "Workspace" => "工作区",
        "Shared with me" => "与我共享",
        "Shared with me (link)" => "与我共享（链接）",
        "This updates when OpenAI Curated plugins finish loading." => {
            "OpenAI 精选插件加载完成后会自动更新。"
        }
        "This updates when workspace plugins finish loading." => "工作区插件加载完成后会自动更新。",
        "This updates when shared plugins finish loading." => "共享插件加载完成后会自动更新。",
        "No workspace plugins available" => "没有可用的工作区插件",
        "No workspace directory plugins are available." => "工作区目录中没有可用插件。",
        "No shared plugins available" => "没有可用的共享插件",
        "No plugins have been shared with you." => "目前没有与你共享的插件。",
        "No plugins available in this marketplace" => "此市场源中没有可用插件",
        "No plugins available in this marketplace." => "此市场源中没有可用插件。",
        "Select the plugins you want to use and press Enter to install or view details." => {
            "选择要使用的插件，然后按 Enter 安装或查看详情。"
        }
        "Type to search plugins" => "输入文字搜索插件",
        "Add Marketplace" => "添加市场源",
        "Add marketplace" => "添加市场源",
        "Add a marketplace from a Git repo or local root." => "从 Git 仓库或本地根目录添加市场源。",
        "Enter a source to make its plugins available in this menu." => {
            "输入来源，使其中的插件出现在此菜单中。"
        }
        "Enter owner/repo, a Git URL, or a local marketplace path." => {
            "输入 owner/repo、Git URL 或本地市场源路径。"
        }
        "Press Enter to enter a marketplace source." => "按 Enter 输入市场源。",
        "owner/repo, git URL, or local marketplace path" => "owner/repo、Git URL 或本地市场源路径",
        "Examples: owner/repo, git URL, ./marketplace" => {
            "示例：owner/repo、Git URL、./marketplace"
        }
        "Data shared with this app is subject to the app's " => "与此应用共享的数据受该应用的",
        "terms of service" => "服务条款",
        " and " => "和",
        "privacy policy" => "隐私政策",
        "Learn more" => "了解更多",
        "Installed by admin" => "由管理员安装",
        "This plugin is installed by your workspace admin." => "此插件由工作区管理员安装。",
        "Uninstall plugin" => "卸载插件",
        "Remove this plugin now." => "立即移除此插件。",
        "This remote plugin did not provide an uninstall identity." => "此远程插件未提供卸载标识。",
        "Install plugin" => "安装插件",
        "This plugin is disabled by your workspace admin." => "此插件已被工作区管理员禁用。",
        "This plugin is not installable from this marketplace." => "无法从此市场源安装该插件。",
        "Install this plugin now." => "立即安装此插件。",
        "This plugin did not provide an install location." => "此插件未提供安装位置。",
        "MCP Servers" => "MCP 服务器",
        "Auth" => "认证",
        "Version" => "版本",
        "Sharing" => "共享",
        "Disabled by admin" => "由管理员禁用",
        "Enabled by Admin" => "由管理员启用",
        "Not installable" => "不可安装",
        "Admin assigned" => "管理员分配",
        "Available" => "可安装",
        "Local" => "本地",
        "Auth on install" => "安装时认证",
        "Auth on use" => "使用时认证",
        "Listed" => "公开列出",
        "Workspace link" => "工作区链接",
        "Private" => "私有",
        "No explicit principals" => "没有明确的共享对象",
        "No plugin skills." => "此插件没有技能。",
        "No plugin apps." => "此插件没有应用。",
        "No plugin hooks." => "此插件没有钩子。",
        "No plugin MCP servers." => "此插件没有 MCP 服务器。",
        "plugin details are unavailable" => "插件详情不可用",
        "Space to disable; Enter view details." => "按 Space 禁用；按 Enter 查看详情。",
        "Space to enable; Enter view details." => "按 Space 启用；按 Enter 查看详情。",
        "Space to disable." => "按 Space 禁用。",
        "Space to enable." => "按 Space 启用。",
        "Press Enter to view plugin details." => "按 Enter 查看插件详情。",
        "Plugin details are unavailable." => "插件详情不可用。",
        "Press Enter to install or view plugin details." => "按 Enter 安装或查看插件详情。",
        "Remote plugin details are not available yet." => "远程插件详情暂不可用。",
        "ctrl + u upgrade · ctrl + r remove · space toggle · ←/→ tabs · enter details · esc close" => {
            "Ctrl+U 升级 · Ctrl+R 移除 · Space 开关 · ←/→ 切换标签 · Enter 详情 · Esc 关闭"
        }
        "ctrl + r remove · space toggle · ←/→ tabs · enter details · esc close" => {
            "Ctrl+R 移除 · Space 开关 · ←/→ 切换标签 · Enter 详情 · Esc 关闭"
        }
        "ctrl + u upgrade · space toggle · ←/→ tabs · enter details · esc close" => {
            "Ctrl+U 升级 · Space 开关 · ←/→ 切换标签 · Enter 详情 · Esc 关闭"
        }
        "space enable/disable · ←/→ select marketplace · enter view details · esc close" => {
            "Space 启用/禁用 · ←/→ 选择市场源 · Enter 查看详情 · Esc 关闭"
        }
        "Press esc to close." => "按 Esc 关闭。",
        " select" => " 选择",
        "esc close" => "Esc 关闭",
        "Already installed in this session." => "本次会话中已安装。",
        "Install the required Apps in ChatGPT to continue:" => {
            "请在 ChatGPT 中安装所需应用后继续："
        }
        "Manage on ChatGPT" => "在 ChatGPT 中管理",
        "Install on ChatGPT" => "在 ChatGPT 中安装",
        "Open the ChatGPT app management page" => "打开 ChatGPT 应用管理页面",
        "Open the app page in your browser." => "在浏览器中打开应用页面。",
        "ChatGPT apps link unavailable" => "ChatGPT 应用链接不可用",
        "This app did not provide an install/manage URL." => "此应用未提供安装或管理 URL。",
        "Continue" => "继续",
        "This app is already installed." => "此应用已经安装。",
        "Advance to the next app." => "继续下一个应用。",
        "I've installed it" => "我已安装",
        "Trust your confirmation and continue to the next app." => "接受你的确认并继续下一个应用。",
        "Continue without waiting for refresh to complete." => "不等待刷新完成，直接继续。",
        "Skip remaining app setup" => "跳过剩余应用设置",
        "Stop this follow-up flow for this plugin." => "停止此插件的后续设置流程。",
        "Abandon remaining required app setup." => "放弃其余所需应用设置。",

        // MCP inventory chrome. Server/tool/resource names, arguments, URLs,
        // environment variables, and tool output remain verbatim.
        "tool result (image output)" => "工具结果（图片输出）",
        "Unknown" => "未知",
        "Unsupported" => "不支持",
        "Not logged in" => "未登录",
        "Bearer token" => "Bearer Token",
        "    See the " => "    请参阅",
        "MCP docs" => "MCP 文档",
        " to configure them." => "进行配置。",
        "MCP Tools" => "MCP 工具",
        "  • No MCP servers configured." => "  • 尚未配置 MCP 服务器。",
        "  • No MCP tools available." => "  • 没有可用的 MCP 工具。",
        "(disabled)" => "（已禁用）",
        "    • Reason: " => "    • 原因：",
        "    • Status: " => "    • 状态：",
        "enabled" => "已启用",
        "    • Auth: " => "    • 认证：",
        "    • Command: " => "    • 命令：",
        "    • Cwd: " => "    • 当前目录：",
        "    • Env: " => "    • 环境变量：",
        "    • HTTP headers: " => "    • HTTP 请求头：",
        "    • Env HTTP headers: " => "    • 环境变量 HTTP 请求头：",
        "    • Tools: (none)" => "    • 工具：（无）",
        "    • Tools: " => "    • 工具：",
        "    • Resources: (none)" => "    • 资源：（无）",
        "    • Resources: " => "    • 资源：",
        "    • Resource templates: (none)" => "    • 资源模板：（无）",
        "    • Resource templates: " => "    • 资源模板：",
        "Loading MCP inventory" => "正在加载 MCP 清单",
        "Select Syntax Theme" => "选择语法高亮主题",
        "Move up/down to live preview themes" => "上下移动可实时预览主题",
        "Select Pet" => "选择终端宠物",
        "Choose a pet to wake in the terminal." => "选择要在终端中唤醒的宠物。",
        "Disable terminal pets" => "关闭终端宠物",

        // Terminal title and status-line configuration. IDs remain unchanged
        // in configuration; only their picker labels are translated.
        "Configure Status Line" => "配置状态栏",
        "Select which items to display in the status line." => "选择状态栏中显示的项目。",
        "Configure Terminal Title" => "配置终端标题",
        "Select which items to display in the terminal title." => "选择终端标题中显示的项目。",
        "Use theme colors" => "使用主题颜色",
        "Apply colors from the active /theme" => "使用当前 /theme 的颜色",
        "app-name" => "Codex 应用名称",
        "project-name" => "项目名称",
        "current-dir" => "当前工作目录",
        "activity" => "活动指示器",
        "run-state" => "运行状态",
        "thread-title" => "任务标题",
        "git-branch" => "Git 分支",
        "pull-request-number" => "拉取请求编号",
        "branch-changes" => "分支修改",
        "permissions" => "权限方案",
        "approval-mode" => "批准模式",
        "context-remaining" => "剩余上下文",
        "context-used" => "已用上下文",
        "five-hour-limit" => "主要用量限额",
        "weekly-limit" => "次要用量限额",
        "codex-version" => "Codex 版本",
        "context-window-size" => "上下文窗口大小",
        "used-tokens" => "已用 Token 总数",
        "total-input-tokens" => "输入 Token 总数",
        "total-output-tokens" => "输出 Token 总数",
        "thread-id" => "线程 ID",
        "fast-mode" => "快速模式",
        "raw-output" => "原始回滚模式",
        "model" => "模型",
        "model-with-reasoning" => "模型和推理强度",
        "reasoning" => "推理强度",
        "workspace-headline" => "工作区通知",
        "task-progress" => "任务进度",
        "Codex app name" => "Codex 应用名称",
        "Project name (falls back to current directory name)" => {
            "项目名称（不可用时显示当前目录名）"
        }
        "Current working directory" => "当前工作目录",
        "Spinner while working, action-required message while blocked." => {
            "工作时显示动画，等待操作时显示提醒。"
        }
        "Compact session run-state text (Ready, Working, Thinking)" => {
            "简短的会话运行状态（就绪、工作中、思考中）"
        }
        "Current thread title, or thread identifier when unnamed" => {
            "当前任务标题；未命名时显示线程标识符"
        }
        "Current Git branch (omitted when unavailable)" => "当前 Git 分支（不可用时省略）",
        "Current model name" => "当前模型名称",
        "Current model name with reasoning level" => "当前模型名称和推理强度",
        "Current reasoning level" => "当前推理强度",
        "Project name (omitted when unavailable)" => "项目名称（不可用时省略）",
        "Open pull request number for the current branch (omitted when unavailable)" => {
            "当前分支的拉取请求编号（不可用时省略）"
        }
        "Committed branch changes against the default branch (omitted when unavailable)" => {
            "相对默认分支的已提交修改（不可用时省略）"
        }
        "Active permission profile or sandbox mode" => "当前权限方案或沙箱模式",
        "Active command approval mode" => "当前命令批准模式",
        "Percentage of context window remaining (omitted when unknown)" => {
            "上下文窗口剩余百分比（未知时省略）"
        }
        "Percentage of context window used (omitted when unknown)" => {
            "上下文窗口已用百分比（未知时省略）"
        }
        "Remaining usage on the primary usage limit (omitted when unavailable)" => {
            "主要用量限额的剩余额度（不可用时省略）"
        }
        "Remaining usage on the secondary usage limit (omitted when unavailable)" => {
            "次要用量限额的剩余额度（不可用时省略）"
        }
        "Codex application version" => "Codex 应用版本",
        "Total context window size in tokens (omitted when unknown)" => {
            "上下文窗口的 Token 总容量（未知时省略）"
        }
        "Total tokens used in session (omitted when zero)" => "会话已用 Token 总数（为零时省略）",
        "Total input tokens used in session" => "会话已用输入 Token 总数",
        "Total output tokens used in session" => "会话已用输出 Token 总数",
        "Current thread identifier (omitted until thread starts)" => {
            "当前线程标识符（线程开始前省略）"
        }
        "Whether Fast mode is currently active" => "快速模式当前是否启用",
        "Whether raw scrollback mode is active" => "原始回滚模式是否启用",
        "Workspace notification headline (Enterprise workspaces only; omitted when unavailable)" => {
            "工作区通知标题（仅企业工作区；不可用时省略）"
        }
        "Latest task progress from update_plan (omitted until available)" => {
            "update_plan 的最新任务进度（出现前省略）"
        }

        _ => return None,
    })
}

#[cfg(test)]
#[path = "i18n_tests.rs"]
mod tests;
