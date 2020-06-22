use tui::widgets::ListState;

pub fn next(state: &mut ListState, item_len: usize) {
    state.select(Some(match state.selected() {
        Some(i) => (i + 1) % item_len,
        None => 0,
    }));
}

pub fn previous(state: &mut ListState, item_len: usize) {
    state.select(Some(match state.selected() {
        Some(i) => (i + item_len - 1) % item_len,
        None => 0,
    }));
}

pub fn select_any(state: &mut ListState, item_len: usize) -> bool {
    if item_len == 0 {
        return false;
    }
    if state.selected().is_none() {
        state.select(Some(0));
        return true;
    }
    false
}
