use crate::components::{CharRenderer, Health, Named};
use crate::resources::game_data::GameData;
use crate::ui::{
    markers::{ConsoleUI, InteractiveUI, StatusUI},
    panel::{Panel, Widget},
};
use crate::shared::Vector2;
use specs::prelude::*;

use crate::console::resource::Console;
pub struct ConsoleWindowSystem;
impl<'a> System<'a> for ConsoleWindowSystem {
    type SystemData = (
        Read<'a, Console>,
        WriteStorage<'a, Panel>,
        ReadStorage<'a, ConsoleUI>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (console, mut panels, console_ui) = data;

        for (panel, _console_ui) in (&mut panels, &console_ui).join() {
            panel.widgets = vec![];
            let mut text = String::from("");
            let mut i = 0;
            // TODO this is garbage af
            for log in &console.logs {
                if i >= console.y_offset - panel.bounds.y + 3 {
                    if text.is_empty() {
                        text = format!("> {}", &log.message);
                    } else {
                        text = format!("{}\n> {}", text, &log.message);
                    }
                }
                i += 1;
            }
            panel.widgets.push(Widget::text_box(&text));
        }
    }
}

pub struct StatusWindowSystem;
impl<'a> System<'a> for StatusWindowSystem {
    type SystemData = (
        WriteStorage<'a, Panel>,
        ReadStorage<'a, StatusUI>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Health>,
        Read<'a, GameData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut panel, status, nameds, healths, _game_data, entities) = data;
        for (panel, _status, entity) in (&mut panel, &status, &entities).join() {
            // clear
            panel.widgets = vec![];
            /* NAME */
            {
                let name = nameds.get(entity);
                let name = Named::name_or_noname(name);
                let name = format!("Name: {}", name);
                panel.widgets.push(Widget::label(&name));
            } /* HEALTH */
            {
                let health = healths.get(entity);
                if let Some(health) = health {
                    let health = format!("HP: {}/{}", health.current, health.max,);
                    panel.widgets.push(Widget::label(&health));
                }
            }
        }
    }
}

use crate::input::tcod_input;
use crate::resources::{game_data::StateChangeRequest, Window};

pub struct InteractiveUISystem;
impl<'a> System<'a> for InteractiveUISystem {
    type SystemData = (
        ReadStorage<'a, Panel>,
        ReadStorage<'a, InteractiveUI>,
        Write<'a, Console>,
        Write<'a, GameData>,
        Write<'a, Window>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (panels, interactives, mut console, mut game_data, mut window, entities) = data;

        let mut entities_to_remove = vec![];
        for (_panel, _interactive, entity) in (&panels, &interactives, &entities).join() {
            println!("INTERACTIVE UI ACTIVE");
            let key = tcod_input::get(&mut window.root);
            use tcod_input::InputCode;
            use InputCode::*;
            game_data.state_change_request = Some(StateChangeRequest::WaitForUI);
            match key {
                // ESCAPE TODO rename Quit to Escape
                One => {
                    let mut panel = Panel::new(
                        "[ESC] to close",
                        Vector2::new(5, 5),
                        Vector2::new(20, 20),
                        CharRenderer::ui_body(),
                        CharRenderer::ui_border(),
                    );
                    panel.widgets.push(Widget::text_box("hi"));
                }
                Quit => {
                    game_data.state_change_request = Option::None;
                    entities_to_remove.push(entity);
                }
                // Console
                ConsoleSrollUp => console.scroll(-1),
                ConsoleSrollDown => console.scroll(1),
                _ => (),
            }
        }
        for entity in entities_to_remove {
            entities.delete(entity);
        }
    }
}
