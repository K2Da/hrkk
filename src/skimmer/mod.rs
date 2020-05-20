use crate::error::Result;
use ansi_term::Color;
use std::cmp::max;
pub mod commands;
pub mod list;
pub mod resources;
use skim::prelude::*;

fn max_lengths(list: &Vec<Vec<String>>) -> Vec<usize> {
    let mut max_list: Vec<usize> = vec![];
    for cols in list {
        for (i, col) in cols.iter().enumerate() {
            if let Some(current) = max_list.get_mut(i) {
                *current = max(*current, col.len());
            } else {
                max_list.push(col.len());
            }
        }
    }
    max_list
}

fn align_table(max_list: &Vec<usize>, cols: &Vec<&str>) -> (String, String) {
    let mut line: String = "".to_string();
    let mut color: String = "".to_string();

    for (i, col) in cols.iter().enumerate() {
        let max = *max_list.get(i).unwrap_or(&0);
        if i == 0 {
            line = format!("{:<max$}", col, max = max);
            color = format!("{:<max$}", col, max = max);
        } else {
            line = format!("{} {} {:<max$}", line, "│", col, max = max);
            color = format!(
                "{} {} {:<max$}",
                color,
                &Color::Fixed(64).paint("│").to_string(),
                col,
                max = max
            );
        }
    }

    (line, color)
}

pub fn preview_width() -> isize {
    if let Some((w, _)) = term_size::dimensions() {
        max(((w / 2) + 4) as isize, 40)
    } else {
        80
    }
}

pub fn skim_run(multi: bool, receiver: SkimItemReceiver) -> Result<Vec<Arc<dyn SkimItem>>> {
    Ok(Skim::run_with(
        &skim_option(multi, &format!("right:{}", preview_width() + 1)),
        Some(receiver),
    )
    .map(|out| out.selected_items)
    .unwrap_or_else(|| Vec::new()))
}

fn skim_option(multi: bool, preview: &str) -> SkimOptions {
    SkimOptionsBuilder::default()
        .height(Some("90%"))
        .multi(multi)
        .no_mouse(true)
        .preview(Some(""))
        .preview_window(Some(&preview))
        .ansi(true)
        .build()
        .unwrap()
}
