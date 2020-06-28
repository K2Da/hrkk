use crate::error::Result;
use crate::help::{Help, Helps};
use crate::ui::{UiState, ViewerMode};
use widget::util::table;

use super::SceneBase;
use crate::ui::{layout, select_next_scene, util::event::Events, widget, NextScene, UiScene};
use rustbox::keyboard::Key;
use tui::backend::RustboxBackend;
use tui::terminal::Frame;

#[derive(Clone)]
pub(crate) struct Scene {
    pub(in crate::ui) base: super::SceneBase,
    pub(crate) search_text: String,

    status: widget::Status,
    search: widget::Search,
    table: widget::Commands,
    log: widget::Log,
    info: widget::Info,
    viewer: widget::Viewer,
    help: widget::Help,

    helps: Helps,
    help_summary: Texts,
}

pub(crate) fn new(base: SceneBase) -> Scene {
    let helps = Helps::new(all_helps());
    let help_summary = helps.to_summary_text();
    Scene {
        base,
        search_text: "".to_string(),
        status: widget::status::new(),
        search: widget::search::new(),
        table: widget::commands::new(),
        log: widget::log::new(),
        info: widget::info::new(),
        viewer: widget::viewer::new(crate::show::Section::new_without_yaml()),
        help: widget::help::new(),

        helps,
        help_summary,
    }
}

fn all_helps() -> Vec<Help> {
    let mut all_helps = vec![];
    all_helps.append(&mut helps());
    super::common_helps(&mut all_helps);
    all_helps
}

fn helps() -> Vec<Help> {
    vec![Help::new(
        "Enter",
        Some("select"),
        "select sub command to execute",
    )]
}

use crate::show::{Texts, Txt};
impl Scene {
    pub(crate) fn status(&self, current: bool) -> crate::show::Texts {
        if current {
            Texts(vec![Txt::colored("commands", crate::color::CURRENT)])
        } else {
            Texts(vec![Txt::raw("commands")])
        }
    }

    pub(in crate::ui) fn handle_events(
        &mut self,
        ui_state: &mut UiState,
        events: &mut Events,
        key: Option<rustbox::keyboard::Key>,
    ) -> Result<NextScene> {
        if let Some(overlay) = &mut self.base.overlay {
            return overlay.handle_events(ui_state, events, key);
        }

        if let Some(key) = key {
            if let Some(next_scene) = self.handle_keys(ui_state, events, key) {
                return Ok(next_scene);
            }
        }

        if table::select_any(self.table.filtered_len(), &mut self.table.state) {
            self.viewer = widget::viewer::new(self.table.command_detail());
        }

        Ok(NextScene::Same)
    }

    fn handle_keys(
        &mut self,
        ui_state: &mut UiState,
        events: &mut Events,
        key: Key,
    ) -> Option<NextScene> {
        use crate::ui::key_handler::*;

        if let Some(()) = text_input(key, &mut self.search_text) {
            self.table.filter(&self.search_text);
            return None;
        }

        if let Some(next) = common(key, &mut self.base, false) {
            return Some(next);
        }

        if let Some(row_selected) = table_move(
            key,
            self.table.filtered_len(),
            &mut self.table.state,
            self.table.last_height,
            &mut self.viewer.scroll,
            self.viewer.line_len,
        ) {
            if row_selected {
                self.viewer = widget::viewer::new(self.table.command_detail());
            }
            return None;
        }

        if let Some(popup) = text_popup_open(
            key,
            &self.base,
            &ui_state.logs,
            &self.helps,
            Box::new(UiScene::Commands(self.clone())),
        ) {
            self.overlay(UiScene::TextPopup(popup));
            return None;
        }

        if let Some(popup) = section_popup_open(
            key,
            &self.base,
            || self.table.command_detail(),
            Box::new(UiScene::Commands(self.clone())),
        ) {
            self.overlay(UiScene::SectionPopup(popup));
            return None;
        }

        match key {
            Key::Enter => {
                if let Some(resource) = self.table.selected_resource() {
                    return Some(NextScene::Scene(select_next_scene(
                        Some(Box::new(UiScene::Commands(self.clone()))),
                        &self.base.opts.clone(),
                        &None,
                        resource,
                        ui_state,
                        events.tx.clone(),
                    )));
                }
            }
            _ => (),
        }

        None
    }

    pub(in crate::ui) fn draw(
        &mut self,
        ui_state: &mut UiState,
        mut f: &mut Frame<RustboxBackend>,
    ) {
        let (status, (search, table, log), (info, viewer, help)) = layout::main::layout(f.size());

        self.status.draw(&mut f, status, self.status(true));
        self.search.draw(&mut f, search, &self.search_text);
        self.table.draw(&mut f, table);
        self.log.draw(&mut f, log, ui_state.logs.to_text(2));
        self.viewer.draw(&mut f, &ViewerMode::Summary, viewer);
        self.info.draw(
            &mut f,
            info,
            &self.base.opts.region().unwrap().name(),
            self.viewer.scroll,
            self.viewer.line_len,
            false,
            &ui_state,
        );
        self.help.draw(&mut f, help, &self.help_summary);

        if let Some(ui) = &mut self.base.overlay {
            ui.draw(ui_state, f);
        }
    }

    pub(crate) fn overlay(&mut self, other: UiScene) {
        self.base.overlay = Some(Box::new(other));
    }
}
