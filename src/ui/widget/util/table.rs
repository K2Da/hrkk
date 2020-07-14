use crate::color;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use tui::{
    layout::Constraint,
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
};

pub(crate) fn walk(mut step: isize, item_len: usize, state: &mut TableState) {
    if item_len == 0 {
        return;
    }

    if step < 0 {
        step += item_len as isize;
    }

    state.select(Some(match state.selected() {
        Some(i) => (i + step as usize) % item_len,
        None => 0,
    }));
}

pub(crate) fn walk_to_wall(step: isize, item_len: usize, state: &mut TableState) {
    if item_len == 0 {
        return;
    }

    state.select(Some(match state.selected() {
        Some(i) => {
            let loc: isize = i as isize + step;
            std::cmp::min(item_len - 1, std::cmp::max(0, loc) as usize)
        }
        None => 0,
    }));
}

pub(crate) fn select_any(item_len: usize, state: &mut TableState) -> bool {
    if item_len == 0 {
        return false;
    }
    if state.selected().is_none() {
        state.select(Some(0));
        return true;
    }
    false
}

pub(crate) trait Matchable: Clone {
    fn match_string(&self) -> String;
}

pub(crate) fn filter<T>(search_text: &str, items: &Vec<T>, state: &mut TableState) -> Vec<usize>
where
    T: Matchable,
{
    let matcher = SkimMatcherV2::default().ignore_case();
    let mut matched_items = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            (
                matcher
                    .fuzzy_match(&item.match_string(), search_text)
                    .unwrap_or(0),
                item,
                i,
            )
        })
        .filter(|(score, _, _)| search_text.len() == 0 || *score > 0)
        .collect::<Vec<(i64, &T, usize)>>();

    matched_items
        .sort_by(|(a, _, _), (b, _, _)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    state.select(None);

    matched_items.iter().map(|(_, _, index)| *index).collect()
}

pub(crate) fn filtered_items<T>(items: &Vec<T>, indexes: &Vec<usize>) -> Vec<(usize, T)>
where
    T: Matchable,
{
    indexes
        .iter()
        .map(|index| (*index, items[*index].clone()))
        .collect()
}

pub(crate) fn style<'a, H, R, D>(
    table: Table<'a, H, R>,
    title: &'a str,
    widths: &'a [Constraint],
) -> Table<'a, H, R>
where
    H: Iterator,
    D: Iterator,
    D::Item: std::fmt::Display,
    R: Iterator<Item = Row<D>>,
{
    table
        .block(Block::default().borders(Borders::ALL).title(title))
        .header_gap(0)
        .highlight_style(
            Style::default()
                .fg(color::HIGHLIGHT)
                .modifier(Modifier::BOLD),
        )
        .highlight_symbol(">️")
        .widths(widths)
}
