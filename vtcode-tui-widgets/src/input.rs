use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Clear, Padding, Paragraph, Widget, Wrap},
};

use crate::{PanelChrome, WidgetPalette};

#[derive(Clone, Copy, Debug, Default)]
pub struct CursorProps {
    pub x: u16,
    pub y: u16,
    pub visible: bool,
    pub use_fake_cursor: bool,
}

#[derive(Clone, Debug)]
pub struct InputProps {
    pub text: Text<'static>,
    pub status_line: Option<Line<'static>>,
    pub cursor: CursorProps,
    pub padding: Padding,
    pub palette: WidgetPalette,
    pub chrome: PanelChrome,
}

pub struct InputWidget {
    props: InputProps,
}

impl InputWidget {
    pub fn new(props: InputProps) -> Self {
        Self { props }
    }

    pub fn inner_area_for(area: Rect, props: &InputProps) -> Rect {
        let (input_area, _) = split_input_areas(area, props.status_line.is_some());
        build_block(props).inner(input_area)
    }
}

impl Widget for InputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        if area.height == 0 || area.width == 0 {
            return;
        }

        let (input_area, status_area) = split_input_areas(area, self.props.status_line.is_some());
        let block = build_block(&self.props);
        let inner = block.inner(input_area);

        Paragraph::new(self.props.text)
            .style(self.props.palette.input_background)
            .wrap(Wrap { trim: false })
            .block(block)
            .render(input_area, buf);

        if self.props.cursor.visible && inner.width > 0 && inner.height > 0 {
            let cursor_x = self
                .props
                .cursor
                .x
                .min(inner.width.saturating_sub(1))
                .saturating_add(inner.x);
            let cursor_y = self
                .props
                .cursor
                .y
                .min(inner.height.saturating_sub(1))
                .saturating_add(inner.y);

            if let Some(cell) = buf.cell_mut((cursor_x, cursor_y))
                && self.props.cursor.use_fake_cursor
            {
                cell.set_style(cell.style().reversed());
                if cell.symbol().is_empty() {
                    cell.set_symbol(" ");
                }
            }
        }

        if let (Some(status_line), Some(status_rect)) = (self.props.status_line, status_area) {
            Paragraph::new(status_line)
                .style(self.props.palette.base)
                .wrap(Wrap { trim: false })
                .render(status_rect, buf);
        }
    }
}

fn split_input_areas(area: Rect, has_status_line: bool) -> (Rect, Option<Rect>) {
    if !has_status_line || area.height < 2 {
        return (area, None);
    }

    let block_height = area.height.saturating_sub(1).max(1);
    (
        Rect::new(area.x, area.y, area.width, block_height),
        Some(Rect::new(area.x, area.y + block_height, area.width, 1)),
    )
}

fn build_block(props: &InputProps) -> Block<'static> {
    let mut block = if props.chrome.show_border {
        Block::bordered()
    } else {
        Block::new()
    };

    block = block
        .style(props.palette.input_background)
        .padding(props.padding);

    if props.chrome.show_border {
        block = block
            .border_type(props.chrome.border_type)
            .border_style(if props.chrome.active {
                props.palette.border_active
            } else {
                props.palette.border
            });
    }

    if let Some(title) = props.chrome.title.clone() {
        block = block.title(title);
    }

    block
}
