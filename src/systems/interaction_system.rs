use specs::prelude::*;

// use crate::ui::Panel;
// use crate::components::CharRenderer;
use crate::components::{
    Named,
    CharRenderer,
};
use crate::event_channel::{
    EventChannel,
    InteractionEvent,
    // Event,
};
// use crate::console::{
/*
use crate::console::resource::{
    Log,
    LogLevel,
    Console,
};
*/
use crate::components::Player;
use crate::ui::{
    panel::Widget,
    Panel,
    markers::InteractiveUI,
};
use crate::shared::Vector2;
use crate::resources::game_data::{
    GameData,
    StateChangeRequest::WaitForUI,
};

pub struct InteractionSystem;

#[allow(unused_must_use)]
impl<'a> System<'a> for InteractionSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Panel>,
        WriteStorage<'a, InteractiveUI>,
        // Write<'a, Console>,
        Write<'a, GameData>,
        Write<'a, EventChannel<InteractionEvent>>,
        Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            names,
            players,
            mut panels,
            mut interactive_uis,
            // mut console,
            mut game_data,
            mut events,
            entities,
            ) = data;

        while let Some(event) = events.pop() {
            let one = names.get(event.entities[0]);
            let two = names.get(event.entities[1]);
            let one = Named::name_or_noname(one);
            let two = Named::name_or_noname(two);
            if players.get(event.entities[0]).is_some() ||
                    players.get(event.entities[1]).is_some() {
                        /*
                console.log(Log::new(
                        LogLevel::Game,
                        format!("{} has collided with {}", one, two),
                        ));
                        */
                let window = entities.create();
                let mut panel = Panel::new(
                            "[ESC] to close",
                            Vector2::new(5,5),
                            Vector2::new(20,20),
                            CharRenderer::ui_body(),
                            CharRenderer::ui_border(),
                            );
                panel.widgets.push(
                    Widget::text_box(&format!("{} has collided with {}", one, two))
                    );

                panels.insert(window, panel);
                interactive_uis.insert(window, InteractiveUI::default());
                game_data.state_change_request = Some(WaitForUI);
            }
        }
    }
}
