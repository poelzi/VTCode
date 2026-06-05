use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr as _;

use crate::{Panel, PanelChrome, WidgetPalette};

#[derive(Clone, Debug)]
pub struct FooterProps {
    pub status_left: Line<'static>,
    pub status_right: Option<Line<'static>>,
    pub hint: Option<Line<'static>>,
    pub palette: WidgetPalette,
    pub chrome: PanelChrome,
}

pub struct FooterWidget {
    props: FooterProps,
}

impl FooterWidget {
    pub fn new(props: FooterProps) -> Self {
        Self { props }
    }
}

impl Widget for FooterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        Clear.render(area, buf);
        let inner =
            Panel::new(self.props.palette, self.props.chrome).render_and_get_inner(area, buf);
        if inner.height == 0 || inner.width == 0 {
            return;
        }

        let mut lines = vec![align_status_line(
            self.props.status_left,
            self.props.status_right,
            inner.width,
        )];

        if inner.height >= 2
            && let Some(hint) = self.props.hint
        {
            lines.push(hint);
        }

        Paragraph::new(lines)
            .style(self.props.palette.base)
            .render(inner, buf);
    }
}

fn align_status_line(
    left: Line<'static>,
    right: Option<Line<'static>>,
    width: u16,
) -> Line<'static> {
    let Some(right) = right else {
        return left;
    };

    let left_width = line_width(&left);
    let right_width = line_width(&right);
    let available = usize::from(width);
    if left_width + right_width + 2 > available {
        return left;
    }

    let padding = available.saturating_sub(left_width + right_width);
    let mut spans = left.spans;
    spans.push(Span::raw(" ".repeat(padding)));
    spans.extend(right.spans);
    Line::from(spans)
}

fn line_width(line: &Line<'_>) -> usize {
    line.spans
        .iter()
        .map(|span| span.content.as_ref().width())
        .sum()
}
