use specs::{Read, Join, System, WriteStorage};

use crate::components::{
    ai::{Goal, AI},
    pending_actions::{Action, PendingActions},
};

use crate::resources::{
    game_data::GameData,
};

// use rand::prelude::*;
use rand;

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, AI>,
        WriteStorage<'a, PendingActions>,
        Read<'a, GameData>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut ais,
            mut pending_actionses,
            game_data,
            ) = data;

        for (ai, pending_actions) in (&mut ais, &mut pending_actionses).join() {
            if ai.last_moved < game_data.current_turn {
                ai.last_moved = game_data.current_turn;
                match ai.goal {
                    // TODO make better lmao
                    Some(Goal::MoveRandom) => {
                        let mut delta = (0, 0);
                        if rand::random() {
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
                        }
                        pending_actions.actions.push(Action::Move { delta });
                    }
                    _ => (),
                };
            }
        }
    }
}
