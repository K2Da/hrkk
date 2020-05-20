use crate::error::Result;
use crate::show;
use ansi_term::Color;
use ansi_term::Style;
use skim::prelude::*;

const RESOURCE_BG: Color = Color::Fixed(255);
const RESOURCE_FG: Color = Color::Fixed(0);

pub fn skim(list: &(String, Vec<(String, String)>)) -> Result<Vec<Arc<dyn SkimItem>>> {
    let (sender, receiver): (SkimItemSender, SkimItemReceiver) = unbounded();

    print_list_items(list, &sender);

    super::skim_run(false, receiver)
}

fn print_list_items(list: &(String, Vec<(String, String)>), sender: &Sender<Arc<dyn SkimItem>>) {
    let (title, list) = list;
    for (line, desc) in list {
        sender
            .send(Arc::new(SkItem {
                title: title.to_owned(),
                value: line.to_owned(),
                desc: desc.to_owned(),
            }))
            .unwrap();
    }
}

pub struct SkItem {
    title: String,
    value: String,
    desc: String,
}

impl SkimItem for SkItem {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(AnsiString::parse(&self.value.clone()))
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.value)
    }

    fn preview(&self) -> ItemPreview {
        let header = Style::new()
            .on(RESOURCE_BG)
            .fg(RESOURCE_FG)
            .paint(format!(
                " {:<max$}\n",
                "Select parameter",
                max = crate::skimmer::preview_width() as usize
            ))
            .to_string();

        let ansi_text = show::Section::new_without_yaml()
            .string_name(&format!("{}", &self.title,))
            .str("value", &self.value)
            .str("description", &self.desc)
            .print_all();

        ItemPreview::AnsiText(format!("\n{}{}", header, ansi_text))
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.value)
    }
}
