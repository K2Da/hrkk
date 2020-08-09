use crate::help::Help;
use crate::show::Section;
use crate::ui::scene::SceneBase;
use crate::ui::{scene, UiScene};
use rustbox::keyboard::Key;

pub(crate) fn section_popup_open<F>(
    key: Key,
    scene_base: &SceneBase,
    view_section: F,
    scene: Box<UiScene>,
) -> Option<crate::ui::scene::section_popup::Scene>
where
    F: Fn() -> Section,
{
    return match key {
        Key::Char('V') | Key::Ctrl('v') => {
            let log_scene = scene::section_popup::new(
                scene_base.duplicate(None, Some(scene)),
                "View",
                view_section().clone(),
            );
            Some(log_scene)
        }

        _ => None,
    };
}

pub(crate) fn helps() -> Vec<Help> {
    vec![Help::new("V", None, "popup viewer window")]
}
