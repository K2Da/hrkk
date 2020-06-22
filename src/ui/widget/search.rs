use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Block, Borders, Paragraph, Text},
};

pub struct TextBox {}

pub fn new() -> TextBox {
    TextBox {}
}

impl TextBox {
    pub fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect, text: &str)
    where
        B: Backend,
    {
        let p = [Text::raw(" "), Text::raw(text)];
        let widget = Paragraph::new(p.iter())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" type small letters to search "),
            )
            .wrap(false)
            .raw(false);
        f.render_widget(widget, area);
    }
}
