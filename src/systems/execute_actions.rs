use specs::{Entities, Join, System, ReadStorage, WriteStorage, Write};

use crate::components::{
    Named,
    pending_actions::{Action, PendingActions},
    Position,
    TileMap
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
        Write<'a, Console>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut pending_actionses, names, tilemaps, mut console, entities) = data;

        for (pending_actions, entity) in (&mut pending_actionses, &entities).join() {
            // for action in &pending_actions.actions {
            // TODO wut.. yuck..
            while let Some(Some(action)) = Some(pending_actions.actions.pop()) {
                let mut action_string = String::from("");
                let took_action = match action {
                    // pending move action
                    Action::Move { delta } => {
                        let mut position = positions.get_mut(entity).unwrap();
                        let x = position.x + delta.0;
                        let y = position.y + delta.1;
                        let new_pos = Vector2::new(x, y);
                        let mut move_allowed = true;
                        for tilemap in (tilemaps).join() {
                            if !tilemap.passable_at(new_pos) {
                                move_allowed = false;
                            }
                        }
                        if move_allowed {
                            position.x = new_pos.x;
                            position.y = new_pos.y;
                            action_string = format!("move to {},{}",
                                                    position.x, position.y);
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
                let name_string = match name {
                    Some(name) => name.value.to_owned(),
                    None => String::from("UNNAMED ENTITY"),
                };
                // let _result = entities.delete(entity);
                (*console).log(Log::new(
                    LogLevel::Debug,
                    &format!("{}: {}", name_string, action_string),
                ));
            }
        }
    }
}
