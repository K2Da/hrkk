pub(in crate::ui) mod help;
pub(in crate::ui) use help::Help;
pub(in crate::ui) mod commands;
pub(in crate::ui) use commands::Commands;
pub(in crate::ui) mod text_list;
pub(in crate::ui) use text_list::TextList;
pub(in crate::ui) mod log;
pub(in crate::ui) use log::Log;
pub(in crate::ui) mod resources;
pub(in crate::ui) use resources::Resources;
pub(in crate::ui) mod search;
pub(in crate::ui) use search::Search;
pub(in crate::ui) mod status;
pub(in crate::ui) use status::Status;
pub(in crate::ui) mod util;
pub(in crate::ui) mod viewer;
pub(in crate::ui) use viewer::Viewer;
pub(in crate::ui) mod info;
pub(in crate::ui) use info::Info;
pub(in crate::ui) mod popup_text;
pub(in crate::ui) use popup_text::PopupText;

use crate::color;
use crate::show::Txt;
use tui::style::Style;
use tui::widgets::Text;

fn column_max_list(list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut max_list: Vec<usize> = vec![];
    for cols in list {
        for (i, col) in cols.iter().enumerate() {
            if let Some(current) = max_list.get_mut(i) {
                *current = std::cmp::max(*current, *col);
            } else {
                max_list.push(*col);
            }
        }
    }
    max_list
}

pub(crate) fn text_button(button: &str) -> [Text; 3] {
    [
        Text::raw("["),
        Text::styled(button, Style::default().fg(color::BUTTON)),
        Text::raw("]"),
    ]
}

pub(crate) fn txt_button(button: &str) -> [Txt; 3] {
    [
        Txt::raw("["),
        Txt::colored(button, color::BUTTON),
        Txt::raw("]"),
    ]
}
