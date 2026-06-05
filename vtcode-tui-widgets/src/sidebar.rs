use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use crate::{LayoutMode, Panel, PanelChrome, WidgetPalette};

#[derive(Clone, Debug)]
pub enum SidebarSectionBody {
    Paragraph(Vec<Line<'static>>),
    List(Vec<Line<'static>>),
}

#[derive(Clone, Debug)]
pub struct SidebarSectionProps {
    pub chrome: PanelChrome,
    pub body: SidebarSectionBody,
}

#[derive(Clone, Debug)]
pub struct SidebarProps {
    pub sections: Vec<SidebarSectionProps>,
    pub palette: WidgetPalette,
    pub mode: LayoutMode,
}

pub struct SidebarWidget {
    props: SidebarProps,
}

impl SidebarWidget {
    pub fn new(props: SidebarProps) -> Self {
        Self { props }
    }
}

impl Widget for SidebarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 || !self.props.mode.allow_sidebar() {
            return;
        }

        Clear.render(area, buf);
        let constraints = section_constraints(self.props.sections.len());
        let chunks = Layout::vertical(constraints).split(area);

        for (section, section_area) in self.props.sections.into_iter().zip(chunks.iter()) {
            let inner = Panel::new(self.props.palette, section.chrome)
                .render_and_get_inner(*section_area, buf);
            if inner.height == 0 || inner.width == 0 {
                continue;
            }

            match section.body {
                SidebarSectionBody::Paragraph(lines) => {
                    Paragraph::new(lines)
                        .style(self.props.palette.base)
                        .wrap(Wrap { trim: true })
                        .render(inner, buf);
                }
                SidebarSectionBody::List(lines) => {
                    Paragraph::new(lines)
                        .style(self.props.palette.base)
                        .wrap(Wrap { trim: false })
                        .render(inner, buf);
                }
            }
        }
    }
}

fn section_constraints(count: usize) -> Vec<Constraint> {
    match count {
        0 => vec![Constraint::Percentage(100)],
        1 => vec![Constraint::Percentage(100)],
        2 => vec![Constraint::Percentage(50), Constraint::Percentage(50)],
        3 => vec![
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
        _ => vec![Constraint::Percentage((100 / count).max(1) as u16); count],
    }
}
