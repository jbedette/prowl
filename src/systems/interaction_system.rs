use specs::prelude::*;

// use crate::ui::Panel;
// use crate::components::CharRenderer;
use crate::components::{CharRenderer, Named};
use crate::event_channel::{
    EventChannel,
    InteractionEvent,
    // Event,
};
// use crate::console::{
use crate::components::Player;
use crate::components::Ship;
use crate::console::resource::{Console, Log, LogLevel};
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
        // Write<'a, Console>,
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
            // mut console,
            mut game_data,
            mut events,
            entities,
        ) = data;

        while let Some(event) = events.pop() {
            let parties = (event.entities[0], event.entities[1]);
            let one = names.get(parties.0);
            let two = names.get(parties.1);
            let one = Named::name_or_noname(one);
            let two = Named::name_or_noname(two);
            if players.get(parties.0).is_some() || players.get(parties.1).is_some()
            {
                /*
                console.log(Log::new(
                        LogLevel::Game,
                        format!("{} has collided with {}", one, two),
                        ));
                        */
                let window = entities.create();
                let mut panel = Panel::new(
                    "[ESC] to close",
                    Vector2::new(5, 5),
                    Vector2::new(20, 20),
                    CharRenderer::ui_body(),
                    CharRenderer::ui_border(),
                );
                if ships.get(parties.0).is_some() && ships.get(parties.1).is_some(){
                    panel.widgets.push(Widget::text_box(&format!("{} collided with another ship, {}!", one, two)));
                }
                else{
                    panel.widgets.push(Widget::text_box(&format!("{} has docked at the island of {}", one, two)))
                }
                /*
                panel.widgets.push(Widget::text_box(&get_menu(
                    event.entities[0],
                    event.entities[1],
                )));
                */

                panels.insert(window, panel);
                interactive_uis.insert(window, InteractiveUI::default());
                game_data.state_change_request = Some(WaitForUI);
            }
        }
    }
}
/*
            //if players.get(event.entities[0]).is_some() || players.get(event.entities[1]).is_some()
fn get_menu(one:Entity, two:Entity)-> String{
    let ent_names = (names.get(one),names.get(two));
    if ships.get(one).is_some(){
    }
}
*/