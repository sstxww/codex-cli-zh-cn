use super::*;

#[test]
fn english_is_the_default_and_unknown_text_is_preserved() {
    assert_eq!(
        translate_for(UiLocale::EnUs, "Select Model"),
        "Select Model"
    );
    assert_eq!(
        translate_for(UiLocale::ZhCn, "deepseek-chat"),
        "deepseek-chat"
    );
}

#[test]
fn chinese_translates_fixed_and_dynamic_ui_text() {
    assert_eq!(translate_for(UiLocale::ZhCn, "Select Model"), "选择模型");
    assert_eq!(
        translate_for(UiLocale::ZhCn, "gpt-5.6-codex (current)"),
        "gpt-5.6-codex（当前）"
    );
    assert_eq!(
        translate_for(UiLocale::ZhCn, "Select Reasoning Level for gpt-5.6-codex"),
        "选择 gpt-5.6-codex 的推理强度"
    );
}

#[test]
fn locale_parser_accepts_common_simplified_chinese_spellings() {
    assert_eq!(UiLocale::from_env_value(Some("zh-CN")), UiLocale::ZhCn);
    assert_eq!(UiLocale::from_env_value(Some("zh_Hans")), UiLocale::ZhCn);
    assert_eq!(UiLocale::from_env_value(Some("en-US")), UiLocale::EnUs);
}

#[test]
fn canonical_slash_command_name_is_not_localized() {
    use crate::slash_command::SlashCommand;

    assert_eq!(SlashCommand::Model.command(), "model");
    assert_eq!(SlashCommand::Permissions.command(), "permissions");
    assert_eq!(SlashCommand::Resume.command(), "resume");
}

#[test]
fn mixed_cjk_and_ascii_width_uses_terminal_cells() {
    assert_eq!(crate::width::display_width("模型 / Model"), 12);
    assert_eq!(crate::width::display_width("中文（中国）"), 12);
}

#[test]
fn zh_cn_core_ui_labels_snapshot() {
    let labels = [
        ("/model", "Select Model"),
        ("/permissions", "Update Model Permissions"),
        ("/resume", "Resume a previous session"),
        ("/statusline", "Configure Status Line"),
        ("/title", "Configure Terminal Title"),
        ("/memories", "Memories"),
        ("/hooks", "Hooks"),
        ("/plugins", "Plugins"),
        ("/theme", "Select Syntax Theme"),
        ("/keymap", "Keymap"),
        ("empty", "no matches"),
    ];
    let rendered = labels
        .into_iter()
        .map(|(command, label)| format!("{command:<14}{}", translate_for(UiLocale::ZhCn, label)))
        .collect::<Vec<_>>()
        .join("\n");

    insta::assert_snapshot!("zh_cn_core_ui_labels", rendered);
}
