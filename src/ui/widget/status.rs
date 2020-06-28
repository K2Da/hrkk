use crate::show::Texts;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Paragraph, Text},
};

#[derive(Clone)]
pub(crate) struct Status {}

pub(crate) fn new() -> Status {
    Status {}
}

impl Status {
    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect, line: Texts)
    where
        B: Backend,
    {
        let mut texts = vec![Text::raw("☁ ️")];
        texts.append(&mut line.to_tui_texts().0);
        let widget = Paragraph::new(texts.iter())
            .wrap(false)
            .raw(false)
            .scroll(0);
        f.render_widget(widget, area);
    }
}
