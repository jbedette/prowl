use specs::{
    System,
    WriteStorage,
    Join,
    Entities,
};

use crate::components::{
    Position,
    pending_actions::{
        PendingActions,
        Action,
    }
};


pub struct ExecuteActionSystem;

impl<'a> System<'a> for ExecuteActionSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, PendingActions>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut positions,
            mut pending_actionses,
            entities,
        ) = data;

        for (pending_actions, entity) in (&mut pending_actionses, &entities).join() {
            // for action in &pending_actions.actions {
            // TODO wut.. yuck..
            while let Some(Some(action)) = Some(pending_actions.actions.pop()) {
                match action {
                    Action::Move { delta } => {
                        let mut position = positions.get_mut(entity).unwrap();
                        position.x += delta.0;
                        position.y += delta.1;
                    },
                    _ => ()
                }
            }
        }
    }
}
