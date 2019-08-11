use specs::{Join, System, WriteStorage};

use crate::components::{
    ai::{Goal, AI},
    pending_actions::{Action, PendingActions},
};

use rand;

use crate::shared::Vector2;

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, AI>,
        WriteStorage<'a, PendingActions>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut ais,
            mut pending_actionses,
            ) = data;

        for (ai, pending_actions) in (&mut ais, &mut pending_actionses).join() {
            match ai.goal {
                // Literally move in an entirely random direction
                Some(Goal::MoveRandom) => {
                    let delta;
                    if rand::random() {
                        if rand::random() {
                            if rand::random() { delta = Vector2::north(); }
                            else { delta = Vector2::south(); }
                        } else {
                            if rand::random() { delta = Vector2::east(); }
                            else { delta = Vector2::west(); }
                        }
                        pending_actions.actions.push(Action::Move { delta });
                    }
                }
                _ => (),
            };
        }
    }
}
