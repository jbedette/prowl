use specs::{Entities, Join, System, ReadStorage, WriteStorage, Write};

use crate::components::{
    Named,
    pending_actions::{Action, PendingActions},
    Position,
    TileMap,
    // markers::{
        // MoveableEntity,
    // }
};
use crate::resources::console::{
    Console,
    Log,
    LogLevel
};

use crate::shared::Vector2;

pub struct ExecuteActionSystem;

impl<'a> System<'a> for ExecuteActionSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, PendingActions>,
        ReadStorage<'a, Named>,
        WriteStorage<'a, TileMap>,
        // ReadStorage<'a, MoveableEntity>,
        Write<'a, Console>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
             mut positions,
             mut pending_actionses,
             names,
             mut tilemaps,
             // moveable_entities,
             mut console,
             entities
        ) = data;

        for (pending_actions, entity) in (&mut pending_actionses, &entities).join() {
            // for action in &pending_actions.actions {
            // TODO wut.. yuck..
            while let Some(action) = pending_actions.actions.pop() {
                let mut action_string = String::from("");
                let took_action = match action {
                    // pending move action
                    Action::Move { delta } => {
                        let position = positions.get(entity).unwrap();
                        let x = position.value.x + delta.0;
                        let y = position.value.y + delta.1;
                        let new_pos = Vector2::new(x, y);
                        let mut move_allowed = true;
                        for tilemap in (&mut tilemaps).join() {
                            if !tilemap.passable_at(new_pos) {
                                move_allowed = false;
                            } else {
                                tilemap.add_to_dynamic(new_pos);
                            }
                        }
                        if move_allowed {
                            let mut position = positions.get_mut(entity).unwrap();
                            position.value = new_pos;
                            action_string = format!("move to {}", position.value);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                if !took_action { break; }
                // Log into console.
                let name = names.get(entity);
                let name_string = Named::name_or_noname(name);
                // println!("{}: {}", name_string, action_string);
                (*console).log(Log::new(
                    LogLevel::Debug,
                    &format!("{}: {}", name_string, action_string),
                ));
            }
        }

        for tilemap in (&mut tilemaps).join() {
            tilemap.clear_dynamic();
            for position in (&positions).join() {
                tilemap.add_to_dynamic(position.value);
            }
        }

    }
}
