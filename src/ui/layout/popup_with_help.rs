use tui::layout::{Constraint, Constraint::*, Direction, Layout, Rect};

pub(crate) fn layout(percent_x: u16, percent_y: u16, r: Rect) -> (Rect, Rect, Rect) {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(center_constraint(percent_y).as_ref())
        .split(r);

    let center_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(center_constraint(percent_x).as_ref())
        .split(popup_layout[1])[1];

    let text_help = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Min(10), Length(2)].as_ref())
        .margin(1)
        .split(center_box);

    (center_box, text_help[0], text_help[1])
}

fn center_constraint(percent: u16) -> [Constraint; 3] {
    [
        Percentage((100 - percent) / 2),
        Percentage(percent),
        Percentage((100 - percent) / 2),
    ]
}
