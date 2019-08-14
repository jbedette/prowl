use specs::prelude::*;

// use crate::ui::Panel;
// use crate::components::CharRenderer;
use crate::components::{
    game_resources::{Food, GameResource, Metal, Water, Wood},
    Active, CharRenderer, Named, Player, Ship,
};
use crate::event_channel::{EventChannel, InteractionEvent};
// use crate::console::{
// use crate::console::resource::{Console, Log, LogLevel};
use crate::resources::game_data::{GameData, StateChangeRequest::WaitForUI};
use crate::shared::Vector2;
use crate::ui::{markers::InteractiveUI, panel::Widget, Panel};

pub struct InteractionSystem;

#[allow(unused_must_use)]
impl<'a> System<'a> for InteractionSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Ship>,
        WriteStorage<'a, Panel>,
        WriteStorage<'a, InteractiveUI>,
        ReadStorage<'a, GameResource<Food>>,
        ReadStorage<'a, GameResource<Water>>,
        ReadStorage<'a, GameResource<Metal>>,
        ReadStorage<'a, GameResource<Wood>>,
        WriteStorage<'a, Active>,
        Write<'a, GameData>,
        Write<'a, EventChannel<InteractionEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            players,
            ships,
            mut panels,
            mut interactive_uis,
            foods,
            waters,
            metals,
            woods,
            // mut console,
            mut actives,
            mut game_data,
            mut event_channel,
            entities,
        ) = data;

        while let Some(event) = event_channel.events.pop() {
            let parties = (event.entities[0], event.entities[1]);
            let one = names.get(parties.0);
            let two = names.get(parties.1);
            let one = Named::name_or_noname(one);
            let two = Named::name_or_noname(two);
            if players.get(parties.0).is_some() || players.get(parties.1).is_some() {
                let window = entities.create();
                let mut panel = Panel::new(
                    "[X] to close",
                    Vector2::new(5, 5),
                    Vector2::new(20, 20),
                    CharRenderer::ui_body(),
                    CharRenderer::ui_border(),
                    2, // it's always gonna be two, yes magic numbers are bad
                );
                if ships.get(parties.0).is_some() && ships.get(parties.1).is_some() {
                    panel.widgets.push(Widget::text_box(&format!(
                        "The {} collided with The {}!",
                        one, two
                    )));
                } else {
                    actives.get_mut(parties.0).unwrap().yes = true;
                    actives.get_mut(parties.1).unwrap().yes = true;
                    event_channel.events.push(InteractionEvent {
                        entities: vec![parties.0, parties.1],
                        menu_code: 1,
                    });
                    let mut res = Vec::new();
                    for (food, water, wood, metal, entity) in
                        (&foods, &waters, &woods, &metals, &entities).join()
                    {
                        res.push(format!(
                            "\n{}: {}{}",
                            food.get_name(),
                            food.get_count(),
                            format!(
                                "\n{}: {}{}",
                                water.get_name(),
                                water.get_count(),
                                format!(
                                    "\n{}: {}{}",
                                    wood.get_name(),
                                    wood.get_count(),
                                    format!("\n{}: {}", metal.get_name(), metal.get_count(),)
                                )
                            )
                        ));
                    }
                    panel.widgets.push(Widget::text_box(&format!(
                        "{} has docked at the island of {}\n{}\n\n1)Buy\n2)Sell",
                        one, two, res[0]
                    )))
                }

                panels.insert(window, panel);
                interactive_uis.insert(window, InteractiveUI::default());
                game_data.state_change_request = Some(WaitForUI);
                break;
            }
        }
    }
}
