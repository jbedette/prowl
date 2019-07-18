use specs::{Join, System, WriteStorage};

use crate::components::{
    ai::{Goal, AI},
    pending_actions::{Action, PendingActions},
};

// use rand::prelude::*;
use rand;

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, AI>,
        WriteStorage<'a, PendingActions>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ais,
            mut pending_actionses
            ) = data;

        for (ai, pending_actions) in (&ais, &mut pending_actionses).join() {
            match ai.goal {
                // TODO make better lmao
                Some(Goal::MoveRandom) => {
                    let mut delta = (0, 0);
                    if rand::random() {
                        if rand::random() {
                            delta.0 += 1;
                        } else {
                            delta.0 -= 1;
                        }
                    } else if rand::random() {
                        if rand::random() {
                            delta.1 += 1;
                        } else {
                            delta.1 -= 1;
                        }
                    }
                    pending_actions.actions.push(Action::Move { delta });
                }
                _ => (),
            };
        }
    }
}
