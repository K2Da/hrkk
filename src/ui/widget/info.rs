use crate::color;
use crate::ui::{UiState, ViewerMode};
use tui::style::Style;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::Modifier,
    terminal::Frame,
    widgets::{Paragraph, Text},
};

#[derive(Clone)]
pub(crate) struct Info {}

pub(crate) fn new() -> Info {
    Info {}
}

impl Info {
    pub(in crate::ui) fn draw<B>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        region: &str,
        view_scroll: u16,
        line_len: u16,
        show_viewer: bool,
        ui_state: &UiState,
    ) where
        B: Backend,
    {
        let active = Style::default().fg(color::ACTIVE).modifier(Modifier::BOLD);
        let inactive = Style::default().fg(color::INACTIVE);

        let (summary_style, yaml_style) = match ui_state.viewer_mode {
            ViewerMode::Summary => (active, inactive),
            ViewerMode::Yaml => (inactive, active),
        };

        let mut text = vec![
            Text::raw(format!(
                "request: {} / region: {}\n",
                ui_state.api_count, region,
            )),
            Text::raw("\n"),
            Text::raw(format!("({}/{}) ", view_scroll, line_len)),
        ];

        if show_viewer {
            text.append(&mut vec![
                Text::raw("["),
                Text::styled("Y", Style::default().fg(color::BUTTON)),
                Text::raw("] "),
                Text::styled("summary", summary_style),
                Text::raw(" | "),
                Text::styled("yaml", yaml_style),
            ]);
        }

        let guide = Paragraph::new(text.iter()).alignment(Alignment::Right);
        f.render_widget(guide, area);
    }
}
