use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::WidgetPalette;

#[derive(Clone, Debug)]
pub struct PanelChrome {
    pub title: Option<Line<'static>>,
    pub active: bool,
    pub show_border: bool,
    pub border_type: BorderType,
}

impl Default for PanelChrome {
    fn default() -> Self {
        Self {
            title: None,
            active: false,
            show_border: true,
            border_type: BorderType::Plain,
        }
    }
}

#[derive(Clone)]
pub struct Panel {
    palette: WidgetPalette,
    chrome: PanelChrome,
}

impl Panel {
    pub fn new(palette: WidgetPalette, chrome: PanelChrome) -> Self {
        Self { palette, chrome }
    }

    pub fn inner_area(&self, area: Rect) -> Rect {
        self.build_block().map_or(area, |block| block.inner(area))
    }

    pub fn render_and_get_inner(&self, area: Rect, buf: &mut Buffer) -> Rect {
        let inner = self.inner_area(area);
        if let Some(block) = self.build_block() {
            block.render(area, buf);
        }
        inner
    }

    fn build_block(&self) -> Option<Block<'static>> {
        if !self.chrome.show_border {
            return None;
        }

        let border_style = if self.chrome.active {
            self.palette.border_active
        } else {
            self.palette.border
        };
        let mut block = Block::bordered()
            .border_type(self.chrome.border_type)
            .style(self.palette.base)
            .border_style(border_style);

        if let Some(title) = self.chrome.title.clone() {
            block = block.title(title);
        }

        Some(block)
    }
}

impl Widget for Panel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(block) = self.build_block() {
            block.render(area, buf);
        }
    }
}
