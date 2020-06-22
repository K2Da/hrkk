mod layout;
mod scene;
mod util;
mod widget;

use crate::opts::Opts;
use crate::service::AwsResource;
use crate::ui::util::event::{Event, Events};
use std::io;
use termion::{raw::RawTerminal, screen::AlternateScreen};
use tokio::sync::mpsc;
use tui::{backend::TermionBackend, Terminal};

use crate::error::Result;

pub type TypedTerminal = Terminal<TermionBackend<AlternateScreen<RawTerminal<io::Stdout>>>>;

async fn draw(
    scene: &mut UiScene,
    terminal: &mut TypedTerminal,
    events: &mut Events,
) -> Result<Option<UiScene>> {
    match scene {
        UiScene::Commands(scene) => scene.draw(terminal, events).await,
        UiScene::Resource(scene) => scene.draw(terminal, events).await,
        UiScene::OptionPopup(scene) => scene.draw(terminal, events).await,
        UiScene::Exit(_) => Ok(None),
    }
}

pub enum UiScene {
    Commands(scene::commands::Scene),
    Resource(scene::resources::Scene),
    OptionPopup(scene::list_option::Scene),
    Exit(Option<String>),
}

impl UiScene {
    pub fn set_exit_key(&self, events: &mut Events) {
        match self {
            UiScene::Commands(_) => events.set_exit_key(false, true),
            UiScene::Resource(scene) => scene.set_exit_key(events),
            UiScene::OptionPopup(_) => events.set_exit_key(false, false),
            UiScene::Exit(_) => (),
        }
    }
}

pub fn select_next_scene(
    opts: &Opts,
    parameter: &Option<String>,
    resource: Box<dyn AwsResource>,
    tx: mpsc::Sender<Event>,
) -> UiScene {
    use crate::service::ExecuteTarget;

    match parameter {
        Some(parameter) => UiScene::Resource(scene::resources::new(
            Some(parameter.to_string()),
            opts.clone(),
            resource,
            None,
            tx,
        )),
        None => {
            match resource.without_param(opts) {
                ExecuteTarget::ExecuteThis { parameter } => UiScene::Resource(
                    scene::resources::new(parameter, opts.clone(), resource, None, tx),
                ),
                ExecuteTarget::ParameterFromList {
                    option_name,
                    option_list,
                } => UiScene::OptionPopup(scene::list_option::new(
                    opts.clone(),
                    resource.clone(),
                    &option_name,
                    &option_list,
                    tx,
                )),
                ExecuteTarget::ParameterFromResource { param_resource } => UiScene::Resource(
                    scene::resources::new(None, opts.clone(), param_resource, Some(resource), tx),
                ),
                ExecuteTarget::Null => panic!(),
            }
        }
    }
}

pub async fn tui(
    opts: Opts,
    parameter: Option<String>,
    resource: Option<Box<dyn AwsResource>>,
) -> Result<()> {
    let mut terminal = util::terminal()?;
    let mut events = util::event::new();
    let mut scene = match resource {
        Some(resource) => select_next_scene(&opts, &parameter, resource, events.tx.clone()),
        None => UiScene::Commands(scene::commands::new(opts.clone())),
    };
    scene.set_exit_key(&mut events);

    let output_text;

    loop {
        match draw(&mut scene, &mut terminal, &mut events).await? {
            Some(next_scene) => match next_scene {
                UiScene::Exit(ouput) => {
                    output_text = ouput;
                    break;
                }
                _ => {
                    scene = next_scene;
                    scene.set_exit_key(&mut events);
                }
            },
            None => (),
        }
    }

    drop(events);
    drop(terminal);

    if let Some(text) = output_text {
        println!("{}", text);
    }
    Ok(())
}
