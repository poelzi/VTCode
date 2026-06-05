use ratatui::text::{Line, Span};
use vtcode_tui_widgets::{
    LayoutMode, PanelChrome, SidebarProps, SidebarSectionBody, SidebarSectionProps,
};

use super::super::{Session, terminal_capabilities};

pub(crate) fn present(session: &mut Session, mode: LayoutMode) -> SidebarProps {
    let mut sections = Vec::new();

    let queue_items = if let Some(cached) = &session.queued_inputs_preview_cache {
        cached.clone()
    } else {
        let items: Vec<String> = session
            .queued_inputs
            .iter()
            .take(5)
            .map(|input| {
                let preview: String = input.chars().take(50).collect();
                if input.len() > 50 {
                    format!("{preview}...")
                } else {
                    preview
                }
            })
            .collect();
        session.queued_inputs_preview_cache = Some(items.clone());
        items
    };

    if !queue_items.is_empty() {
        let lines = queue_items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                Line::from(vec![
                    Span::styled(format!("{}. ", index + 1), session.styles.accent_style()),
                    Span::styled(item.clone(), session.styles.default_style()),
                ])
            })
            .collect();
        sections.push(SidebarSectionProps {
            chrome: section_chrome(session, "Queue", mode),
            body: SidebarSectionBody::List(lines),
        });
    }

    let context = session
        .input_status_right
        .as_deref()
        .unwrap_or("Ready")
        .to_string();
    sections.push(SidebarSectionProps {
        chrome: section_chrome(session, "Context", mode),
        body: SidebarSectionBody::Paragraph(vec![Line::from(Span::styled(
            context,
            session.styles.default_style(),
        ))]),
    });

    SidebarProps {
        sections,
        palette: session.styles.widget_palette(),
        mode,
    }
}

fn section_chrome(session: &Session, title: &str, mode: LayoutMode) -> PanelChrome {
    PanelChrome {
        title: Some(Line::from(Span::styled(
            title.to_string(),
            session.section_title_style(),
        ))),
        active: false,
        show_border: mode.show_borders(),
        border_type: terminal_capabilities::get_border_type(),
    }
}
