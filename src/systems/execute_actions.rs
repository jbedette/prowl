// When a user input does something, this manages what it does

use specs::{Entities, Join, System, ReadStorage, WriteStorage, Write};

use crate::components::{
    Named,
    pending_actions::{Action, PendingActions},
    Position,
    TileMap,
    Money,
    Player
    
};
use crate::console::resource::{
    // Log,
    // LogLevel,
    Console
};
use crate::event_channel::{
    EventChannel,
    InteractionEvent,
};

use crate::shared::Vector2;

pub struct ExecuteActionSystem;

#[allow(unused)]
impl<'a> System<'a> for ExecuteActionSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, PendingActions>,
        ReadStorage<'a, Named>,
        WriteStorage<'a, TileMap>,
        Write<'a, EventChannel<InteractionEvent>>,
        Write<'a, Console>,
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Money>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
             mut positions,
             mut pending_actionses,
             _names,
             mut tilemaps,
             mut interaction_events,
             mut _console,
             entities,
             player,
             moneys

        ) = data;

        for (pending_actions, entity) in (&mut pending_actionses, &entities).join() {
            // for action in &pending_actions.actions {
            // TODO wut.. yuck..
            while let Some(action) = pending_actions.actions.pop() {
                // let mut action_string = String::from("");
                let took_action = match action {
                    // pending move action
                    Action::Move { delta } => {
                        let position = positions.get(entity).unwrap();
                        let x = position.value.x + delta.x;
                        let y = position.value.y + delta.y;
                        let new_pos = Vector2::new(x, y);
                        let mut move_allowed = true;
                        // for each map (only one right now)
                        for tilemap in (&mut tilemaps).join() {
                            // if position is passable or contains an entity
                            let tile_data = tilemap.passable_at(new_pos);
                            if let Some(entity1) = tile_data.1 {
                                // TODO interaction trigger
                                interaction_events.events.push(InteractionEvent {
                                    entities: vec![entity, entity1],
                                    menu_code: 0,
                                });
                                move_allowed = false;
                            } else {
                                if !tile_data.0 {
                                    move_allowed = false;
                                } else {
                                    tilemap.remove_from_dynamic(position.value);
                                    tilemap.add_to_dynamic(new_pos, entity);
                                }
                            }
                        }
                        if move_allowed {
                            let mut position = positions.get_mut(entity).unwrap();
                            position.value = new_pos;
                            // action_string = format!("mov {}", position.value);
                            true
                        } else {
                            false
                        }
                    }
                    /*
                    Action::Buy => {
                        for (_player, _money, _entity) in (&mut player, &mut moneys, &entities).join(){

                        }
                        true
                    }
                    */
                    _ => false,
                };
                if !took_action { break; }
                // Log into console.
                // let name = names.get(entity);
                // let name_string = Named::name_or_noname(name);
                // println!("{}: {}", name_string, action_string);
                // (*console).log(Log::new(
                //     LogLevel::Debug,
                //     &format!("{}: {}", name_string, action_string),
                // ));
            }
        }
    }
}
