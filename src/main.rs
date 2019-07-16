// use std::{
// thread,
// time
// };
use rand;
use rand::Rng;

use specs::{prelude::*, Builder, World};
use tcod::colors::*;

mod systems;
use systems::{
    AISystem, DeathSystem, ExecuteActionSystem, RenderingSystem, RivalSystem, UserInputSystem,
};

mod components;
use components::{
    ai,
    TileMap,
    pending_actions::PendingActions,
    CharRenderer,
    // Rivals,
    Health,
    Money,
    Named,
    Player,
    Position,
    Weapon,
    AI,
};

mod resources;
use resources::{
    console::{Console, Log, LogLevel},
    game_data::GameData,
};

mod shared;
use shared::Vector2;

const MAP_SIZE: i32 = 400;

fn main() {
    // create an ECS "world"
    let mut world = World::new();
    resources::add_all(&mut world);
    // dispatchers determine the order systems run in
    let mut setup = build_setup_dispatcher();
    let dispatcher = build_main_loop_dispatcher();
    // Register all the components used (setup isn't working correctly?)
    components::register(&mut world);
     // player
    make_person(&mut world, true);
    // populate gameworld
    for _ in 0..600 { make_person(&mut world, false); }
    // build a map (dumb af)
    let mut map = TileMap::new(Vector2::new(MAP_SIZE, MAP_SIZE));
    for _ in 0..400 {
        map.place_island(Vector2::new(
                random_range(00, 380) as i32,
                random_range(00, 380) as i32)
            );
    }
    world.create_entity()
        .with(map)
        .build();
    // run setup state (only does an initial render for now)
    setup.dispatch(&world);
    run(world, dispatcher);
}

// TODO not 'static?
/// initialize systems in the loader state
fn build_setup_dispatcher() -> Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build()
}

// TODO not 'static?
/// initialize systems in the main loop state
fn build_main_loop_dispatcher() -> Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with(RivalSystem, "rivals", &[])
        .with(AISystem, "ai", &[])
        .with(UserInputSystem, "input", &["ai"])
        .with(ExecuteActionSystem, "execute_actions", &["ai", "input"])
        .with(DeathSystem, "deaths", &["execute_actions"])
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build()
}

/// Main game loop.
fn run(mut world: World, mut dispatcher: Dispatcher) {
    let mut turn = 0;
    loop {
        log_turn(&mut world);
        dispatcher.dispatch(&world);
        world.maintain();
        // check if user requested quit
        let game_data = &mut world.write_resource::<GameData>();
        use resources::game_data::StateChangeRequest::*;
        match game_data.state_change_request {
            Some(ResetMenu) => (),
            Some(QuitGame) => break,
            _ => (),
        }
        turn += 1;
        game_data.current_turn = turn;
    }
}
// thread::sleep(time::Duration::from_secs(1));

/// Logs current turn
fn log_turn(world: &mut World) {
    let mut console = world.write_resource::<Console>();
    console.log(Log {
        level: LogLevel::Game,
        // message: format!("Simulation Step {}", i),
        message: format!("Time passes..."),
    });
}

// TODO what determines parameters ?
fn make_person(world: &mut World, is_player: bool) {
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
    let position = (random_range(0, max) as i32, random_range(0, max) as i32);
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
            .with(Position::new(30, 30))
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
            .with(Position::new(position.0, position.1))
            .with(CharRenderer::new(*renderer.0, renderer.1))
            .with(PendingActions::default())
            .with(AI::with_goal(ai::Goal::MoveRandom))
            .build();
    }
}

fn random_range(low: usize, hi: usize) -> usize {
    rand::thread_rng().gen_range(low, hi)
}
