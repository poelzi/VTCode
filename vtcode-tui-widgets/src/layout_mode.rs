use ratatui::layout::Rect;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutMode {
    Compact,
    Standard,
    Wide,
}

impl LayoutMode {
    pub fn from_area(area: Rect) -> Self {
        if area.width < 80 || area.height < 20 {
            Self::Compact
        } else if area.width >= 120 && area.height >= 24 {
            Self::Wide
        } else {
            Self::Standard
        }
    }

    pub fn show_borders(self) -> bool {
        !matches!(self, Self::Compact)
    }

    pub fn allow_sidebar(self) -> bool {
        matches!(self, Self::Wide)
    }

    pub fn show_logs_panel(self) -> bool {
        !matches!(self, Self::Compact)
    }

    pub fn footer_height(self) -> u16 {
        0
    }

    pub fn show_footer(self) -> bool {
        false
    }

    pub fn max_header_percent(self) -> f32 {
        match self {
            Self::Compact => 0.15,
            Self::Standard => 0.25,
            Self::Wide => 0.30,
        }
    }

    pub fn sidebar_width_percent(self) -> u16 {
        match self {
            Self::Wide => 28,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutMode;
    use ratatui::layout::Rect;

    #[test]
    fn derives_mode_from_viewport() {
        assert_eq!(
            LayoutMode::from_area(Rect::new(0, 0, 60, 15)),
            LayoutMode::Compact
        );
        assert_eq!(
            LayoutMode::from_area(Rect::new(0, 0, 100, 30)),
            LayoutMode::Standard
        );
        assert_eq!(
            LayoutMode::from_area(Rect::new(0, 0, 150, 40)),
            LayoutMode::Wide
        );
    }
}
