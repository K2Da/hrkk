use crate::help::Help;
use rustbox::keyboard::Key;

pub(crate) fn text_input(key: Key, text: &mut String) -> Option<()> {
    match key {
        Key::Char(key) => {
            if key.is_uppercase() {
                None
            } else {
                text.push(key);
                Some(())
            }
        }

        Key::Esc => {
            if text.len() > 0 {
                text.clear();
                Some(())
            } else {
                None
            }
        }

        Key::Backspace => {
            text.pop();
            Some(())
        }

        _ => None,
    }
}

pub(crate) fn helps() -> Vec<Help> {
    vec![
        Help::new(
            "a-z",
            Some("filter"),
            "small letters, numeric and symbols to filter list items",
        ),
        Help::new("BS", None, "delete filtering texts"),
        Help::new(
            "ESC",
            Some("clear/back/quit"),
            "clear filter, back to menu or quit this command",
        ),
    ]
}
