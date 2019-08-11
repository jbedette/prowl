use crate::components::{CharRenderer, Health, Named};
use crate::resources::game_data::GameData;
use crate::shared::Vector2;
use crate::ui::{
    markers::{ConsoleUI, InteractiveUI, StatusUI},
    panel::{Panel, Widget},
};
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
        WriteStorage<'a, Panel>,
        WriteStorage<'a, InteractiveUI>,
        Write<'a, Console>,
        Write<'a, GameData>,
        Write<'a, Window>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut panels, mut interactive_uis, mut console, mut game_data, mut window, entities) =
            data;

        let mut entities_to_remove = vec![];
        let mut create = false;
        let count = panels.join().count() as i32;
        for (_panel, _interactive, entity) in (&panels, &interactive_uis, &entities).join() {
            //println!("INTERACTIVE UI ACTIVE");
            let key = tcod_input::get(&mut window.root);
            use tcod_input::InputCode;
            use InputCode::*;
            game_data.state_change_request = Some(StateChangeRequest::WaitForUI);
            match key {
                // ESCAPE TODO rename Quit to Escape
                One => {
                    //entities_to_remove.push(entity);
                    create = true;
                    println!("one");
                }
                Back => {
                    println!("back");
                    game_data.state_change_request = Option::None;
                    entities_to_remove.push(entity);
                }
                /*
                Quit => {
                    game_data.state_change_request = Option::None;
                    entities_to_remove.push(entity);
                }
                */
                // Console
                ConsoleSrollUp => console.scroll(-1),
                ConsoleSrollDown => console.scroll(1),
                _ => (),
            }
        }
        if create {
            let window = entities.create();
            let mut panel = Panel::new(
                "[X] to close",
                Vector2::new(5 + (count * 3), 5 + ((count - 1) * 3)),
                Vector2::new(20, 20),
                CharRenderer::ui_body(),
                CharRenderer::ui_border(),
                count, //todo:replace with smart id system
            );
            panel
                .widgets
                .push(Widget::text_box(&format! {"hi {}", count}));
            panels.insert(window, panel);
            interactive_uis.insert(window, InteractiveUI::default());
        }
        for entity in entities_to_remove {
            entities.delete(entity);
        }
    }
}
