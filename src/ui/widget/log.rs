use crate::show::Texts;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone)]
pub(crate) struct Log {
    pub(crate) scroll: u16,
}

pub(crate) fn new() -> Log {
    Log { scroll: 0 }
}

impl Log {
    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: (Rect, Rect), texts: Texts)
    where
        B: Backend,
    {
        let (tui_texts, _) = texts.to_tui_texts();
        let mut log = Paragraph::new(tui_texts.iter()).wrap(true).raw(false);

        let p = super::text_button("L");
        let guide = Paragraph::new(p.iter()).block(Block::default().borders(Borders::RIGHT));
        f.render_widget(guide, area.0);
        log = log.block(Block::default().borders(Borders::RIGHT));
        f.render_widget(log, area.1);
    }
}
