use crate::show;
use tui::{
    backend::Backend,
    layout::Rect,
    terminal::Frame,
    widgets::{Paragraph, Text},
};
pub struct Box {
    pub scroll: u16,
    pub text: show::Section,
}

pub fn new(text: show::Section) -> Box {
    Box { text, scroll: 0 }
}

impl Box {
    pub fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let mut text = vec![];

        let texts = self.text.print_all(area.width as isize).0;
        for t in &texts {
            match t {
                show::Txt::Raw(str) => text.push(Text::raw(str)),
                show::Txt::Styled(str, style) => text.push(Text::styled(str, style.clone())),
            }
        }

        let widget = Paragraph::new(text.iter()).wrap(false).raw(false).scroll(0);
        f.render_widget(widget, area);
    }
}
