use super::SceneBase;
use crate::error::Result;
use crate::service::AwsResource;
use crate::ui::UiState;
use crate::ui::{layout, select_next_scene, widget, NextScene};
use rustbox::keyboard::Key;
use tui::backend::RustboxBackend;
use tui::terminal::Frame;
use widget::util::list;

#[derive(Clone)]
pub(crate) struct Scene {
    pub(crate) base: super::SceneBase,
    resource: Box<dyn AwsResource>,
    pub(crate) option_list: widget::TextList,
}

pub(crate) fn new(
    base: SceneBase,
    resource: Box<dyn AwsResource>,
    option_name: &str,
    option_list: &Vec<String>,
) -> Scene {
    Scene {
        base,
        resource,
        option_list: widget::text_list::new(option_name, option_list),
    }
}

impl Scene {
    pub(in crate::ui) fn handle_events(
        &mut self,
        ui_state: &mut UiState,
        keys: Vec<rustbox::keyboard::Key>,
    ) -> Result<NextScene> {
        for key in keys {
            match key {
                Key::Esc => return Ok(self.base.back_or_root_menu()),

                Key::Enter => {
                    return Ok(NextScene::Scene(select_next_scene(
                        self.base.history.clone(),
                        &self.base.opts,
                        &self.option_list.selected_item(),
                        self.resource.clone(),
                        ui_state,
                        self.base.tx.clone(),
                    )))
                }

                Key::Ctrl('c') | Key::Char('C') => return Ok(NextScene::Exit(None)),

                Key::Down | Key::Ctrl('j') | Key::Char('J') => {
                    list::next(&mut self.option_list.state, self.option_list.items.len());
                }

                Key::Up | Key::Ctrl('k') | Key::Char('K') => {
                    list::previous(&mut self.option_list.state, self.option_list.items.len())
                }

                _ => {}
            }
        }

        list::select_any(&mut self.option_list.state, self.option_list.items.len());

        Ok(NextScene::Same)
    }

    pub(crate) fn draw(&mut self, mut f: &mut Frame<RustboxBackend>) {
        let popup = layout::popup::layout(50, 50, f.size());
        self.option_list.draw(&mut f, popup);
    }
}
