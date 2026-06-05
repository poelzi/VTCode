use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Clear, Paragraph, Widget},
};

use crate::{Panel, PanelChrome, WidgetPalette};

#[derive(Clone, Debug)]
pub struct TranscriptProps {
    pub lines: Vec<Line<'static>>,
    pub palette: WidgetPalette,
    pub chrome: PanelChrome,
    pub clear_before_render: bool,
    pub fill_backgrounds: bool,
}

pub struct TranscriptWidget {
    props: TranscriptProps,
}

impl TranscriptWidget {
    pub fn new(props: TranscriptProps) -> Self {
        Self { props }
    }

    pub fn inner_area_for(area: Rect, props: &TranscriptProps) -> Rect {
        Panel::new(props.palette, props.chrome.clone()).inner_area(area)
    }
}

impl Widget for TranscriptWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let panel = Panel::new(self.props.palette, self.props.chrome);
        let inner = panel.render_and_get_inner(area, buf);
        if inner.height == 0 || inner.width == 0 {
            return;
        }

        if self.props.clear_before_render {
            Clear.render(inner, buf);
        }

        Paragraph::new(self.props.lines.clone())
            .style(self.props.palette.base)
            .render(inner, buf);

        if self.props.fill_backgrounds {
            apply_full_width_line_backgrounds(buf, inner, &self.props.lines);
        }
    }
}

fn line_background(line: &Line<'_>) -> Option<Color> {
    line.spans.iter().find_map(|span| span.style.bg)
}

fn apply_full_width_line_backgrounds(buf: &mut Buffer, area: Rect, lines: &[Line<'_>]) {
    let max_rows = usize::from(area.height).min(lines.len());
    for (row, line) in lines.iter().take(max_rows).enumerate() {
        if let Some(bg) = line_background(line) {
            buf.set_style(
                Rect::new(area.x, area.y + row as u16, area.width, 1),
                Style::default().bg(bg),
            );
        }
    }
}
