use crate::show::Texts;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone)]
pub(crate) struct Help {
    pub(crate) scroll: u16,
}

pub(crate) fn new() -> Help {
    Help { scroll: 0 }
}

impl Help {
    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: (Rect, Rect), texts: &Texts)
    where
        B: Backend,
    {
        let p = super::text_button("H");
        let guide = Paragraph::new(p.iter()).block(Block::default().borders(Borders::RIGHT));
        f.render_widget(guide, area.0);

        let (h, _) = texts.to_tui_texts();
        let help = Paragraph::new(h.iter()).wrap(true);
        f.render_widget(help, area.1);
    }
}
