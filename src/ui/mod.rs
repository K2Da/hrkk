mod layout;
use tui::backend::RustboxBackend;
mod key_handler;
mod scene;
mod util;
pub mod widget;

use crate::opts::Opts;
use crate::service::AwsResource;
use crate::ui::scene::SceneBase;
use crate::ui::util::event::{Event, Events};
use rustbox::keyboard::Key;
use std::time::Duration;
use tokio::sync::mpsc;
use tui::Frame;
use tui::Terminal;

use crate::error::Result;

pub(crate) type TypedTerminal = Terminal<RustboxBackend>;

pub(crate) enum NextScene {
    Scene(UiScene),
    Same,
    Exit(Option<String>),
}

#[derive(Clone)]
pub(crate) enum UiScene {
    Commands(scene::commands::Scene),
    Resource(scene::resources::Scene),
    OptionPopup(scene::list_option::Scene),
    TextPopup(scene::text_popup::Scene),
    SectionPopup(scene::section_popup::Scene),
}

use UiScene::*;
impl UiScene {
    pub(crate) fn status(&self, current: bool) -> crate::show::Texts {
        match self {
            Commands(scene) => scene.status(current),
            Resource(scene) => scene.status(current),
            OptionPopup(_) | TextPopup(_) | SectionPopup(_) => panic!("status"),
        }
    }

    pub(crate) fn overlay(&mut self, other: UiScene) {
        match self {
            Commands(scene) => scene.overlay(other),
            Resource(scene) => scene.overlay(other),
            OptionPopup(_) | TextPopup(_) | SectionPopup(_) => panic!("overlay"),
        }
    }

    pub(in crate::ui) fn draw(
        &mut self,
        ui_state: &mut UiState,
        mut f: &mut Frame<RustboxBackend>,
    ) {
        match self {
            Commands(scene) => scene.draw(ui_state, &mut f),
            Resource(scene) => scene.draw(ui_state, &mut f),
            OptionPopup(scene) => scene.draw(&mut f),
            TextPopup(scene) => scene.draw(&mut f),
            SectionPopup(scene) => scene.draw(ui_state, &mut f),
        }
    }

    pub(in crate::ui) fn handle_events(
        &mut self,
        ui_state: &mut UiState,
        events: &mut Events,
        keys: Vec<Key>,
    ) -> Result<NextScene> {
        match self {
            Commands(scene) => Ok(scene.handle_events(ui_state, events, keys)?),
            Resource(scene) => Ok(scene.handle_events(ui_state, events, keys)?),
            OptionPopup(scene) => Ok(scene.handle_events(ui_state, keys)?),
            TextPopup(scene) => Ok(scene.handle_events(keys)),
            SectionPopup(scene) => Ok(scene.handle_events(keys)),
        }
    }

    fn take_should_draw(&mut self) -> bool {
        let base = self.base_mut();
        let should_draw = base.should_draw;
        base.should_draw = false;
        should_draw
    }

    fn set_should_draw(&mut self) {
        let base = self.base_mut();
        base.should_draw = true;
    }

    fn base_mut(&mut self) -> &mut SceneBase {
        match self {
            Commands(scene) => &mut scene.base,
            Resource(scene) => &mut scene.base,
            OptionPopup(scene) => &mut scene.base,
            TextPopup(scene) => &mut scene.base,
            SectionPopup(scene) => &mut scene.base,
        }
    }
}

pub(in crate::ui) fn select_next_scene(
    current_scene: Option<Box<UiScene>>,
    opts: &Opts,
    parameter: &Option<String>,
    resource: Box<dyn AwsResource>,
    ui_state: &mut UiState,
    tx: mpsc::Sender<Event>,
) -> UiScene {
    use crate::service::ExecuteTarget;

    match parameter {
        Some(parameter) => UiScene::Resource(scene::resources::new(
            SceneBase::with_history(opts.clone(), tx, current_scene),
            Some(parameter.to_string()),
            resource,
            None,
            ui_state,
        )),
        None => match resource.without_param(opts) {
            ExecuteTarget::ExecuteThis { parameter } => UiScene::Resource(scene::resources::new(
                SceneBase::with_history(opts.clone(), tx, current_scene),
                parameter,
                resource,
                None,
                ui_state,
            )),
            ExecuteTarget::ParameterFromList {
                option_name,
                option_list,
            } => {
                let option = scene::list_option::new(
                    SceneBase::with_history(opts.clone(), tx, current_scene.clone()),
                    resource.clone(),
                    &option_name,
                    &option_list,
                );

                match current_scene {
                    Some(mut scene) => {
                        scene.overlay(UiScene::OptionPopup(option));
                        *scene
                    }
                    None => UiScene::OptionPopup(option),
                }
            }
            ExecuteTarget::ParameterFromResource { param_resource } => {
                UiScene::Resource(scene::resources::new(
                    SceneBase::with_history(opts.clone(), tx, current_scene),
                    None,
                    param_resource,
                    Some(resource),
                    ui_state,
                ))
            }
            ExecuteTarget::Null => panic!("null target"),
        },
    }
}

#[derive(Clone, Debug)]
pub(crate) enum ViewerMode {
    Yaml,
    Summary,
}

struct UiState {
    viewer_mode: ViewerMode,
    logs: crate::log::Logs,
    pub(in crate::ui) api_count: usize,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            viewer_mode: ViewerMode::Summary,
            logs: crate::log::Logs::new(),
            api_count: 0,
        }
    }

    pub fn toggle_viewer_mode(&mut self) {
        self.viewer_mode = match self.viewer_mode {
            ViewerMode::Yaml => ViewerMode::Summary,
            ViewerMode::Summary => ViewerMode::Yaml,
        }
    }

    pub fn api_count_up(&mut self) {
        self.api_count += 1;
    }
}

pub(crate) async fn tui(
    opts: Opts,
    parameter: Option<String>,
    resource: Option<Box<dyn AwsResource>>,
) -> Result<()> {
    let mut terminal = util::terminal()?;
    let mut events = util::event::new();
    let mut ui_state = UiState::new();

    let mut scene = match resource {
        Some(resource) => select_next_scene(
            None,
            &opts,
            &parameter,
            resource,
            &mut ui_state,
            events.tx.clone(),
        ),
        None => UiScene::Commands(scene::commands::new(SceneBase::minimum(
            opts.clone(),
            events.tx.clone(),
        ))),
    };

    let output_text;
    let mut keys = vec![];

    loop {
        match scene.handle_events(&mut ui_state, &mut events, keys)? {
            NextScene::Same => (),
            NextScene::Scene(next_scene) => scene = next_scene,
            NextScene::Exit(output) => {
                output_text = output;
                break;
            }
        }

        if scene.take_should_draw() {
            terminal.draw(|mut f| scene.draw(&mut ui_state, &mut f))?;
        }

        keys = peek_event(&mut terminal, &mut scene);
    }

    drop(terminal);

    if let Some(text) = output_text {
        print!("{}", text);
    }

    Ok(())
}

fn peek_event(terminal: &mut Terminal<RustboxBackend>, scene: &mut UiScene) -> Vec<Key> {
    let mut keys = vec![];
    for _ in 0..10 {
        let event = terminal
            .backend()
            .rustbox()
            .peek_event(Duration::from_millis(10), false);
        match event {
            Ok(rustbox::Event::KeyEvent(event_key)) => {
                scene.set_should_draw();
                keys.push(event_key);
            }

            Ok(rustbox::Event::ResizeEvent(_, _)) => {
                let _ = terminal.draw(|mut _f| ());
                scene.set_should_draw();
                continue;
            }

            _ => {
                if keys.len() > 0 {
                    return keys;
                }
            }
        }
    }
    keys
}
