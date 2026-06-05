use ratatui::text::Line;
use vtcode_tui_widgets::{HeaderProps, PanelChrome};

use super::super::{Session, terminal_capabilities};

pub(crate) fn present(session: &Session, lines: Vec<Line<'static>>) -> HeaderProps {
    HeaderProps {
        lines,
        palette: session.styles.widget_palette(),
        chrome: PanelChrome {
            title: Some(session.header_block_title()),
            active: false,
            show_border: true,
            border_type: terminal_capabilities::get_border_type(),
        },
    }
}
