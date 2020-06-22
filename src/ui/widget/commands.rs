use crate::service::file::{path_str, store_resource_file_path};
use crate::service::AwsResource;
use crate::show;
use crate::ui::widget::util::table;
use tui::widgets::TableState;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    terminal::Frame,
    widgets::{Row, Table},
};

#[derive(Clone)]
pub struct Item {
    row: Vec<String>,
    match_string: String,
    command: Box<dyn AwsResource>,
}

impl table::Matchable for Item {
    fn match_string(&self) -> String {
        self.match_string.clone()
    }
}

pub struct Selector {
    pub state: TableState,
    items: Vec<Item>,
    pub header: Vec<String>,
    filtered_indexes: Vec<usize>,
}

pub fn new() -> Selector {
    let mut items = vec![];
    let resources = crate::service::all_resources();
    for resource in resources {
        items.push(Item {
            row: vec![resource.service_name(), resource.command_name()],
            match_string: format!("{} {}", resource.service_name(), resource.command_name()),
            command: resource,
        });
    }

    let mut s = Selector {
        state: TableState::default(),
        header: vec!["".to_string(), "".to_string()],
        items,
        filtered_indexes: vec![],
    };

    s.filter("");
    s
}

impl Selector {
    pub fn filter(&mut self, search_text: &str) {
        self.filtered_indexes = table::filter(search_text, &mut self.items, &mut self.state);
    }

    pub fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let filtered_items = table::filtered_items(&self.items, &self.filtered_indexes);

        let widget = table::style(
            Table::new(
                self.header.iter(),
                filtered_items.iter().map(|(_, item)| {
                    Row::StyledData(item.row.iter(), Style::default().fg(Color::White))
                }),
            ),
            " command list ",
            &[Constraint::Length(20), Constraint::Min(10)],
        );

        f.render_stateful_widget(widget, area, &mut self.state);
    }

    pub fn selected_resource(&self) -> Option<Box<dyn AwsResource>> {
        if let Some(index) = self.state.selected() {
            return Some(crate::service::resource_by_name(
                &self.items[self.filtered_indexes[index]].command.name(),
            ));
        }
        None
    }

    pub fn command_detail(&self) -> show::Section {
        match self.state.selected() {
            Some(index) => {
                let item = &self.items[self.filtered_indexes[index]];
                show::Section::new_without_yaml()
                    .string_name(&format!(
                        "{} {}",
                        item.command.service_name(),
                        item.command.command_name()
                    ))
                    .str("document", &item.command.info().document_url)
                    .str(
                        "cache path",
                        &path_str(&store_resource_file_path(&*item.command)),
                    )
                    .str(
                        "result limit",
                        &format!("{}", &item.command.info().max_limit),
                    )
            }
            None => show::Section::new_without_yaml(),
        }
    }

    pub fn filtered_len(&self) -> usize {
        self.filtered_indexes.len()
    }
}
