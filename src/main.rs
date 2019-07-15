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
    Rivals,
    Health,
    Weapon,
    Money,
    Position,
    CharRenderer,
    Player,
    pending_actions::PendingActions,
    AI,
    ai,
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

    // initialize systems in the world
    // format:
    // system, "string id", dependencies (systems that must run before this one)
    let mut loader = specs::DispatcherBuilder::new()
        .with_thread_local(RenderingSystem)
        .build();
    let dispatcher = specs::DispatcherBuilder::new()
        .with(RivalSystem, "rivals", &[])
        .with(AISystem, "ai", &[])
        .with(UserInputSystem, "input", &["ai"])
        .with(ExecuteActionSystem, "execute_actions", &["ai", "input"])
        .with(DeathSystem, "deaths", &["execute_actions"])
        // rendering must be on local thread (i think?)
        .with_thread_local(RenderingSystem)
        .build();
    // dispatcher.setup(&mut world.res); // TODO why doesn't this work?

    // Register all the components (setup isn't working correctly?)
    components::register(&mut world);


    // create some entities in the world
    // TODO make generator functions:
    // create_human() ?
    // Creator object ?
    // how to make random ?
    // what determines parameters ?

    world.create_entity()
        .with(Named::new("Matt"))
        .with(Rivals::new())
        .with(Health::new(100, 80))
        .with(Weapon::new(1))
        .with(Money::new(4))
        .with(Position::new(4, 8))
        .with(CharRenderer::new('@', Color::new(0x00, 0x95, 0xff)))
        .with(PendingActions::default())
        .with(Player::default())
        .build();

    for _ in 0..30 {
        make_person(&mut world, false);
    }
    /*
    world.create_entity()
        .with(Named::new("Lysa"))
        .with(Rivals::new())
        .with(Health::new(9, 12))
        .with(Weapon::new(2))
        .with(Money::new(4))
        .with(Position::new(4, 10))
        .with(CharRenderer::new('L', Color::new(0x20, 0x76, 0xbb)))
        .with(AI::with_goal(ai::Goal::MoveRandom))
        .with(PendingActions::default())
        .build();

    world.create_entity()
        .with(Named::new("Dumbo"))
        .with(Rivals::new())
        .with(Health::new(10, 10))
        .with(Weapon::new(1))
        .with(Money::new(4))
        .with(Position::new(2, 10))
        .with(CharRenderer::new('D', Color::new(0xff, 0x00, 0x95)))
        .with(AI::with_goal(ai::Goal::MoveRandom))
        .with(PendingActions::default())
        .build();
        */

    loader.dispatch(&world);
    run(world, dispatcher);
}

/// Main game loop.
fn run(mut world: World, mut dispatcher: Dispatcher) {
    let mut turn = 0;
    loop {
        log_turn(&mut world, turn);
        dispatcher.dispatch(&world);
        world.maintain();
        // check if user requested quit
        // let quit = world.read_resource::<Quit>();
        let game_data = &mut world.write_resource::<GameData>();
        // let game_data = (*game_data);
        use resources::game_data::StateChangeRequest::*;
        match game_data.switch_state {
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
            .with(CharRenderer::new(*renderer.0, renderer.1))
            .with(PendingActions::default())
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

