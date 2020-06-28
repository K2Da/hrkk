use crate::help::{Help, Helps};
use crate::log::Logs;
use crate::ui::scene::SceneBase;
use crate::ui::{scene, UiScene};
use rustbox::keyboard::Key;

pub(crate) fn text_popup_open(
    key: Key,
    scene_base: &SceneBase,
    logs: &Logs,
    helps: &Helps,
    scene: Box<UiScene>,
) -> Option<crate::ui::scene::text_popup::Scene> {
    return match key {
        Key::Char('L') | Key::Ctrl('l') => {
            let log_scene = scene::text_popup::new(
                scene_base.duplicate(None, Some(scene)),
                "Log",
                logs.to_text(0),
            );
            Some(log_scene)
        }

        Key::Char('H') | Key::Ctrl('h') => {
            let help_scene = scene::text_popup::new(
                scene_base.duplicate(None, Some(scene)),
                "Help",
                helps.to_popup_text(),
            );
            Some(help_scene)
        }

        _ => None,
    };
}

pub(crate) fn helps() -> Vec<Help> {
    vec![
        Help::new("L", None, "popup log window"),
        Help::new("H", None, "popup help window"),
    ]
}
