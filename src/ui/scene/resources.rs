use super::SceneBase;
use crate::api::file::store_yaml;
use crate::color;
use crate::error::Result;
use crate::help::{Help, Helps};
use crate::log::Log;
use crate::service::prelude::Yaml;
use crate::service::AwsResource;
use crate::show::Section;
use crate::show::{Texts, Txt};
use crate::ui::widget::resources::Item;
use crate::ui::{
    layout, select_next_scene, util,
    util::event::{Event, Events},
    widget, NextScene, UiScene, UiState,
};
use chrono::prelude::*;
use rustbox::keyboard::Key;
use tui::backend::RustboxBackend;
use tui::terminal::Frame;
use widget::util::table;

#[derive(Clone)]
pub(crate) struct Scene {
    pub(crate) base: super::SceneBase,
    pub(crate) search_text: String,
    parameter: Option<String>,
    list_api_call: ListApiCall,
    get_api_call: Vec<DateTime<Local>>,
    getting_item_index: Vec<usize>,
    initial_request_count: usize,

    resource: Box<dyn AwsResource>,
    pub(crate) next_resource: Option<Box<dyn AwsResource>>,

    status: widget::Status,
    search: widget::Search,
    table: widget::Resources,
    log: widget::Log,
    info: widget::Info,
    viewer: widget::Viewer,
    help: widget::Help,

    helps: Helps,
    help_summary: Texts,
}

#[derive(Clone)]
pub(crate) enum ListApiCall {
    None,
    Completed,
    StillHave { next_token: String },
    Requesting { start: DateTime<Local> },
}

pub(in crate::ui) fn new(
    base: SceneBase,
    parameter: Option<String>,
    resource: Box<dyn AwsResource>,
    next_resource: Option<Box<dyn AwsResource>>,
    ui_state: &mut UiState,
) -> Scene {
    let initial_request_count = base.opts.list_request_count();
    let helps = Helps::new(all_helps(&*resource));
    let help_summary = helps.to_summary_text();
    let mut scene = Scene {
        base,

        search_text: "".to_string(),
        parameter,

        list_api_call: ListApiCall::None,
        get_api_call: vec![],
        getting_item_index: vec![],

        initial_request_count,
        resource: resource.clone(),
        next_resource,

        status: widget::status::new(),
        search: widget::search::new(),
        table: widget::resources::new(resource),
        log: widget::log::new(),
        info: widget::info::new(),
        viewer: widget::viewer::new(Section::new_without_yaml()),
        help: widget::help::new(),

        helps,
        help_summary,
    };
    scene.call_list_api(ui_state);
    scene
}

fn all_helps(resource: &dyn AwsResource) -> Vec<Help> {
    let mut all_helps = vec![];
    all_helps.append(&mut helps(resource));
    super::common_helps(&mut all_helps);
    all_helps
}

fn helps(resource: &dyn AwsResource) -> Vec<Help> {
    let mut helps = vec![];

    if resource.has_resource_url() {
        helps.push(Help::new(
            "O",
            Some("open console"),
            "open aws console in a browser for the selected resource",
        ))
    }

    if resource.has_get_api() {
        helps.push(Help::new(
            "G",
            Some("get detail"),
            "get all resource detail with get api",
        ));
    }

    helps.append(&mut vec![
        Help::new("Enter", Some("select"), "select resource to print the name"),
        Help::new("TAB", Some("mark"), "mark resource to select"),
        Help::new(
            "A",
            Some("fetch"),
            "fetch resources if there still have been resource to fetch",
        ),
        Help::new("R", Some("reload"), "reload resources"),
        Help::new(
            "E",
            Some("export"),
            "create yaml file of marked resources in the current directory",
        ),
        Help::new(
            "Y",
            Some("viewer mode"),
            "toggle viewer mode between yaml and summary",
        ),
    ]);

    helps
}

impl Scene {
    pub(crate) fn status(&self, current: bool) -> crate::show::Texts {
        let mut texts = vec![];
        if let Some(history) = &self.base.history {
            texts.append(&mut history.status(false).0);
            texts.push(Txt::raw(" > "));
        }

        texts.push(if current {
            Txt::colored(&self.resource.resource_full_name(), color::CURRENT)
        } else {
            Txt::raw(&self.resource.resource_full_name())
        });

        if current {
            if let Some(next_resource) = &self.next_resource {
                texts.push(Txt::raw(" > "));
                texts.push(Txt::raw(&next_resource.resource_full_name()))
            }
        }
        Texts(texts)
    }

    pub(in crate::ui) fn handle_events(
        &mut self,
        ui_state: &mut UiState,
        events: &mut Events,
        keys: Vec<rustbox::keyboard::Key>,
    ) -> Result<NextScene> {
        if let Some(overlay) = &mut self.base.overlay {
            return overlay.handle_events(ui_state, events, keys);
        }

        for key in keys {
            if let Some(next_scene) = self.handle_keys(key, ui_state)? {
                return Ok(next_scene);
            }
        }

        loop {
            match util::event::next(events) {
                Some(Event::ListResponse {
                    start,
                    yaml,
                    next_token,
                }) => {
                    self.handle_list_response(ui_state, start, yaml, next_token);
                    self.base.should_draw = true;
                }
                Some(Event::GetResponse {
                    start,
                    yaml,
                    resource_index,
                }) => {
                    self.handle_get_response(ui_state, start, yaml, resource_index);
                    self.base.should_draw = true;
                }
                Some(Event::Log(log)) => {
                    ui_state.logs.push(log);
                    self.base.should_draw = true;
                }
                None => break,
            }
        }

        if table::select_any(self.table.filtered_len(), &mut self.table.state) {
            let section = self.create_section_and_get_detail(ui_state);
            self.viewer = widget::viewer::new(section);
        }

        Ok(NextScene::Same)
    }

    fn create_section_and_get_detail(&mut self, ui_state: &mut UiState) -> Section {
        match self.table.selected_item() {
            Some(item) => {
                let section = self.resource.detail(
                    &item.list_yaml,
                    &item.get_yaml,
                    &self.base.opts.region_name(),
                );
                self.get_detail(&item, ui_state);
                section
            }
            None => Section::new_without_yaml(),
        }
    }

    fn get_detail(&mut self, item: &Item, ui_state: &mut UiState) {
        if !self.getting_item_index.contains(&item.index)
            && item.get_yaml.is_none()
            && self.resource.info().get_api.is_some()
        {
            self.call_get_api(&item, ui_state)
        }
    }

    fn handle_get_response(
        &mut self,
        ui_state: &mut UiState,
        start: DateTime<Local>,
        yaml: Yaml,
        resource_index: usize,
    ) {
        if !self.get_api_call.contains(&start) {
            return;
        }
        self.getting_item_index.retain(|i| *i != resource_index);

        let duration = Local::now().timestamp_millis() - start.timestamp_millis();
        let msg = format!("get {}({} ms).", self.resource.name(), duration);

        ui_state.logs.info(&format!("{} get complete.", msg));
        self.initial_request_count = 0;
        self.table.add_get_yaml(yaml, resource_index);

        let section = self.create_section_and_get_detail(ui_state);
        self.viewer = widget::viewer::new(section);
    }

    fn handle_list_response(
        &mut self,
        ui_state: &mut UiState,
        start: DateTime<Local>,
        yaml: crate::service::ResourceList,
        next_token: Option<String>,
    ) {
        if let ListApiCall::Requesting { start: scene_start } = self.list_api_call {
            if start != scene_start {
                return;
            }
        } else {
            return;
        }

        let duration = Local::now().timestamp_millis() - start.timestamp_millis();
        let msg = format!(
            "fetched {} {}({} ms).",
            yaml.len(),
            self.resource.name(),
            duration
        );

        self.list_api_call = match next_token {
            Some(next_token) => {
                ui_state.logs.info(&msg);
                ListApiCall::StillHave { next_token }
            }
            None => {
                ui_state.logs.info(&format!("{} fetch complete.", msg));
                self.initial_request_count = 0;
                ListApiCall::Completed
            }
        };

        self.table.add_resource_list(yaml, &self.search_text);

        if 0 < self.initial_request_count {
            self.initial_request_count -= 1;
            if 0 < self.initial_request_count {
                self.call_list_api(ui_state);
            }
        }
        self.get_initial_some(ui_state);
    }

    fn handle_keys(&mut self, key: Key, ui_state: &mut UiState) -> Result<Option<NextScene>> {
        use crate::ui::key_handler::*;

        if let Some(()) = text_input(key, &mut self.search_text) {
            self.table.filter(&self.search_text);
            return Ok(None);
        }

        if let Some(next) = common(key, &mut self.base, true) {
            return Ok(Some(next));
        }

        if let Some(row_selected) = table_move(
            key,
            self.table.filtered_len(),
            &mut self.table.state,
            self.table.last_height,
            &mut self.viewer.scroll,
            self.viewer.line_len,
        ) {
            if row_selected {
                let section = self.create_section_and_get_detail(ui_state);
                self.viewer = widget::viewer::new(section);
            }
            return Ok(None);
        }

        if let Some(log) = text_popup_open(
            key,
            &self.base,
            &ui_state.logs,
            &self.helps,
            Box::new(UiScene::Resource(self.clone())),
        ) {
            self.overlay(UiScene::TextPopup(log));
            return Ok(None);
        }

        if let Some(popup) = section_popup_open(
            key,
            &self.base,
            || {
                self.table
                    .selected_item_detail(&self.base.opts.region_name())
            },
            Box::new(UiScene::Resource(self.clone())),
        ) {
            self.overlay(UiScene::SectionPopup(popup));
            return Ok(None);
        }

        match key {
            Key::Enter => return Ok(Some(self.select_resource(ui_state))),
            Key::Tab => self.table.toggle_selected(),
            Key::Char('A') | Key::Ctrl('a') => self.call_list_api(ui_state),
            Key::Char('R') | Key::Ctrl('r') => self.reload(ui_state),
            Key::Char('E') | Key::Ctrl('e') => self.export(ui_state)?,
            Key::Char('Y') | Key::Ctrl('y') => ui_state.toggle_viewer_mode(),
            Key::Char('G') | Key::Ctrl('g') if self.resource.get_api().is_some() => {
                self.get_all(ui_state)
            }
            Key::Char('O') | Key::Ctrl('o') => self.open_resource_url()?,
            _ => ui_state.logs.error("key not assigned"),
        }

        Ok(None)
    }

    fn select_resource(&mut self, ui_state: &mut UiState) -> NextScene {
        return match &self.next_resource {
            Some(resource) => NextScene::Scene(select_next_scene(
                Some(Box::new(UiScene::Resource(self.clone()))),
                &self.base.opts,
                &self.table.selected_key(),
                resource.clone(),
                ui_state,
                self.base.tx.clone(),
            )),
            None => match self.table.selected_names_or_url(&self.base.opts) {
                Some(names_or_url) => NextScene::Exit(Some(names_or_url)),
                None => {
                    ui_state.logs.info("no item");
                    NextScene::Same
                }
            },
        };
    }

    fn open_resource_url(&self) -> Result<()> {
        if self.resource.has_resource_url() {
            if let Some(item) = self.table.selected_item() {
                let url = self.resource.console_url(
                    &item.list_yaml,
                    &item.get_yaml,
                    &self.base.opts.region_name(),
                );
                open::that(url)?;
            }
        }
        Ok(())
    }

    pub(in crate::ui) fn draw(
        &mut self,
        ui_state: &mut UiState,
        mut f: &mut Frame<RustboxBackend>,
    ) {
        let (status, (search, table, log), (info, viewer, help)) = layout::main::layout(f.size());

        self.status.draw(&mut f, status, self.status(true));
        self.search.draw(&mut f, search, &self.search_text);
        self.table.draw(&mut f, table, &self.list_api_call);
        self.log.draw(&mut f, log, ui_state.logs.to_text(2));
        self.viewer.draw(&mut f, &ui_state.viewer_mode, viewer);
        self.info.draw(
            &mut f,
            info,
            &self.base.opts.region_name(),
            self.viewer.scroll,
            self.viewer.line_len,
            true,
            &ui_state,
        );
        self.help.draw(&mut f, help, &self.help_summary);

        if let Some(ui) = &mut self.base.overlay {
            ui.draw(ui_state, f);
        }
    }

    fn reload(&mut self, ui_state: &mut UiState) {
        self.list_api_call = ListApiCall::None;
        self.table.clear();
        self.call_list_api(ui_state);
    }

    fn export(&mut self, ui_state: &mut UiState) -> Result<()> {
        for (index, yaml) in self.table.selected_yamls().iter().enumerate() {
            let name = format!(
                "{}-{}-{}",
                self.resource.command_name(),
                self.resource.resource_type_name(),
                index + 1
            );
            store_yaml(yaml, &name)?;
            ui_state.logs.info(&format!(
                "{} stored in yaml file {}.",
                self.resource.resource_name(yaml),
                name
            ));
        }
        Ok(())
    }

    fn get_all(&mut self, ui_state: &mut UiState) {
        for item in &self.table.items.clone() {
            self.get_detail(item, ui_state);
        }
    }

    fn get_initial_some(&mut self, ui_state: &mut UiState) {
        let max = self.base.opts.get_request_count();
        for (index, item) in self.table.items.clone().iter().enumerate() {
            if index > max {
                break;
            }
            self.get_detail(item, ui_state)
        }
    }

    fn call_list_api(&mut self, ui_state: &mut UiState) {
        let next_token = match &self.list_api_call {
            ListApiCall::None => None,
            ListApiCall::StillHave { next_token } => Some(next_token.to_owned()),
            ListApiCall::Completed => {
                ui_state
                    .logs
                    .info(&format!("no more {}.", self.resource.name()));
                return;
            }
            ListApiCall::Requesting { start: _start } => {
                ui_state.logs.info("requesting");
                return;
            }
        };

        let resource = self.resource.clone();
        let mut tx = self.base.tx.clone();
        let parameter = self.parameter.clone();
        let opts = self.base.opts.clone();
        let start = Local::now();

        tokio::spawn(async move {
            match crate::api::list::call(&*resource, &parameter, &opts, next_token).await {
                Ok((yaml, next_token)) => {
                    let _ = tx
                        .send(Event::ListResponse {
                            start,
                            yaml,
                            next_token,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(Event::Log(Log::error(&format!("{:?}", e)))).await;
                }
            }
        });

        ui_state.api_count_up();
        self.list_api_call = ListApiCall::Requesting { start };
    }

    fn call_get_api(&mut self, item: &Item, ui_state: &mut UiState) {
        let resource_index = item.index;
        let list_yaml = item.list_yaml.clone();
        let resource = self.resource.clone();
        let mut tx = self.base.tx.clone();
        let opts = self.base.opts.clone();
        let start = Local::now();

        tokio::spawn(async move {
            match crate::api::get::call(&*resource, &list_yaml, &opts).await {
                Ok(yaml) => {
                    let _ = tx
                        .send(Event::GetResponse {
                            start,
                            yaml,
                            resource_index,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(Event::Log(Log::error(&format!("{:?}", e)))).await;
                }
            }
        });

        ui_state.api_count_up();

        self.get_api_call.push(start);
        self.getting_item_index.push(item.index);
    }

    pub(crate) fn overlay(&mut self, other: UiScene) {
        self.base.overlay = Some(Box::new(other));
    }
}
