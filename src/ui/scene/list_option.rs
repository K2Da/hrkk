use crate::error::Result;
use crate::opts::Opts;
use crate::service::AwsResource;
use crate::ui::{
    layout, select_next_scene, util,
    util::event::{Event, Events},
    widget, TypedTerminal, UiScene,
};
use termion::event::Key;
use tokio::sync::mpsc;
use widget::util::list;

pub struct Scene {
    opts: Opts,

    resource: Box<dyn AwsResource>,
    pub option_list: widget::list::Box,
    tx: mpsc::Sender<Event>,
}

pub fn new(
    opts: Opts,
    resource: Box<dyn AwsResource>,
    option_name: &str,
    option_list: &Vec<String>,
    tx: mpsc::Sender<Event>,
) -> Scene {
    Scene {
        opts,
        resource,
        option_list: widget::list::new(option_name, option_list),
        tx,
    }
}

impl Scene {
    pub async fn draw(
        &mut self,
        terminal: &mut TypedTerminal,
        events: &mut Events,
    ) -> Result<Option<UiScene>> {
        if let Some(Event::Input(key)) = util::event::next(events).await {
            match key {
                Key::Esc => {
                    return Ok(Some(UiScene::Commands(crate::ui::scene::commands::new(
                        self.opts.clone(),
                    ))))
                }

                Key::Char('\n') => {
                    return Ok(Some(select_next_scene(
                        &self.opts,
                        &self.option_list.selected_item(),
                        self.resource.clone(),
                        self.tx.clone(),
                    )))
                }

                Key::Ctrl('c') => return Ok(Some(UiScene::Exit(None))),

                Key::Down => {
                    list::next(&mut self.option_list.state, self.option_list.items.len());
                }

                Key::Up => {
                    list::previous(&mut self.option_list.state, self.option_list.items.len())
                }

                _ => {}
            }
        }

        list::select_any(&mut self.option_list.state, self.option_list.items.len());

        terminal.draw(|mut f| {
            let popup = layout::popup::layout(50, 50, f.size());
            self.option_list.draw(&mut f, popup);
        })?;

        Ok(None)
    }
}
