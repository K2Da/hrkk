use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn layout(area: Rect) -> (Rect, Rect, Rect) {
    let lr = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(0)
        .split(area);

    let tb = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .margin(0)
        .split(lr[0]);

    (tb[0], tb[1], lr[1])
}
