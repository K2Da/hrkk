use crate::show::Texts;
use tui::widgets::BorderType;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Clone)]
pub(in crate::ui) struct PopupText {
    title: String,
    texts: Texts,
    pub(in crate::ui) line_len: u16,
}

pub(in crate::ui) fn new(title: &str, helps: Texts) -> PopupText {
    PopupText {
        title: title.to_owned(),
        texts: helps,
        line_len: 0,
    }
}

impl PopupText {
    pub(crate) fn draw<B>(
        &mut self,
        f: &mut Frame<B>,
        area: (Rect, Rect, Rect),
        texts: &Texts,
        offset: u16,
    ) where
        B: Backend,
    {
        let (center_box, text, help) = area;
        let (paragraph_text, line_len) = texts.to_tui_texts();
        self.line_len = line_len;

        let title = format!(" {} ({}/{}) ", self.title, offset, self.line_len);
        f.render_widget(tui::widgets::Clear, center_box);
        f.render_widget(
            Paragraph::new([].iter()).block(
                Block::default()
                    .border_type(BorderType::Double)
                    .borders(Borders::ALL)
                    .title(&title),
            ),
            center_box,
        );

        let text_paragraph = Paragraph::new(paragraph_text.iter())
            .wrap(false)
            .scroll(offset);
        f.render_widget(text_paragraph, text);

        let (text_vec, _) = self.texts.to_tui_texts();
        let helps = text_vec.iter();
        let help_paragraph = Paragraph::new(helps)
            .wrap(true)
            .block(Block::default().borders(Borders::TOP));

        f.render_widget(help_paragraph, help);
    }
}
