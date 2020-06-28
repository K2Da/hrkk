use super::SceneBase;
use crate::help::Helps;
use crate::show::Section;
use crate::ui::key_handler::popup;
use crate::ui::UiState;
use crate::ui::{layout, NextScene};
use crate::ui::{widget, ViewerMode};
use rustbox::keyboard::Key;
use tui::backend::RustboxBackend;
use tui::terminal::Frame;

#[derive(Clone)]
pub(crate) struct Scene {
    pub(crate) base: super::SceneBase,
    section: Section,
    offset: u16,
    text_block: widget::PopupText,
}

pub(crate) fn new(base: SceneBase, title: &str, section: Section) -> Scene {
    Scene {
        base,
        offset: 0,
        section,

        text_block: widget::popup_text::new(title, Helps::new(popup::helps()).to_summary_text()),
    }
}

impl Scene {
    pub(crate) fn handle_events(&mut self, key: Option<Key>) -> NextScene {
        popup(
            key,
            &mut self.base,
            &mut self.offset,
            self.text_block.line_len,
        )
    }

    pub(in crate::ui) fn draw(&mut self, ui_state: &mut UiState, f: &mut Frame<RustboxBackend>) {
        let area = layout::popup_with_help::layout(80, 80, f.size());

        self.text_block.draw(
            f,
            area,
            &match ui_state.viewer_mode {
                ViewerMode::Yaml => self.section.print_all_yaml(area.1.width as isize),
                ViewerMode::Summary => self.section.print_summary(area.1.width as isize),
            },
            self.offset,
        );
    }
}
