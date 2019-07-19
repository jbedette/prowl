use specs::{Entities, Join, System, ReadStorage, WriteStorage, Write};

use crate::components::{
    Named,
    pending_actions::{Action, PendingActions},
    Position,
    TileMap,
    markers::{
        MoveableEntity,
    }
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
        ReadStorage<'a, TileMap>,
        ReadStorage<'a, MoveableEntity>,
        Write<'a, Console>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
             mut positions,
             mut pending_actionses,
             names,
             tilemaps,
             moveable_entities,
             mut console,
             entities
        ) = data;
        for (pending_actions, entity) in (&mut pending_actionses, &entities).join() {
            // for action in &pending_actions.actions {
            // TODO wut.. yuck..
            while let Some(Some(action)) = Some(pending_actions.actions.pop()) {
                let mut action_string = String::from("");
                let took_action = match action {
                    // pending move action
                    Action::Move { delta } => {
                        // let mut position = positions.get_mut(entity).unwrap();
                        let position = positions.get(entity).unwrap();
                        let x = position.value.x + delta.0;
                        let y = position.value.y + delta.1;
                        let new_pos = Vector2::new(x, y);
                        let mut move_allowed = true;
                        for tilemap in (tilemaps).join() {
                            if !tilemap.passable_at(new_pos) {
                                move_allowed = false;
                            }
                        }
                        // TODO this is super inefficient!
                        // it's looping through all objects that move,
                        // no matter how far away. not great...
                        for (_mover, _position1) in (&moveable_entities, &positions)
                            .join()
                            .filter(|(_, position1)| {
                                // new_pos.x == position1.value.x &&
                                // new_pos.y == position1.value.y
                                new_pos == position1.value
                            })
                        {
                            // if new_pos == position1.value {
                            move_allowed = false;
                            action_string = format!("move to {} BLOCKED", new_pos);
                            // }
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
    }
}
