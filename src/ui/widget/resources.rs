use crate::service::AwsResource;
use crate::show;
use crate::ui::scene::resources::ApiCall;
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
    row: Vec<String>,
    match_string: String,
    yaml: Yaml,
}

impl table::Matchable for Item {
    fn match_string(&self) -> String {
        self.match_string.clone()
    }
}

#[derive(Clone)]
pub(crate) struct Resources {
    pub(crate) state: TableState,
    items: Vec<Item>,
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

    pub(crate) fn add_yaml(&mut self, yaml: Vec<Yaml>, search_text: &str) {
        for y in yaml {
            self.items.push(Item {
                row: self.resource.line(&y),
                match_string: self.resource.line(&y).join(" "),
                yaml: y.clone(),
            });
        }
        self.calc_column_max_lengths();
        self.filter(search_text);
    }

    fn calc_column_max_lengths(&mut self) {
        let mut row_lengths: Vec<Vec<usize>> = self
            .items
            .iter()
            .map(|item| item.row.iter().map(|s| s.len()).collect())
            .collect();
        row_lengths.push(self.resource.header().iter().map(|s| s.len()).collect());
        self.column_max_lengths = super::column_max_list(&row_lengths);
    }

    pub(crate) fn selected_key(&self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            return Some(
                self.resource
                    .resource_name(&self.items[self.filtered_indexes[index]].yaml),
            );
        }
        None
    }

    pub(crate) fn selected_names(&mut self, delimiter: &str) -> Option<String> {
        let selected_items = self
            .selected_items()
            .iter()
            .map(|item| self.resource.resource_name(&item.yaml))
            .collect::<Vec<String>>();

        if selected_items.len() > 0 {
            Some(selected_items.join(delimiter))
        } else {
            None
        }
    }

    pub(crate) fn selected_yamls(&mut self) -> Vec<Yaml> {
        self.selected_items()
            .iter()
            .map(|item| item.yaml.clone())
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

    pub(crate) fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect, api_call: &ApiCall)
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

    fn title(&self, api_call: &ApiCall) -> String {
        let api_status = match api_call {
            ApiCall::None => "-",
            ApiCall::StillHave { .. } => "remaining",
            ApiCall::Requesting { .. } => "requesting",
            ApiCall::Completed => "fetched all",
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

    pub(crate) fn item_detail(&self) -> show::Section {
        match self.state.selected() {
            Some(index) => {
                let item = &self.items[self.filtered_indexes[index]];
                self.resource.detail(&item.yaml)
            }
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
