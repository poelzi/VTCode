use ratatui::layout::Rect;
use vtcode_tui_widgets::{LayoutMode, PanelChrome, TranscriptProps, TranscriptWidget};

use super::super::{Session, TranscriptLine, terminal_capabilities};
use crate::tui::config::constants::ui;

pub(crate) struct TranscriptPresentation {
    pub(crate) props: TranscriptProps,
}

pub(crate) fn present(
    session: &mut Session,
    area: Rect,
    mode: LayoutMode,
) -> TranscriptPresentation {
    let mut props = TranscriptProps {
        lines: Vec::new(),
        palette: session.styles.widget_palette(),
        chrome: PanelChrome {
            title: None,
            active: false,
            show_border: mode.show_borders(),
            border_type: terminal_capabilities::get_border_type(),
        },
        clear_before_render: false,
        fill_backgrounds: true,
    };

    if area.height == 0 || area.width == 0 {
        session.set_transcript_area(None);
        session.clear_transcript_file_link_targets();
        return TranscriptPresentation { props };
    }

    let inner = TranscriptWidget::inner_area_for(area, &props);
    if inner.height == 0 || inner.width == 0 {
        session.set_transcript_area(None);
        session.clear_transcript_file_link_targets();
        return TranscriptPresentation { props };
    }

    session.set_transcript_area(Some(inner));

    let effective_height = inner.height.min(ui::TUI_MAX_VIEWPORT_HEIGHT);
    let effective_width = inner.width.min(ui::TUI_MAX_VIEWPORT_WIDTH);
    session.apply_transcript_rows(effective_height);
    if effective_width == 0 {
        session.clear_transcript_file_link_targets();
        return TranscriptPresentation { props };
    }
    session.apply_transcript_width(effective_width);

    let viewport_rows = effective_height as usize;
    let padding = usize::from(ui::INLINE_TRANSCRIPT_BOTTOM_PADDING);
    let effective_padding = padding.min(viewport_rows.saturating_sub(1));
    let total_rows = session.total_transcript_rows(effective_width) + effective_padding;
    let (top_offset, _) = session.prepare_transcript_scroll(total_rows, viewport_rows);
    let vertical_offset = top_offset.min(session.scroll_manager.max_offset());
    session.transcript_view_top = vertical_offset;

    let cached_lines =
        session.collect_transcript_window_cached(effective_width, vertical_offset, viewport_rows);
    let fill_count = viewport_rows.saturating_sub(cached_lines.len());
    let needs_mutation = fill_count > 0 || !session.queued_inputs.is_empty();

    let visible_lines = if needs_mutation {
        let mut lines = cached_lines.to_vec();
        if fill_count > 0 {
            let target_len = lines.len() + fill_count;
            lines.resize_with(target_len, TranscriptLine::default);
        }
        session.overlay_queue_lines(&mut lines, effective_width);
        lines
    } else {
        cached_lines.to_vec()
    };

    props.lines = session.decorate_visible_transcript_links(visible_lines, inner);
    props.clear_before_render = session.transcript_content_changed;
    session.transcript_content_changed = false;

    TranscriptPresentation { props }
}
