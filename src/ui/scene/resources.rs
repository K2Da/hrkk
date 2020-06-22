use crate::error::Result;
use crate::service::AwsResource;
use crate::ui::{
    layout, select_next_scene, util,
    util::event::{Event, Events},
    widget, TypedTerminal, UiScene,
};
use crate::Opts;
use termion::event::Key;
use tokio::sync::mpsc;
use widget::util::table;

pub struct Scene {
    pub search_text: String,
    parameter: Option<String>,
    opts: Opts,
    api_call: ApiCall,
    resource: Box<dyn AwsResource>,
    pub next_resource: Option<Box<dyn AwsResource>>,
    tx: mpsc::Sender<Event>,
    search: widget::search::TextBox,
    table: widget::resources::Selector,
    viewer: widget::viewer::Box,
}

impl Scene {
    pub fn set_exit_key(&self, events: &mut Events) {
        match self.next_resource {
            Some(_) => events.set_exit_key(false, false),
            None => events.set_exit_key(true, false),
        }
    }
}

pub enum ApiCall {
    Completed,
    StillHave { next_token: String },
    Requesting,
}

fn send_api(
    resource: Box<dyn AwsResource>,
    mut tx: mpsc::Sender<Event>,
    parameter: Option<String>,
    opts: Opts,
    next_token: Option<String>,
) -> ApiCall {
    tokio::spawn(async move {
        match crate::service::fetch(&*resource, &parameter, &opts, next_token).await {
            Ok((yaml, next_token)) => {
                if let Err(_) = tx.send(Event::DescribeResponse { yaml, next_token }).await {
                    return;
                }
            }
            Err(e) => {
                if let Err(_) = tx.send(Event::Err(e)).await {
                    return;
                }
            }
        }
    });
    ApiCall::Requesting
}

pub fn new(
    parameter: Option<String>,
    opts: Opts,
    resource: Box<dyn AwsResource>,
    next_resource: Option<Box<dyn AwsResource>>,
    tx: mpsc::Sender<Event>,
) -> Scene {
    let cloned_resource = resource.clone();
    let api_call = send_api(
        cloned_resource,
        tx.clone(),
        parameter.clone(),
        opts.clone(),
        None,
    );

    Scene {
        search_text: "".to_string(),
        parameter,
        opts,
        api_call,
        resource: resource.clone(),
        tx,
        next_resource,
        search: widget::search::new(),
        table: widget::resources::new(resource),
        viewer: widget::viewer::new(crate::show::Section::new_without_yaml()),
    }
}

impl Scene {
    pub async fn draw(
        &mut self,
        terminal: &mut TypedTerminal,
        events: &mut Events,
    ) -> Result<Option<UiScene>> {
        match util::event::next(events).await {
            Some(Event::Input(key)) => match key {
                Key::Char('\n') => {
                    return match &self.next_resource {
                        Some(resource) => Ok(Some(select_next_scene(
                            &self.opts,
                            &self.table.selected_key(),
                            resource.clone(),
                            self.tx.clone(),
                        ))),
                        None => Ok(Some(UiScene::Exit(self.table.selected_names()))),
                    }
                }

                Key::Char('A') => self.call_api(),

                Key::Char('\t') => self.table.toggle_selected(),

                Key::Char(key) => {
                    self.search_text.push(key);
                    self.table.filter(&self.search_text);
                }

                Key::Backspace => {
                    self.search_text.pop();
                    self.table.filter(&self.search_text);
                }

                Key::Esc => {
                    return Ok(Some(UiScene::Commands(crate::ui::scene::commands::new(
                        self.opts.clone(),
                    ))))
                }

                Key::Ctrl('c') => return Ok(Some(UiScene::Exit(None))),

                Key::Down => {
                    table::next(self.table.filtered_len(), &mut self.table.state);
                    self.viewer = widget::viewer::new(self.table.item_detail());
                }

                Key::Up => {
                    table::previous(self.table.filtered_len(), &mut self.table.state);
                    self.viewer = widget::viewer::new(self.table.item_detail());
                }

                _ => {}
            },
            Some(Event::DescribeResponse { yaml, next_token }) => {
                self.api_call = match next_token {
                    Some(next_token) => ApiCall::StillHave { next_token },
                    None => ApiCall::Completed,
                };
                self.table.add_yaml(yaml, &self.search_text);
            }
            _ => (),
        }

        if table::select_any(self.table.filtered_len(), &mut self.table.state) {
            self.viewer = widget::viewer::new(self.table.item_detail());
        }

        terminal.draw(|mut f| {
            let (search, table, viewer) = layout::main::layout(f.size());

            self.search.draw(&mut f, search, &self.search_text);
            self.table.draw(&mut f, table, &self.api_call);
            self.viewer.draw(&mut f, viewer);
        })?;

        Ok(None)
    }

    pub fn call_api(&mut self) {
        match &self.api_call {
            ApiCall::StillHave { next_token } => {
                self.api_call = send_api(
                    self.resource.clone(),
                    self.tx.clone(),
                    self.parameter.clone(),
                    self.opts.clone(),
                    Some(next_token.to_owned()),
                );
            }
            _ => (),
        }
    }
}
