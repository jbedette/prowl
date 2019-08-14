/// Kills entities that are out of health.
use specs::prelude::*;

use crate::components::{
    Player,
    Health,
    // Named
    game_resources::{
        Food, Water,
        GameResource
    }
};

use crate::console::resource::{
    Log, LogLevel, Console
};

pub struct FoodSystem;

impl<'a> System<'a> for FoodSystem {
    type SystemData = (
        // ReadStorage<'a, Named>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, GameResource<Food>>,
        WriteStorage<'a, GameResource<Water>>,
        ReadStorage<'a, Player>,
        // WriteStorage<'a, Population>
        Entities<'a>,
        Write<'a, Console>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut healths,
            mut foods,
            mut waters,
            players,
            entities,
            mut console,
            ) = data;

        for (health, food, water, entity) in
                (&mut healths, &mut foods, &mut waters, &entities)
                .join() {
            food.adjust_count(-1);
            water.adjust_count(-1);
            if food.get_count() == 0 || water.get_count() == 0 {
                health.current -= 1;
                if players.get(entity).is_some() {
                    (*console).log(Log::new(
                        LogLevel::Debug,
                        &format!("Starving! Find more food!"),
                    ));
                }
            }
        }
    }
}
