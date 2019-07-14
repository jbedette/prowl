use specs::{
    System,
    WriteStorage,
    Join,
};

use crate::components::{
    ai::{
        AI,
        Goal
    },
    Position,
};

// use rand::prelude::*;
use rand;

pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, AI>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ais,
            mut positions
        ) = data;

        for (ai, position) in (&ais, &mut positions).join() {
            match ai.goal {
                // TODO make better lmao
                Some(Goal::MoveRandom) => {
                    if rand::random() {
                        if rand::random() {
                            position.x += 1;
                        } else if rand::random() {
                            position.x -= 1;
                        }
                    } else {
                        if rand::random() {
                            position.y += 1;
                        } else if rand::random() {
                            position.y -= 1;
                        }
                    }
                },
                _ => (),
            };
        }
    }
}
