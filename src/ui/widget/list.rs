use tui::widgets::ListState;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, List, Text},
};

pub struct Box {
    pub state: ListState,
    name: String,
    pub items: Vec<String>,
}

pub fn new(name: &str, items: &Vec<String>) -> Box {
    Box {
        state: ListState::default(),
        name: format!(" {} ", name),
        items: items.clone(),
    }
}

impl Box {
    pub fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let widget = List::new(self.items.iter().map(|i| Text::raw(i)))
            .block(Block::default().borders(Borders::ALL).title(&self.name))
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            .highlight_symbol(">️ ");

        f.render_stateful_widget(widget, area, &mut self.state);
    }

    pub fn selected_item(&self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            return Some(self.items[index].clone());
        }
        None
    }
}
