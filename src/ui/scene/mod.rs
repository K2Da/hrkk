pub(crate) mod commands;
use crate::ui::key_handler;
pub(crate) mod list_option;
pub(crate) mod resources;
pub(crate) mod section_popup;
pub(crate) mod text_popup;
use crate::help::Help;
use crate::opts::Opts;
use crate::ui::util::event::Event;
use crate::ui::{scene, NextScene, UiScene};
use tokio::sync::mpsc;

pub(crate) type EventSender = mpsc::Sender<Event>;

#[derive(Clone)]
pub(crate) struct SceneBase {
    pub should_draw: bool,
    opts: Opts,
    tx: EventSender,
    overlay: Option<Box<UiScene>>,
    history: Option<Box<UiScene>>,
}

impl SceneBase {
    pub(crate) fn minimum(opts: Opts, tx: EventSender) -> Self {
        Self {
            should_draw: true,
            opts,
            tx,
            overlay: None,
            history: None,
        }
    }

    pub(crate) fn with_history(opts: Opts, tx: EventSender, history: Option<Box<UiScene>>) -> Self {
        Self {
            should_draw: true,
            opts,
            tx,
            overlay: None,
            history,
        }
    }

    pub(crate) fn duplicate(
        &self,
        overlay: Option<Box<UiScene>>,
        history: Option<Box<UiScene>>,
    ) -> Self {
        Self {
            should_draw: true,
            opts: self.opts.clone(),
            tx: self.tx.clone(),
            overlay,
            history,
        }
    }

    pub(crate) fn back_or_root_menu(&mut self) -> NextScene {
        NextScene::Scene(match &self.history {
            Some(last) => {
                let back_to = *last.clone();
                back_to
            }
            None => UiScene::Commands(scene::commands::new(self.duplicate(None, None))),
        })
    }
}

fn common_helps(all_helps: &mut Vec<Help>) {
    all_helps.append(&mut key_handler::text_input::helps());
    all_helps.append(&mut key_handler::table_move::helps());
    all_helps.append(&mut key_handler::text_popup_open::helps());
    all_helps.append(&mut key_handler::section_popup_open::helps());
    all_helps.append(&mut key_handler::common::helps());
}
