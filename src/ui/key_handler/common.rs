use crate::help::Help;
use crate::ui::scene::SceneBase;
use crate::ui::NextScene;
use rustbox::keyboard::Key;

pub(crate) fn common(key: Key, base: &mut SceneBase, exit_to_menu: bool) -> Option<NextScene> {
    match key {
        Key::Esc => {
            if exit_to_menu {
                Some(base.back_or_root_menu())
            } else {
                Some(NextScene::Exit(None))
            }
        }
        Key::Ctrl('c') | Key::Char('C') => Some(NextScene::Exit(None)),
        _ => None,
    }
}

pub(crate) fn helps() -> Vec<Help> {
    vec![Help::new("C", Some("quit"), "quit this command")]
}
