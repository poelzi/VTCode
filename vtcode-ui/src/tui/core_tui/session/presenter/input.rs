use ratatui::{layout::Rect, text::Line};
use vtcode_tui_widgets::{CursorProps, InputProps, InputWidget, PanelChrome};

use super::super::{Session, terminal_capabilities};

pub(crate) struct InputPresentation {
    pub(crate) props: InputProps,
    pub(crate) inner_area: Rect,
}

pub(crate) fn present(session: &Session, area: Rect) -> InputPresentation {
    let shell_mode_title = session
        .shell_mode_border_title()
        .map(|title| Line::raw(title.to_string()));
    let show_border = shell_mode_title.is_some();
    let mut props = InputProps {
        text: ratatui::text::Text::default(),
        status_line: if area.height > 1 {
            session
                .build_input_status_widget_data(area.width)
                .map(Line::from)
        } else {
            None
        },
        cursor: CursorProps::default(),
        padding: session.input_block_padding(),
        palette: session.styles.widget_palette(),
        chrome: PanelChrome {
            title: shell_mode_title,
            active: show_border,
            show_border,
            border_type: terminal_capabilities::get_border_type(),
        },
    };
    let inner_area = InputWidget::inner_area_for(area, &props);
    let widget_data = session.build_input_widget_data(inner_area.width, inner_area.height);
    props.text = widget_data.text;
    props.cursor = CursorProps {
        x: widget_data.cursor_x,
        y: widget_data.cursor_y,
        visible: widget_data.cursor_should_be_visible,
        use_fake_cursor: widget_data.use_fake_cursor,
    };

    InputPresentation { props, inner_area }
}
