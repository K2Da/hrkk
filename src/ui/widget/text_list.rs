use crate::ui::widget::util::list;
use tui::widgets::ListState;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, List, Text},
};

#[derive(Clone)]
pub(crate) struct TextList {
    pub(crate) state: ListState,
    pub(crate) name: String,
    pub(crate) items: Vec<String>,
}

pub(crate) fn new(name: &str, items: &Vec<String>) -> TextList {
    let mut list_box = TextList {
        state: ListState::default(),
        name: format!(" {} ", name),
        items: items.clone(),
    };
    list::select_any(&mut list_box.state, items.len());
    list_box
}

impl TextList {
    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let widget = List::new(self.items.iter().map(|i| Text::raw(i)))
            .block(Block::default().borders(Borders::ALL).title(&self.name))
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            .highlight_symbol(">️ ");

        f.render_widget(tui::widgets::Clear, area);
        f.render_stateful_widget(widget, area, &mut self.state);
    }

    pub(crate) fn selected_item(&self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            return Some(self.items[index].clone());
        }
        None
    }
}
