use clap::Parser;
use codex_arg0::Arg0DispatchPaths;
use codex_arg0::arg0_dispatch_or_else;
use codex_config::LoaderOverrides;
use codex_tui::AppExitInfo;
use codex_tui::Cli;
use codex_tui::ExitReason;
use codex_tui::run_main;
use codex_utils_cli::CliConfigOverrides;
use std::io::Write;
use supports_color::Stream;

const UI_LOCALE_ENV: &str = "CODEX_UI_LOCALE";

fn format_exit_messages(exit_info: AppExitInfo, color_enabled: bool) -> Vec<String> {
    let is_fatal = matches!(&exit_info.exit_reason, ExitReason::Fatal(_));
    let AppExitInfo {
        token_usage,
        thread_id,
        resume_hint,
        ..
    } = exit_info;

    let mut lines = Vec::new();
    if !token_usage.is_zero() {
        lines.push(token_usage.to_string());
    }

    if let Some(resume_cmd) = resume_hint {
        let resume_cmd = resume_cmd.replacen("codex resume", "codex-zh resume", 1);
        let command = if color_enabled {
            format!("\u{1b}[36m{resume_cmd}\u{1b}[39m")
        } else {
            resume_cmd
        };
        lines.push(format!("若要继续此会话，请运行 {command}"));
    } else if is_fatal && let Some(thread_id) = thread_id {
        lines.push(format!("会话 ID：{thread_id}"));
    }

    lines
}

/// Codex 中文交互客户端。
///
/// 这里只提供中文交互界面以及 resume、fork 两个会话入口；管理类和非交互命令继续使用
/// 未改动的官方 `codex`。
#[derive(Debug, Parser)]
#[command(
    author,
    name = "codex-zh",
    version,
    subcommand_negates_reqs = true,
    bin_name = "codex-zh",
    override_usage = "codex-zh [OPTIONS] [PROMPT]\n       codex-zh [OPTIONS] <COMMAND> [ARGS]"
)]
struct TopCli {
    #[clap(flatten)]
    config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    interactive: Cli,

    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    /// 继续以前的交互会话。
    Resume(ResumeCommand),

    /// 从以前的交互会话创建分支。
    Fork(ForkCommand),
}

#[derive(Debug, Parser)]
struct ResumeCommand {
    /// 会话 ID（UUID）或会话名称。
    #[arg(value_name = "SESSION_ID")]
    session_id: Option<String>,

    /// 不显示选择器，直接继续最近一次会话。
    #[arg(long = "last", default_value_t = false)]
    last: bool,

    /// 显示全部会话，不按当前目录筛选。
    #[arg(long = "all", default_value_t = false)]
    all: bool,

    /// 在选择器和 --last 的范围中包括非交互会话。
    #[arg(long = "include-non-interactive", default_value_t = false)]
    include_non_interactive: bool,

    #[clap(flatten)]
    interactive: Cli,
}

#[derive(Debug, Parser)]
struct ForkCommand {
    /// 会话 ID（UUID）或会话名称。
    #[arg(value_name = "SESSION_ID")]
    session_id: Option<String>,

    /// 不显示选择器，直接从最近一次会话创建分支。
    #[arg(long = "last", default_value_t = false)]
    last: bool,

    /// 显示全部会话，不按当前目录筛选。
    #[arg(long = "all", default_value_t = false)]
    all: bool,

    #[clap(flatten)]
    interactive: Cli,
}

fn configure_interactive(top_cli: TopCli) -> Cli {
    let mut inner = match top_cli.subcommand {
        None => top_cli.interactive,
        Some(Subcommand::Resume(command)) => {
            let mut inner = command.interactive;
            let resume_session_id = if command.last && inner.prompt.is_none() {
                inner.prompt = command.session_id;
                None
            } else {
                command.session_id
            };
            inner.resume_picker = resume_session_id.is_none() && !command.last;
            inner.resume_last = command.last;
            inner.resume_session_id = resume_session_id;
            inner.resume_show_all = command.all;
            inner.resume_include_non_interactive = command.include_non_interactive;
            inner
        }
        Some(Subcommand::Fork(command)) => {
            let mut inner = command.interactive;
            let fork_session_id = if command.last && inner.prompt.is_none() {
                inner.prompt = command.session_id;
                None
            } else {
                command.session_id
            };
            inner.fork_picker = fork_session_id.is_none() && !command.last;
            inner.fork_last = command.last;
            inner.fork_session_id = fork_session_id;
            inner.fork_show_all = command.all;
            inner
        }
    };

    inner
        .config_overrides
        .raw_overrides
        .splice(0..0, top_cli.config_overrides.raw_overrides);
    inner
}

fn main() -> anyhow::Result<()> {
    // SAFETY: this is the first operation in the process, before the async
    // runtime or any worker threads can read the environment.
    unsafe { std::env::set_var(UI_LOCALE_ENV, "zh-CN") };

    arg0_dispatch_or_else(|arg0_paths: Arg0DispatchPaths| async move {
        let inner = configure_interactive(TopCli::parse());
        let exit_info = run_main(
            inner,
            arg0_paths,
            LoaderOverrides::default(),
            /* explicit_remote_endpoint */ None,
        )
        .await?;
        let is_fatal = match &exit_info.exit_reason {
            ExitReason::Fatal(message) => {
                eprintln!("错误：{message}");
                true
            }
            ExitReason::UserRequested => false,
        };

        let color_enabled = supports_color::on(Stream::Stdout).is_some();
        for line in format_exit_messages(exit_info, color_enabled) {
            println!("{line}");
        }
        if is_fatal {
            std::io::stdout().flush()?;
            std::process::exit(1);
        }
        Ok(())
    })
}
