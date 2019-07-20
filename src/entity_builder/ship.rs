use crate::components::{
    ai,
    pending_actions::PendingActions,
    CharRenderer,
    Health,
    Money,
    Named,
    Position,
    Weapon,
    AI,
    markers::{
        Player,
    }
};
use crate::shared::{
    Vector2,
    random::random_range,
};
use specs::prelude::*;
use tcod::colors::*;
// TODO maybe don't do this
use crate::MAP_SIZE;


// TODO what determines parameters ?
pub fn make_ship(world: &mut World, is_player: bool) {
    let names = [
        "Mark", "Dumbo", "Kyle", "Jumbo", "Jarvis", "Cool Man", "Smarto",
        "Dweebster", "Markov", "Callios", "Krun", "Eliza", "Thadd",
        "Frigado", "Buttface", "MacGregor", "Envin", "Kuroga", "Shiela",
        "Cicily", "Ness", "Endo", "Bendo", "Fry", "Leela", "Bender",
        "Zoidberg", "Hermes", "Amy", "Zapp", "Scruffy"
    ];
    let name = names[random_range(0, names.len())];
    let health = random_range(80, 200) as i64;
    let weapon = random_range(1, 10) as u64;
    let money = random_range(30, 300) as u64;
    let max = (MAP_SIZE - 1) as usize;
    let x = random_range(0, max) as i32;
    let y = random_range(0, max) as i32;
    let position = Vector2::new(x,y);
    let renderer = (
        &name.chars().next().unwrap().clone(),
        Color::new(
            random_range(0x88, 0xff) as u8,
            random_range(0x88, 0xff) as u8,
            random_range(0x88, 0xff) as u8,
        ),
    );
    // TODO should be able to combine these with some swapped
    // components somehow... right?
    if is_player {
        world.create_entity()
            .with(Named::new(name))
            .with(Health::new(health, health))
            .with(Weapon::new(weapon))
            .with(Money::new(money))
            .with(Position::new(Vector2::new(30, 30)))
            .with(PendingActions::default())
            // special
            .with(CharRenderer::new('@', Color::new(0x00, 0x95, 0xff)))
            // special
            .with(Player::default())
            .build();
    } else {
        world.create_entity()
            .with(Named::new(name))
            .with(Health::new(health, health))
            .with(Weapon::new(weapon))
            .with(Money::new(money))
            .with(Position::new(position))
            .with(CharRenderer::new(*renderer.0, renderer.1))
            .with(PendingActions::default())
            .with(AI::with_goal(ai::Goal::MoveRandom))
            .build();
    }
}
