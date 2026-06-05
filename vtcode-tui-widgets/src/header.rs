use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{Panel, PanelChrome, WidgetPalette};

#[derive(Clone, Debug)]
pub struct HeaderProps {
    pub lines: Vec<Line<'static>>,
    pub palette: WidgetPalette,
    pub chrome: PanelChrome,
}

pub struct HeaderWidget {
    props: HeaderProps,
}

impl HeaderWidget {
    pub fn new(props: HeaderProps) -> Self {
        Self { props }
    }
}

impl Widget for HeaderWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        if area.height == 0 || area.width == 0 {
            return;
        }

        let inner =
            Panel::new(self.props.palette, self.props.chrome).render_and_get_inner(area, buf);
        if inner.height == 0 || inner.width == 0 {
            return;
        }

        Paragraph::new(self.props.lines)
            .style(self.props.palette.base)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
