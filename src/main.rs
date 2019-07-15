// use std::{
    // thread,
    // time
// };
use rand;
use rand::Rng;

use specs::{
    World,
    Builder,
    prelude::*
};
use tcod::{
    colors::*,
};

mod systems;
use systems::{
    RivalSystem,
    DeathSystem,
    RenderingSystem,
    UserInputSystem,
    AISystem,
    ExecuteActionSystem,
};

mod components;
use components::{
    Named,
    // Rivals,
    Health,
    Weapon,
    Money,
    Position,
    CharRenderer,
    Player,
    pending_actions::PendingActions,
    AI,
    ai,
    // TileMap,
};

mod resources;
use resources::{
    console::{
        Console,
        Log,
        LogLevel,
    },
    game_data::GameData,
};

mod shared;

fn main() {
    // create an ECS "world"
    let mut world = World::new();
    resources::add_all(&mut world);

    // format:
    // let loader = build_load_dispatcher;
    // let dispatcher = build_main_loop_dispatcher();
    let mut loader = specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build();
    let dispatcher = specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with(RivalSystem, "rivals", &[])
        .with(AISystem, "ai", &[])
        .with(UserInputSystem, "input", &["ai"])
        .with(ExecuteActionSystem, "execute_actions", &["ai", "input"])
        .with(DeathSystem, "deaths", &["execute_actions"])
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build();
    // dispatcher.setup(&mut world.res); // TODO why doesn't this work?

    // Register all the components (setup isn't working correctly?)
    components::register(&mut world);
    make_person(&mut world, true);
    for _ in 0..30 { make_person(&mut world, false); }
    loader.dispatch(&world);
    run(world, dispatcher);
}

/*
/// initialize systems in the loader state
fn build_load_dispatcher() -> Dispatcher {
    specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        // rendering must be on local thread
        .with_thread_local(RenderingSystem)
        .build()
}

/// initialize systems in the main loop state
fn build_main_loop_dispatcher() -> Dispatcher {
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
*/

/// Main game loop.
fn run(mut world: World, mut dispatcher: Dispatcher) {
    let mut turn = 0;
    loop {
        log_turn(&mut world, turn);
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
fn log_turn(world: &mut World, i: i32) {
    let mut console = world.write_resource::<Console>();
        console.log(Log {
            level: LogLevel::Debug,
            message: format!("Simulation Step {}", i),
        });
}

// TODO what determines parameters ?
fn make_person(world: &mut World, is_player: bool) {
    let names = [
        "Mark",
        "Dumbo",
        "Kyle",
        "Jumbo",
        "Jarvis",
        "Cool Man",
        "Smarto",
    ];
    let name = names[random_range(0, names.len())];
    let health = random_range(80, 200) as i64;
    let weapon = random_range(1, 10) as u64;
    let money = random_range(30, 300) as u64;
    let position = (random_range(0, 50) as i32,
                    random_range(0, 50) as i32);
    let renderer = (
        &name.chars().next().unwrap().clone(), 
        Color::new(
            random_range(0, 255) as u8,
            random_range(0, 255) as u8,
            random_range(0, 255) as u8,
            ));
    if is_player {
        world.create_entity()
            .with(Named::new(name))
            .with(Health::new(health, health))
            .with(Weapon::new(weapon))
            .with(Money::new(money))
            .with(Position::new(position.0, position.1))
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

