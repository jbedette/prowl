use specs::prelude::*;
use crate::components::{
    AI,
    Health, 
    TileMap, 
    Money, 
    Named, 
    Position, 
    CharRenderer, 
    Weapon, 
    Player, 
    Ship, 
    Active, 
    pending_actions, 
    game_resources,
    };

pub fn register(world: &mut World) {
    world.register::<Named>();
    world.register::<Health>();
    world.register::<Money>();
    world.register::<Weapon>();
    world.register::<Position>();
    world.register::<CharRenderer>();
    world.register::<Player>();
    world.register::<Ship>();
    world.register::<pending_actions::PendingActions>();
    world.register::<AI>();
    world.register::<TileMap>();
    world.register::<Active>();
    game_resources::register_all(world);
}