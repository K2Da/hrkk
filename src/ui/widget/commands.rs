use crate::service::AwsResource;
use crate::show;
use crate::ui::widget::util::table;
use tui::widgets::TableState;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::Style,
    terminal::Frame,
    widgets::{Row, Table},
};

#[derive(Clone)]
pub(crate) struct Item {
    row: Vec<String>,
    match_string: String,
    command: Box<dyn AwsResource>,
}

impl table::Matchable for Item {
    fn match_string(&self) -> String {
        self.match_string.clone()
    }
}

#[derive(Clone)]
pub(crate) struct Commands {
    pub(crate) state: TableState,
    items: Vec<Item>,
    pub(crate) header: Vec<String>,
    filtered_indexes: Vec<usize>,
    pub(crate) last_height: u16,
}

pub(crate) fn new() -> Commands {
    let mut items = vec![];
    let resources = crate::service::all_resources();
    for resource in resources {
        items.push(Item {
            row: vec![resource.service_name(), resource.command_name()],
            match_string: format!("{} {}", resource.service_name(), resource.command_name()),
            command: resource,
        });
    }

    let mut s = Commands {
        state: TableState::default(),
        header: vec!["".to_string(), "".to_string()],
        items,
        filtered_indexes: vec![],
        last_height: 0,
    };

    s.filter("");
    s
}

impl Commands {
    pub(crate) fn filter(&mut self, search_text: &str) {
        self.filtered_indexes = table::filter(search_text, &mut self.items, &mut self.state);
    }

    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let filtered_items = table::filtered_items(&self.items, &self.filtered_indexes);
        self.last_height = area.height;

        let widget = table::style(
            Table::new(
                self.header.iter(),
                filtered_items
                    .iter()
                    .map(|(_, item)| Row::StyledData(item.row.iter(), Style::default())),
            ),
            " command list ",
            &[Constraint::Length(20), Constraint::Min(10)],
        );

        f.render_stateful_widget(widget, area, &mut self.state);
    }

    pub(crate) fn selected_resource(&self) -> Option<Box<dyn AwsResource>> {
        if let Some(index) = self.state.selected() {
            return Some(crate::service::resource_by_name(
                &self.items[self.filtered_indexes[index]].command.name(),
            ));
        }
        None
    }

    pub(crate) fn command_detail(&self) -> show::Section {
        match self.state.selected() {
            Some(index) => {
                let item = &self.items[self.filtered_indexes[index]];
                let info = item.command.info();
                let mut section = show::Section::new_without_yaml()
                    .string_name(&format!(
                        "{} {}",
                        item.command.service_name(),
                        item.command.command_name()
                    ))
                    .str("result limit", &format!("{}", &item.command.max_limit()));

                section = section.str("List API", &info.list_api.format.name());

                section = section.str("docs", &info.list_api.document);

                if let Some(get) = &info.get_api {
                    section = section.str("Get API", &get.format.name());
                    section = section.str("docs", get.document);
                }

                if let Some(url) = &info.resource_url {
                    section = section.str("resource uri", &format!("{:?}", url));
                }

                if let Some(parameter_name) = &info.list_api.format.parameter_name() {
                    section = section.str("required parameter", parameter_name);
                }

                section
            }
            None => show::Section::new_without_yaml(),
        }
    }

    pub(crate) fn filtered_len(&self) -> usize {
        self.filtered_indexes.len()
    }
}
