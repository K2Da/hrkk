use crate::help::Help;
use crate::ui::scene::SceneBase;
use crate::ui::NextScene;
use rustbox::keyboard::Key;

pub(in crate::ui) fn popup(
    key: Option<Key>,
    base: &mut SceneBase,
    offset: &mut u16,
    line_len: u16,
) -> NextScene {
    if let Some(key) = key {
        match key {
            Key::Esc | Key::Ctrl('v') | Key::Char('V') => return base.back_or_root_menu(),

            Key::Ctrl('c') | Key::Char('C') => return NextScene::Exit(None),

            Key::Down => *offset = std::cmp::min(line_len, *offset + 1),

            Key::Up => *offset = std::cmp::max(0, *offset as i32 - 1) as u16,

            _ => (),
        }
    }
    NextScene::Same
}

pub(crate) fn helps() -> Vec<Help> {
    vec![
        Help::new("⬆⬇️️", Some("scroll"), ""),
        Help::new("ESC/V️", Some("close"), ""),
        Help::new("C", Some("quit"), ""),
    ]
}
