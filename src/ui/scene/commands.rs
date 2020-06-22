use crate::error::Result;
use widget::util::table;

use crate::ui::{
    layout, select_next_scene, util,
    util::event::{Event, Events},
    widget, TypedTerminal, UiScene,
};
use crate::Opts;
use termion::event::Key;

pub struct Scene {
    pub search_text: String,

    opts: Opts,

    pub search: widget::search::TextBox,
    pub table: widget::commands::Selector,
    pub viewer: widget::viewer::Box,
}

pub fn new(opts: Opts) -> Scene {
    Scene {
        opts,
        search_text: "".to_string(),
        search: widget::search::new(),
        table: widget::commands::new(),
        viewer: widget::viewer::new(crate::show::Section::new_without_yaml()),
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
                Key::Char(key) => {
                    if key == '\n' {
                        if let Some(resource) = self.table.selected_resource() {
                            return Ok(Some(select_next_scene(
                                &self.opts.clone(),
                                &None,
                                resource,
                                events.tx.clone(),
                            )));
                        }
                    } else {
                        self.search_text.push(key);
                        self.table.filter(&self.search_text);
                    }
                }

                Key::Backspace => {
                    self.search_text.pop();
                    self.table.filter(&self.search_text);
                }

                Key::Esc => {
                    return Ok(Some(UiScene::Exit(None)));
                }

                Key::Ctrl('c') => return Ok(Some(UiScene::Exit(None))),

                Key::Down => {
                    table::next(self.table.filtered_len(), &mut self.table.state);
                    self.viewer = widget::viewer::new(self.table.command_detail());
                }

                Key::Up => {
                    table::previous(self.table.filtered_len(), &mut self.table.state);
                    self.viewer = widget::viewer::new(self.table.command_detail());
                }

                _ => {}
            }
        }

        if table::select_any(self.table.filtered_len(), &mut self.table.state) {
            self.viewer = widget::viewer::new(self.table.command_detail());
        }

        terminal.draw(|mut f| {
            let (search, table, viewer) = layout::main::layout(f.size());

            self.search.draw(&mut f, search, &self.search_text);
            self.table.draw(&mut f, table);
            self.viewer.draw(&mut f, viewer);
        })?;

        Ok(None)
    }
}
