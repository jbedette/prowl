use crate::ui::{
    panel::{
        Panel,
        Widget,
    },
    markers::{
        StatusUI,
        ConsoleUI,
        InteractiveUI,
    },
};
use crate::components::{
    Named,
    Health,
};
use specs::prelude::*;
use crate::resources::game_data::GameData;

use crate::console::resource::Console;
pub struct ConsoleWindowSystem;
impl<'a> System<'a> for ConsoleWindowSystem {
    type SystemData = (
        Read<'a, Console>,
        WriteStorage<'a, Panel>,
        ReadStorage<'a, ConsoleUI>,
        );
    fn run(&mut self, data: Self::SystemData) {
        let (
            console,
            mut panels,
            console_ui
            ) = data;

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
        let (
            mut panel,
            status,
            nameds,
            healths,
            _game_data,
            entities,
            ) = data;
        for (panel, _status, entity) in
            (&mut panel, &status, &entities).join() {
            // clear
            panel.widgets = vec![];
            /* NAME */ {
                let name = nameds.get(entity);
                let name = Named::name_or_noname(name);
                let name = format!("Name: {}", name);
                panel.widgets.push(Widget::label(&name));
            } /* HEALTH */ {
                let health = healths.get(entity);
                if let Some(health) = health {
                    let health = format!(
                        "HP: {}/{}",
                        health.current,
                        health.max,
                        );
                    panel.widgets.push(Widget::label(&health));
                }
            }
        }
    }
}

pub struct InteractiveUISystem;
impl<'a> System<'a> for InteractiveUISystem {
    type SystemData = (
        ReadStorage<'a, Panel>,
        ReadStorage<'a, InteractiveUI>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (panels, interactives) = data;

        for (_panel, _interactive) in (&panels, &interactives).join() {
        }
    }
}
