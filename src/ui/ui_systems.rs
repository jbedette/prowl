use crate::components::{
    game_resources::{Food, GameResource, Metal, Water, Wood},
    Active, CharRenderer, Health, Money, Named, Player,
};
use crate::event_channel::{EventChannel, InteractionEvent};
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
        ReadStorage<'a, Money>,
        WriteStorage<'a, GameResource<Water>>,
        WriteStorage<'a, GameResource<Food>>,
        WriteStorage<'a, GameResource<Metal>>,
        WriteStorage<'a, GameResource<Wood>>,
        Read<'a, GameData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut panel,
            status,
            nameds,
            healths,
            moneys,
            waters,
            foods,
            metals,
            woods,
            _game_data,
            entities,
        ) = data;
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
            {
                let money = moneys.get(entity);
                if let Some(money) = money {
                    let money = format!("Gold: {}", money.current);

                    panel.widgets.push(Widget::label(&money));
                }
            }
            // resource section
            // oof gotta get this in a loop or something
            {
                let food = foods.get(entity);
                if let Some(food) = food {
                    let food = format! {"{}: {}", food.get_name(),food.get_count(),};
                    panel.widgets.push(Widget::label(&food));
                }
                let water = waters.get(entity);
                if let Some(water) = water {
                    let water = format! {"{}: {}", water.get_name(),water.get_count(),};
                    panel.widgets.push(Widget::label(&water));
                }
                let wood = woods.get(entity);
                if let Some(wood) = wood {
                    let wood = format! {"{}: {}", wood.get_name(),wood.get_count(),};
                    panel.widgets.push(Widget::label(&wood));
                }
                let metal = metals.get(entity);
                if let Some(metal) = metal {
                    let metal = format! {"{}: {}", metal.get_name(),metal.get_count(),};
                    panel.widgets.push(Widget::label(&metal));
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
        ReadStorage<'a, Named>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, GameResource<Water>>,
        WriteStorage<'a, GameResource<Food>>,
        WriteStorage<'a, GameResource<Metal>>,
        WriteStorage<'a, GameResource<Wood>>,
        WriteStorage<'a, Active>,
        WriteStorage<'a, Panel>,
        WriteStorage<'a, InteractiveUI>,
        Write<'a, EventChannel<InteractionEvent>>,
        Write<'a, Console>,
        Write<'a, GameData>,
        Write<'a, Window>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            _player,
            mut moneys,
            mut waters,
            mut foods,
            mut metals,
            mut woods,
            mut actives,
            mut panels,
            mut interactive_uis,
            mut event_channel,
            mut console,
            mut game_data,
            mut window,
            entities,
        ) = data;

        let mut ui_opts = 0;
        let count = panels.join().count() as i32;
        //println!("INTERACTIVE UI ACTIVE");
        let key = tcod_input::get(&mut window.root);
        use tcod_input::InputCode;
        use InputCode::*;
        game_data.state_change_request = Some(StateChangeRequest::WaitForUI);
        match key {
            One => {
                ui_opts = 1;
            }
            Two => {
                ui_opts = 2;
            }
            Three => {
                ui_opts = 3;
            }
            Four => {
                ui_opts = 4;
            }
            Back => {
                ui_opts = 5;
            }
            // Console
            ConsoleSrollUp => console.scroll(-1),
            ConsoleSrollDown => console.scroll(1),
            _ => {
                ui_opts = 5;
            }
        }
        if ui_opts != 5 {
            while let Some(event) = event_channel.events.pop() {
                let parties = (event.entities[0], event.entities[1]);
                let mut flip = 1;
                let code = event.menu_code;
                match code {
                    1 => {
                        let mut buy_sell = "Buy";
                        let mut first_layer_choice = 2;
                        let window = entities.create();
                        let mut panel = Panel::new(
                            "[X] to close",
                            Vector2::new(5 + (count * 3), 5 + ((count - 1) * 3)),
                            Vector2::new(20, 20),
                            CharRenderer::ui_body(),
                            CharRenderer::ui_border(),
                            count, //todo:replace with smart id system
                        );
                        if ui_opts == 2 {
                            first_layer_choice = 3;
                            buy_sell = "Sell";
                        }
                        panel
                            .widgets
                            .push(Widget::text_box(&format! {"{} for 25 Gold:\n\n1)Food\n2)Water\n3)Wood\n4)Metal",buy_sell}));
                        panels.insert(window, panel);
                        interactive_uis.insert(window, InteractiveUI::default());
                        event_channel.events.push(InteractionEvent {
                            entities: vec![parties.0, parties.1],
                            menu_code: first_layer_choice,
                        });
                        break;
                    }
                    //buy
                    3 => {
                        flip *= -1;
                    }
                    _ => (),
                };
                for (food, water, wood, metal, entity, panel) in (
                    &mut foods,
                    &mut waters,
                    &mut woods,
                    &mut metals,
                    &entities,
                    &mut panels,
                )
                    .join()
                {
                    let active = actives.get(entity).unwrap().yes;
                    let is_player = _player.get(entity).is_some();
                    let money = moneys.get_mut(entity).unwrap();
                    match ui_opts {
                        1 => {
                            if active && is_player {
                                food.transaction(25 * flip);
                                money.transaction((-25 * flip) as i64);
                            } else if active {
                                food.transaction(-25 * flip);
                            }
                        }
                        2 => {
                            if active && is_player {
                                water.transaction(25 * flip);
                                money.transaction((-25 * flip) as i64);
                            } else if active {
                                water.transaction(-25 * flip);
                            }
                        }
                        3 => {
                            if active && is_player {
                                wood.transaction(25 * flip);
                                money.transaction((-25 * flip) as i64);
                            } else if active {
                                wood.transaction(-25 * flip);
                            }
                        }
                        4 => {
                            if active && is_player {
                                metal.transaction(25 * flip);
                                money.transaction((-25 * flip) as i64);
                            } else if active {
                                metal.transaction(-25 * flip);
                            }
                        }
                        _ => (),
                    };
                    ui_opts = 5;
                }
                /*
                event_channel.events.push(InteractionEvent {
                    entities: vec![parties.0, parties.1],
                    menu_code: 0,
                });
                */
                break;
            }
        }
        if ui_opts == 5 {
            for (_panel, _interactive, entity) in
                (&panels, &interactive_uis, &entities).join().last()
            {
                // println!("here{} ", count);
                if count - 3 <= 0 {
                    for active in (&mut actives).join() {
                        active.yes = false;
                    }
                    while let Some(event) = event_channel.events.pop() {}
                    game_data.state_change_request = Option::None;
                }
                let mut entities_to_remove = Vec::new();
                entities_to_remove.push(entity);
                for entity in entities_to_remove {
                    entities.delete(entity);
                }
            }
        }
    }
}
