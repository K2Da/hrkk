use tui::layout::{Constraint::*, Direction, Layout, Rect};

pub(crate) fn layout(area: Rect) -> (Rect, (Rect, Rect, (Rect, Rect)), (Rect, Rect, (Rect, Rect))) {
    let title_body = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(1), Min(10)].as_ref())
        .split(area);

    let left_right = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Percentage(50), Percentage(50)].as_ref())
        .split(title_body[1]);

    let search_table_log = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(3), Min(10), Length(2)].as_ref())
        .split(left_right[0]);

    let log = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Length(4), Min(10)].as_ref())
        .split(search_table_log[2]);

    let info_view_help = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Length(3), Min(10), Length(2)].as_ref())
        .split(left_right[1]);

    let help = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Length(4), Min(10)].as_ref())
        .split(info_view_help[2]);

    (
        title_body[0],
        (search_table_log[0], search_table_log[1], (log[0], log[1])),
        (info_view_help[0], info_view_help[1], (help[0], help[1])),
    )
}
