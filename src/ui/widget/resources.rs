use crate::service::AwsResource;
use crate::show;
use crate::ui::scene::resources::ApiCall;
use crate::ui::widget::util::table;
use tui::widgets::TableState;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    terminal::Frame,
    widgets::{Row, Table},
};
use yaml_rust::Yaml;

#[derive(Debug, Clone)]
pub struct Item {
    row: Vec<String>,
    match_string: String,
    yaml: Yaml,
}

impl table::Matchable for Item {
    fn match_string(&self) -> String {
        self.match_string.clone()
    }
}

pub struct Selector {
    pub state: TableState,
    items: Vec<Item>,
    filtered_indexes: Vec<usize>,
    pub selected_indexes: Vec<usize>,
    resource: Box<dyn AwsResource>,
}

pub fn new(resource: Box<dyn AwsResource>) -> Selector {
    let items = vec![];

    let mut s = Selector {
        state: TableState::default(),
        items,
        resource,
        filtered_indexes: vec![],
        selected_indexes: vec![],
    };

    s.filter("");
    s
}

impl Selector {
    pub fn filter(&mut self, search_text: &str) {
        self.filtered_indexes = table::filter(search_text, &mut self.items, &mut self.state);
    }

    pub fn add_yaml(&mut self, yaml: Vec<Yaml>, search_text: &str) {
        for y in yaml {
            self.items.push(Item {
                row: self.resource.line(&y),
                match_string: self.resource.line(&y).join(" "),
                yaml: y.clone(),
            });
        }
        self.filter(search_text);
    }

    pub fn selected_key(&self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            return Some(
                self.resource
                    .resource_name(&self.items[self.filtered_indexes[index]].yaml),
            );
        }
        None
    }

    pub fn selected_names(&mut self) -> Option<String> {
        if let Some(index) = self.state.selected() {
            self.selected_indexes.push(self.filtered_indexes[index]);
        }

        if self.selected_indexes.len() == 0 {
            return None;
        }

        Some(
            self.items
                .iter()
                .enumerate()
                .filter(|(index, _)| self.selected_indexes.contains(index))
                .map(|(_, item)| self.resource.resource_name(&item.yaml))
                .collect::<Vec<String>>()
                .join("|"),
        )
    }

    pub fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect, api_call: &ApiCall)
    where
        B: Backend,
    {
        let filtered_items = table::filtered_items(&self.items, &self.filtered_indexes);
        let header = self.resource.header();
        let header_width = &self.resource.header_width();
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
                            Row::StyledData(row.iter(), Style::default().fg(Color::White))
                        }
                    }),
                ),
                &title,
                header_width,
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
            " [{} {}] {} ({}filtered {} from {}) - {} ",
            self.resource.service_name(),
            self.resource.resource_type_name(),
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

    pub fn item_detail(&self) -> show::Section {
        match self.state.selected() {
            Some(index) => {
                let item = &self.items[self.filtered_indexes[index]];
                self.resource.detail(&item.yaml)
            }
            None => show::Section::new_without_yaml(),
        }
    }

    pub fn filtered_len(&self) -> usize {
        self.filtered_indexes.len()
    }

    pub fn toggle_selected(&mut self) {
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
