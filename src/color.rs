use ansi_term::{Color, Style};

#[derive(Debug, Default, Clone)]
pub struct ColoredString {
    string: String,
    colored: String,
}

impl ColoredString {
    pub fn no_style(string: &str) -> Self {
        Self {
            string: string.to_string(),
            colored: string.to_string(),
        }
    }

    pub fn empty() -> Self {
        Self {
            string: "".to_string(),
            colored: "".to_string(),
        }
    }

    pub fn new(string: &str, fg: Option<Color>, bg: Option<Color>) -> Self {
        let mut style = Style::new();

        if let Some(fg) = fg {
            style = style.fg(fg);
        }

        if let Some(bg) = bg {
            style = style.on(bg);
        }

        Self {
            string: string.to_string(),
            colored: style.paint(string).to_string(),
        }
    }

    pub fn push_str(&mut self, other: Self) {
        self.string.push_str(&other.string);
        self.colored.push_str(&other.colored);
    }

    pub fn len(&self) -> isize {
        self.string.len() as isize
    }
}

use std::fmt;
impl std::fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.colored)
    }
}
