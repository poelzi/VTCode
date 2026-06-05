use ratatui::style::Style;

#[derive(Clone, Copy, Debug, Default)]
pub struct WidgetPalette {
    pub base: Style,
    pub muted: Style,
    pub accent: Style,
    pub border: Style,
    pub border_active: Style,
    pub selected: Style,
    pub input_background: Style,
    pub link: Style,
}
