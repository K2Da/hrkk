use crate::opts::Opts;
use crate::service::AwsResource;
use crate::show;
use crate::ui::scene::resources::ListApiCall;
use crate::ui::widget::util::table;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    terminal::Frame,
    widgets::TableState,
    widgets::{Row, Table},
};
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub(crate) index: usize,
    row: Vec<String>,
    match_string: String,
    pub(crate) list_yaml: Yaml,
    pub(crate) get_yaml: Option<Yaml>,
}

impl table::Matchable for Item {
    fn match_string(&self) -> String {
        self.match_string.clone()
    }
}

impl Item {
    pub(crate) fn get_parameter(&self) -> String {
        self.row.first().unwrap_or(&"".to_owned()).to_string()
    }
}

#[derive(Clone)]
pub(crate) struct Resources {
    pub(crate) state: TableState,
    pub(crate) items: Vec<Item>,
    filtered_indexes: Vec<usize>,
    pub(crate) last_height: u16,
    pub(crate) selected_indexes: Vec<usize>,
    column_max_lengths: Vec<usize>,
    resource: Box<dyn AwsResource>,
}

pub(crate) fn new(resource: Box<dyn AwsResource>) -> Resources {
    let items = vec![];

    let mut s = Resources {
        state: TableState::default(),
        items,
        resource,
        filtered_indexes: vec![],
        last_height: 0,
        selected_indexes: vec![],
        column_max_lengths: vec![],
    };

    s.filter("");
    s.calc_column_max_lengths();
    s
}

impl Resources {
    pub(crate) fn clear(&mut self) {
        self.items = vec![];
        self.filtered_indexes = vec![];
        self.selected_indexes = vec![];
    }

    pub(crate) fn filter(&mut self, search_text: &str) {
        self.filtered_indexes = table::filter(search_text, &mut self.items, &mut self.state);
    }

    pub(crate) fn add_resource_list(
        &mut self,
        yaml: crate::service::ResourceList,
        search_text: &str,
    ) {
        for (line, yaml) in yaml {
            self.items.push(Item {
                index: self.items.len(),
                match_string: line.join(" "),
                row: line,
                list_yaml: yaml,
                get_yaml: None,
            });
        }
        self.calc_column_max_lengths();
        self.filter(search_text);
    }

    pub(crate) fn add_get_yaml(&mut self, yaml: Yaml, resource_index: usize) {
        let mut item = &mut self.items[resource_index];
        item.get_yaml = Some(yaml);
        item.row = self.resource.line(&item.list_yaml, &item.get_yaml);
        item.match_string = item.row.join(" ");
        self.calc_column_max_lengths();
    }

    pub(crate) fn calc_column_max_lengths(&mut self) {
        let mut row_lengths: Vec<Vec<usize>> = self
            .items
            .iter()
            .map(|item| item.row.iter().map(|s| s.len()).collect())
            .collect();
        row_lengths.push(self.resource.header().iter().map(|s| s.len()).collect());
        self.column_max_lengths = super::column_max_list(&row_lengths);
    }

    pub(crate) fn selected_key(&self) -> Option<String> {
        if let Some(item) = self.selected_item() {
            return Some(self.resource.resource_name(&item.list_yaml));
        }
        None
    }

    pub(crate) fn selected_item(&self) -> Option<Item> {
        if let Some(index) = self.selected_index() {
            return Some(self.items[index].clone());
        }
        None
    }

    pub(crate) fn selected_index(&self) -> Option<usize> {
        if let Some(index) = self.state.selected() {
            return Some(self.filtered_indexes[index]);
        }
        None
    }

    pub(crate) fn selected_names_or_url(&mut self, opts: &Opts) -> Option<String> {
        let selected_items = self
            .selected_items()
            .iter()
            .map(|item| match opts.output_type() {
                crate::opts::OutputType::ConsoleURL => {
                    self.resource
                        .console_url(&item.list_yaml, &item.get_yaml, &opts.region_name())
                }
                crate::opts::OutputType::ResourceIdentifier => {
                    self.resource.resource_name(&item.list_yaml)
                }
            })
            .collect::<Vec<String>>();

        if selected_items.len() > 0 {
            Some(selected_items.join(&opts.delimiter()))
        } else {
            None
        }
    }

    pub(crate) fn selected_yamls(&mut self) -> Vec<Yaml> {
        self.selected_items()
            .iter()
            .map(|item| item.list_yaml.clone())
            .collect()
    }

    fn selected_items(&mut self) -> Vec<Item> {
        if let Some(index) = self.state.selected() {
            self.selected_indexes.push(self.filtered_indexes[index]);
        }

        if self.selected_indexes.len() == 0 {
            return vec![];
        }

        self.items
            .iter()
            .enumerate()
            .filter(|(index, _)| self.selected_indexes.contains(index))
            .map(|(_, item)| item.clone())
            .collect()
    }

    fn column_width(&self) -> Vec<Constraint> {
        self.column_max_lengths
            .iter()
            .enumerate()
            .map(|(i, max_length)| {
                Constraint::Length((*max_length as u16) + if i == 0 { 2 } else { 0 })
            })
            .collect()
    }

    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect, api_call: &ListApiCall)
    where
        B: Backend,
    {
        let filtered_items = table::filtered_items(&self.items, &self.filtered_indexes);
        self.last_height = area.height;
        let header = self.resource.header();
        let column_width = self.column_width();
        let title = self.title(api_call);
        let selected_indexes = self.selected_indexes.clone();

        let rows = filtered_items
            .iter()
            .map(|(index, item)| {
                if selected_indexes.contains(index) {
                    (*index, Self::row(item.row.clone(), true))
                } else {
                    (*index, Self::row(item.row.clone(), false))
                }
            })
            .collect::<Vec<(usize, Vec<String>)>>();

        f.render_stateful_widget(
            table::style(
                Table::new(
                    header.iter(),
                    rows.iter().map(|(index, row)| {
                        if selected_indexes.contains(index) {
                            Row::StyledData(row.iter(), Style::default().fg(Color::Yellow))
                        } else {
                            Row::Data(row.iter())
                        }
                    }),
                ),
                &title,
                &column_width,
            ),
            area,
            &mut self.state,
        );
    }

    fn row(mut row: Vec<String>, selected: bool) -> Vec<String> {
        row[0] = format!("{}{}", if selected { "*" } else { " " }, row[0]);
        row
    }

    fn title(&self, api_call: &ListApiCall) -> String {
        let api_status = match api_call {
            ListApiCall::None => "-",
            ListApiCall::StillHave { .. } => "remaining",
            ListApiCall::Requesting { .. } => "requesting",
            ListApiCall::Completed => "fetched all",
        };

        let selected = if self.selected_indexes.len() > 0 {
            format!("selected {} / ", self.selected_indexes.len())
        } else {
            "".to_string()
        };

        format!(
            " {} ({}filtered {} from {}) - {} ",
            match self.state.selected() {
                Some(i) => (i + 1).to_string(),
                None => "-".to_string(),
            },
            selected,
            self.filtered_indexes.len(),
            self.items.len(),
            api_status,
        )
    }

    pub(crate) fn selected_item_detail(&self, region: &str) -> show::Section {
        match self.selected_item() {
            Some(item) => self
                .resource
                .detail(&item.list_yaml, &item.get_yaml, region),
            None => show::Section::new_without_yaml(),
        }
    }

    pub(crate) fn filtered_len(&self) -> usize {
        self.filtered_indexes.len()
    }

    pub(crate) fn toggle_selected(&mut self) {
        if let Some(index) = self.state.selected() {
            let index = self.filtered_indexes[index];
            if self.selected_indexes.contains(&index) {
                self.selected_indexes.retain(|i| *i != index);
            } else {
                self.selected_indexes.push(index);
            }
        }
    }
}
