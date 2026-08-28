//! Render file-change approval descriptions and cross-platform destinations.

use super::approval_overlay::ApplyPatchApprovalRequest;
use crate::diff_model::FileChange;
use crate::render::renderable::Renderable;
use codex_utils_path_uri::LegacyAppPathString;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;

pub(super) fn build_header(request: &ApplyPatchApprovalRequest) -> Box<dyn Renderable> {
    let mut header = Vec::new();
    if let Some(thread_label) = &request.thread_label {
        header.push(Line::from(vec![
            crate::i18n::tr_static("Thread: ").into(),
            thread_label.clone().bold(),
        ]));
    }

    if !header.is_empty() {
        header.push(Line::from(""));
    }

    let description = request
        .reason
        .as_deref()
        .filter(|reason| !reason.is_empty())
        .map(std::borrow::Cow::Borrowed)
        .unwrap_or_else(|| crate::i18n::tr("Apply proposed file edits"));
    header.push(Line::from_iter([
        crate::i18n::tr_static("Description: ").into(),
        description.into_owned().italic(),
    ]));

    let mut destinations: Vec<_> = request
        .changes
        .iter()
        .flat_map(|(path, change)| {
            let move_destination = match change {
                FileChange::Update {
                    move_path: Some(move_path),
                    ..
                } => Some(move_path),
                FileChange::Add { .. }
                | FileChange::Delete { .. }
                | FileChange::Update {
                    move_path: None, ..
                } => None,
            };
            std::iter::once(path).chain(move_destination)
        })
        .map(|path| {
            LegacyAppPathString::from_path(path)
                .to_inferred_path_uri()
                .map(|path| path.inferred_native_path_string())
                .unwrap_or_else(|| request.cwd.join(path).display().to_string())
        })
        .collect();
    destinations.sort();
    destinations.dedup();
    if destinations.is_empty() {
        header.push(Line::from(vec![
            crate::i18n::tr_static("Destination: ").into(),
            crate::i18n::tr_static("unavailable").bold(),
        ]));
    }
    for destination in destinations {
        header.push(Line::from_iter([
            crate::i18n::tr_static("Destination: ").into(),
            destination.bold(),
        ]));
    }
    Box::new(Paragraph::new(header).wrap(Wrap { trim: false }))
}
