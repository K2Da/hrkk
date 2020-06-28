use crate::color;
use crate::show::{Texts, Txt};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub(crate) struct Help {
    key: String,
    short: Option<String>,
    action: String,
}

impl Help {
    pub fn new(key: &str, short: Option<&str>, action: &str) -> Self {
        Help {
            key: key.to_owned(),
            short: short.map(|s| s.to_owned()),
            action: action.to_owned(),
        }
    }

    fn to_short_text(&self) -> Vec<Txt> {
        let mut text = crate::ui::widget::txt_button(&self.key).to_vec();
        text.push(Txt::raw(":"));
        text.push(Txt::raw(&self.short.as_ref().unwrap()));
        text.push(Txt::raw(" "));
        text
    }

    fn to_long_text(&self, key_max: usize) -> Vec<Txt> {
        let mut text = vec![];
        text.push(Txt::raw(&"│"));
        text.push(Txt::raw(&" ".repeat(key_max - self.key.width_cjk())));
        text.push(Txt::colored(&self.key, color::BUTTON));
        text.push(Txt::raw(&" │ "));
        text.push(Txt::raw(&self.action));
        text.push(Txt::raw("\n"));
        text
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Helps(Vec<Help>);

impl Helps {
    pub(crate) fn new(helps: Vec<Help>) -> Self {
        Self(helps)
    }

    pub(crate) fn to_summary_text(&self) -> Texts {
        Texts(
            self.0
                .iter()
                .filter(|i| i.short.is_some())
                .map(|i| i.to_short_text())
                .into_iter()
                .flatten()
                .collect(),
        )
    }

    pub(crate) fn to_popup_text(&self) -> Texts {
        let key_max = self.0.iter().map(|i| i.key.width_cjk()).max().unwrap_or(0) + 1;

        Texts(
            self.0
                .iter()
                .map(|i| i.to_long_text(key_max))
                .into_iter()
                .flatten()
                .collect(),
        )
    }
}
