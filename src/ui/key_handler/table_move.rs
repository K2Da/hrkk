use crate::help::Help;
use crate::ui::widget::util::table;
use rustbox::keyboard::Key;
use tui::widgets::TableState;

pub fn table_move(
    key: Key,
    item_len: usize,
    state: &mut TableState,
    height: u16,
    viewer_scroll: &mut u16,
    viewer_line_len: u16,
) -> Option<bool> {
    match key {
        Key::Char('K') | Key::Ctrl('k') => {
            *viewer_scroll = std::cmp::max(*viewer_scroll as i16 - 1, 0) as u16;
            Some(false)
        }

        Key::Char('J') | Key::Ctrl('j') => {
            *viewer_scroll = std::cmp::min(*viewer_scroll + 1, viewer_line_len);
            Some(false)
        }

        Key::Ctrl('b') | Key::Char('B') => {
            table::walk_to_wall(-((height / 2) as isize), item_len, state);
            Some(true)
        }

        Key::Ctrl('f') | Key::Char('F') => {
            table::walk_to_wall((height / 2) as isize, item_len, state);
            Some(true)
        }

        Key::Up => {
            table::walk(-1, item_len, state);
            Some(true)
        }

        Key::Down => {
            table::walk(1, item_len, state);
            Some(true)
        }

        Key::PageUp | Key::Ctrl('u') | Key::Char('U') => {
            *viewer_scroll = std::cmp::max(*viewer_scroll as i16 - (height / 2) as i16, 0) as u16;
            Some(false)
        }

        Key::PageDown | Key::Ctrl('d') | Key::Char('D') => {
            *viewer_scroll = std::cmp::min(*viewer_scroll + (height / 2) as u16, viewer_line_len);
            Some(false)
        }

        _ => return None,
    }
}

pub(crate) fn helps() -> Vec<Help> {
    vec![
        Help::new("⬆⬇", Some("move list"), "move list(left side)"),
        Help::new(
            "B/F",
            Some("move list faster"),
            "move list(left side) 1/2 screen",
        ),
        Help::new("K/J", Some("scroll viewer"), "scroll viewer(right side)"),
        Help::new(
            "U/D",
            Some("scroll viewer faster"),
            "scroll viewer(right side) 1/2 screen",
        ),
    ]
}
