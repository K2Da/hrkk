use super::SceneBase;
use crate::help::Helps;
use crate::show::Texts;
use crate::ui::key_handler::popup;
use crate::ui::widget;
use crate::ui::{layout, NextScene};
use rustbox::keyboard::Key;
use tui::backend::RustboxBackend;
use tui::terminal::Frame;

#[derive(Clone)]
pub(crate) struct Scene {
    pub(crate) base: super::SceneBase,
    offset: u16,
    texts: Texts,
    text_block: widget::PopupText,
}

pub(crate) fn new(base: SceneBase, title: &str, texts: Texts) -> Scene {
    Scene {
        base,
        offset: 0,
        texts,

        text_block: widget::popup_text::new(
            title,
            Helps::new(popup::helps()).to_summary_text(),
            true,
        ),
    }
}

impl Scene {
    pub(crate) fn handle_events(&mut self, key: Vec<Key>) -> NextScene {
        popup(
            key,
            &mut self.base,
            &mut self.offset,
            self.text_block.line_len,
        )
    }

    pub(crate) fn draw(&mut self, f: &mut Frame<RustboxBackend>) {
        let area = layout::popup_with_help::layout(80, 80, f.size());

        self.text_block.draw(f, area, &self.texts, self.offset);
    }
}
