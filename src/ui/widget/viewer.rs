use crate::show;
use crate::ui::ViewerMode;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone)]
pub(crate) struct Viewer {
    pub(in crate::ui) scroll: u16,
    text: show::Section,
    pub(in crate::ui) line_len: u16,
}

pub(crate) fn new(text: show::Section) -> Viewer {
    Viewer {
        text,
        scroll: 0,
        line_len: 0,
    }
}

impl Viewer {
    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, viewer_mode: &ViewerMode, area: Rect)
    where
        B: Backend,
    {
        let printed = match viewer_mode {
            ViewerMode::Yaml => self.text.print_all_yaml(area.width as isize),
            ViewerMode::Summary => self.text.print_summary(area.width as isize),
        };

        let (texts, line_len) = printed.to_tui_texts();
        self.line_len = line_len;
        let widget = Paragraph::new(texts.iter())
            .block(Block::default().borders(Borders::BOTTOM))
            .wrap(false)
            .raw(false)
            .scroll(self.scroll);
        f.render_widget(widget, area);
    }
}
