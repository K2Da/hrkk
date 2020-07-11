use super::SceneBase;
use crate::api::file::store_yaml;
use crate::color;
use crate::error::Result;
use crate::help::{Help, Helps};
use crate::log::Log;
use crate::service::AwsResource;
use crate::show::{Texts, Txt};
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
use yaml_rust::Yaml;

#[derive(Clone)]
pub(crate) struct Scene {
    pub(crate) base: super::SceneBase,
    pub(crate) search_text: String,
    parameter: Option<String>,
    api_call: ApiCall,
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
pub(crate) enum ApiCall {
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
    let initial_request_count = base.opts.request_count();
    let helps = Helps::new(all_helps());
    let help_summary = helps.to_summary_text();
    let mut scene = Scene {
        base,

        search_text: "".to_string(),
        parameter,

        api_call: ApiCall::None,
        initial_request_count,
        resource: resource.clone(),
        next_resource,

        status: widget::status::new(),
        search: widget::search::new(),
        table: widget::resources::new(resource),
        log: widget::log::new(),
        info: widget::info::new(),
        viewer: widget::viewer::new(crate::show::Section::new_without_yaml()),
        help: widget::help::new(),

        helps,
        help_summary,
    };
    scene.call_api(ui_state);
    scene
}

fn all_helps() -> Vec<Help> {
    let mut all_helps = vec![];
    all_helps.append(&mut helps());
    super::common_helps(&mut all_helps);
    all_helps
}

fn helps() -> Vec<Help> {
    vec![
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
    ]
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
        ui_state: &mut super::super::UiState,
        events: &mut Events,
        key: Option<rustbox::keyboard::Key>,
    ) -> Result<NextScene> {
        if let Some(overlay) = &mut self.base.overlay {
            return overlay.handle_events(ui_state, events, key);
        }

        if let Some(key) = key {
            if let Some(next_scene) = self.handle_keys(key, ui_state)? {
                return Ok(next_scene);
            }
        }

        loop {
            match util::event::next(events) {
                Some(Event::DescribeResponse {
                    start,
                    yaml,
                    next_token,
                }) => {
                    self.handle_response(ui_state, start, yaml, next_token);
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
            self.viewer = widget::viewer::new(self.table.item_detail());
        }

        Ok(NextScene::Same)
    }

    fn handle_response(
        &mut self,
        ui_state: &mut UiState,
        start: DateTime<Local>,
        yaml: Vec<Yaml>,
        next_token: Option<String>,
    ) {
        if let ApiCall::Requesting { start: scene_start } = self.api_call {
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

        self.api_call = match next_token {
            Some(next_token) => {
                ui_state.logs.info(&msg);
                ApiCall::StillHave { next_token }
            }
            None => {
                ui_state.logs.info(&format!("{} fetch complete.", msg));
                self.initial_request_count = 0;
                ApiCall::Completed
            }
        };

        self.table.add_yaml(yaml, &self.search_text);

        if 0 < self.initial_request_count {
            self.initial_request_count -= 1;
            if 0 < self.initial_request_count {
                self.call_api(ui_state);
            }
        }
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
                self.viewer = widget::viewer::new(self.table.item_detail());
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
            || self.table.item_detail(),
            Box::new(UiScene::Resource(self.clone())),
        ) {
            self.overlay(UiScene::SectionPopup(popup));
            return Ok(None);
        }

        match key {
            Key::Enter => return Ok(Some(self.select_resource(ui_state))),
            Key::Tab => self.table.toggle_selected(),
            Key::Char('A') | Key::Ctrl('a') => self.call_api(ui_state),
            Key::Char('R') | Key::Ctrl('r') => self.reload(ui_state),
            Key::Char('E') | Key::Ctrl('e') => self.export(ui_state)?,
            Key::Char('Y') | Key::Ctrl('y') => ui_state.toggle_viewer_mode(),
            _ => ui_state.logs.error("not assigned key"),
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
            None => match self.table.selected_names(&self.base.opts.delimiter()) {
                Some(selected_names) => NextScene::Exit(Some(selected_names)),
                None => {
                    ui_state.logs.info("no item");
                    NextScene::Same
                }
            },
        };
    }

    pub(in crate::ui) fn draw(
        &mut self,
        ui_state: &mut UiState,
        mut f: &mut Frame<RustboxBackend>,
    ) {
        let (status, (search, table, log), (info, viewer, help)) = layout::main::layout(f.size());

        self.status.draw(&mut f, status, self.status(true));
        self.search.draw(&mut f, search, &self.search_text);
        self.table.draw(&mut f, table, &self.api_call);
        self.log.draw(&mut f, log, ui_state.logs.to_text(2));
        self.viewer.draw(&mut f, &ui_state.viewer_mode, viewer);
        self.info.draw(
            &mut f,
            info,
            self.base.opts.region().unwrap().name(),
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
        self.api_call = ApiCall::None;
        self.table.clear();
        self.call_api(ui_state);
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
                self.resource.resource_name(&yaml),
                name
            ));
        }
        Ok(())
    }

    fn call_api(&mut self, ui_state: &mut UiState) {
        let next_token = match &self.api_call {
            ApiCall::None => None,
            ApiCall::StillHave { next_token } => Some(next_token.to_owned()),
            ApiCall::Completed => {
                ui_state
                    .logs
                    .info(&format!("no more {}.", self.resource.name()));
                return;
            }
            ApiCall::Requesting { start: _start } => {
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
                        .send(Event::DescribeResponse {
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
        self.api_call = ApiCall::Requesting { start };
    }

    pub(crate) fn overlay(&mut self, other: UiScene) {
        self.base.overlay = Some(Box::new(other));
    }
}
