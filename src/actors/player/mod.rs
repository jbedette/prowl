/// Builds main character's ship
use crate::components::{
    markers::Ship,
    pending_actions::PendingActions,
    CharRenderer, Health, Money, Named, Position, Weapon, Player,
    game_resources::{
        Wood,
        Metal,
        Food,
        Water,
        GameResource,
        // GameResourceType,
    }
};
// use crate::file_io::read_file;
use crate::shared::{Vector2, random::random_range};
use specs::prelude::*;
use tcod::colors::*;
use crate::ui::{markers::StatusUI, Panel};

use crate::actors::Island;

pub struct PlayerSetupSystem;

#[allow(unused_must_use)]
impl<'a> System<'a> for PlayerSetupSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Named>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Weapon>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, CharRenderer>,
        WriteStorage<'a, PendingActions>,
        WriteStorage<'a, Ship>,
        WriteStorage<'a, Panel>,
        WriteStorage<'a, StatusUI>,
        WriteStorage<'a, GameResource<Water>>,
        WriteStorage<'a, GameResource<Food>>,
        WriteStorage<'a, GameResource<Metal>>,
        WriteStorage<'a, GameResource<Wood>>,
        ReadStorage<'a, Island>,
        Entities<'a>,
        );

    fn run(&mut self, data: Self::SystemData) {
        // use specs::Join;
        let (
            mut players,
            mut names,
            mut healths,
            mut weapons,
            mut moneys,
            mut positions,
            mut renderers,
            mut pending_actionses,
            mut ships,
            mut panels,
            mut statusuis,
            mut waters,
            mut foods,
            mut metals,
            mut woods,
            islands,
            entities,
            ) = data;
        let ship = entities.create();
        let name = Named::new("Imona Bote");
        let health = Health::new(100, 100);
        let weapon = Weapon::new(random_range(1, 10) as u64);
        let money = Money::new(random_range(1, 10) as u64);
        // position - find a large island and spawn from random spot
        let islands: Vec<&Island> = islands.join()
            .filter(|island| island.tile_positions.len() > 200)
            .collect();
        let island = islands[random_range(0, islands.len())];
        let position = island.coast_tile_positions[
            random_range(0, island.coast_tile_positions.len())];
        let position = Position::new(position);
        // renderer
        let renderer = CharRenderer::new(
            '@',
            Color::new(0x00, 0x95, 0xff),
        );
        names.insert(ship, name);
        healths.insert(ship, health);
        weapons.insert(ship, weapon);
        moneys.insert(ship, money);
        positions.insert(ship, position);
        renderers.insert(ship, renderer);
        players.insert(ship, Player::default());
        statusuis.insert(ship, StatusUI::default());
        // Resources
        let mut water = GameResource::<Water>::new(200);
        waters.insert(ship, water);
        let mut food = GameResource::<Food>::new(200);
        foods.insert(ship, food);
        let mut wood = GameResource::<Wood>::new(200);
        wood.set_count(20);
        woods.insert(ship, wood);
        metals.insert(ship, GameResource::<Metal>::new(10));
        // Info Panel
        panels.insert(ship, Panel::new(
                        "Stats",
                        Vector2::new(50, 1),
                        Vector2::new(29, 10),
                        CharRenderer::ui_body(),
                        CharRenderer::ui_border(),
                        0
                ));

        ships.insert(ship, Ship::default());
        pending_actionses.insert(ship, PendingActions::default());
    }
}
