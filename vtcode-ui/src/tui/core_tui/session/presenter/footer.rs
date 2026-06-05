use ratatui::{
    style::Modifier,
    text::{Line, Span},
};
use vtcode_tui_widgets::{FooterProps, LayoutMode, PanelChrome};

use super::super::{Session, status_requires_shimmer};
use crate::tui::core_tui::language_badge::language_badge_style;
use crate::tui::core_tui::widgets::footer_hints;

pub(crate) fn present(
    session: &Session,
    mode: LayoutMode,
    hint_override: Option<&'static str>,
) -> FooterProps {
    let left_status = session.input_status_left.as_deref().unwrap_or("");
    let right_status = session.input_status_right.as_deref().unwrap_or("");
    let hint = if let Some(hint) = hint_override {
        hint
    } else if session.thinking_spinner.is_active {
        footer_hints::PROCESSING
    } else if session.has_active_overlay() {
        footer_hints::MODAL
    } else if session.input_manager.content().is_empty() {
        footer_hints::IDLE
    } else {
        footer_hints::EDITING
    };

    FooterProps {
        status_left: build_left_status(session, left_status),
        status_right: build_right_status(session, right_status),
        hint: (!matches!(mode, LayoutMode::Compact))
            .then(|| Line::from(Span::styled(hint.to_string(), session.styles.muted_style()))),
        palette: session.styles.widget_palette(),
        chrome: PanelChrome {
            title: None,
            active: false,
            show_border: false,
            border_type: super::super::terminal_capabilities::get_border_type(),
        },
    }
}

fn build_left_status(session: &Session, left_status: &str) -> Line<'static> {
    if left_status.is_empty() {
        return Line::default();
    }

    let style = if status_requires_shimmer(left_status) {
        session.styles.muted_style()
    } else {
        session.styles.accent_style()
    };
    Line::from(Span::styled(left_status.to_string(), style))
}

fn build_right_status(session: &Session, status: &str) -> Option<Line<'static>> {
    if status.is_empty() {
        return None;
    }

    let mut spans = Vec::new();
    let mut parts = status.split(" | ").peekable();
    while let Some(part) = parts.next() {
        let style = language_badge_style(part).unwrap_or_else(|| session.styles.muted_style());
        spans.push(Span::styled(part.to_string(), style));
        if parts.peek().is_some() {
            spans.push(Span::styled(
                " | ".to_string(),
                session.styles.muted_style().add_modifier(Modifier::DIM),
            ));
        }
    }

    Some(Line::from(spans))
}
