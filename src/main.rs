use specs::{
    prelude::*,
    Builder,
    World
};
mod console;
mod components;
use components::{
    TileMap,
};

mod systems;
mod renderer;

mod resources;
use resources::{
    game_data::GameData,
};

mod shared;
use shared::{
    Vector2,
};

mod ui;

mod entity_builder;
use entity_builder::{
    ship::make_ship,
};

mod dispatcher_builder;

pub const MAP_SIZE: i32 = 500;

fn main() {
    // create an ECS "world"
    let mut world = World::new();
    resources::add_all(&mut world);
    // Dispatchers determine the order systems run in.
    // There are 3 states right now:
    // Setup: Runs only once at the beginning.
    let mut setup = dispatcher_builder::setup_dispatcher();
    // Turn: Calculates the logic of a turn happening.
    let turn = dispatcher_builder::turn_dispatcher();
    // Input: Loops at the end of every turn, allowing user input and UI.
    let input = dispatcher_builder::input_dispatcher();
    // Register all the components used (setup isn't working correctly?)
    components::register(&mut world);
    ui::register(&mut world);
     // player
    make_ship(&mut world, true);
    // populate gameworld
    for _ in 0..2000 { make_ship(&mut world, false); }
    // build a map (dumb af)
    let mut map = TileMap::new(Vector2::new(MAP_SIZE, MAP_SIZE));
    map.generate();
    // make ui windows
    ui::init::init(&mut world);
    world.create_entity()
        .with(map)
        .build();
    // run setup state (only does an initial render for now)
    setup.dispatch(&world);
    // start game loop
    run_game(world, input, turn);
}

/// Main game loop.
fn run_game(
    mut world: World,
    mut input: Dispatcher,
    mut turn: Dispatcher)
{
    loop {
        // compute a turn
        turn.dispatch(&world);
        // clear removed entities
        world.maintain();
        // input loop
        loop {
            // dispatch input system
            input.dispatch(&world);
            // consider state change
            {
                // open GameData resource
                let game_data = &mut world.write_resource::<GameData>();
                use resources::game_data::StateChangeRequest::*;
                let state_change_request = game_data.state_change_request;
                game_data.state_change_request = None;
                // if state change requested, make it happen here.
                // NOTE currently states are simple. not sure if we'll need more
                // and might need to refactor this into a big "match" or maybe
                // a primitive state machine?
                match state_change_request {
                    // trigger next turn by breaking inner loop
                    Some(NextTurn) => {
                        game_data.current_turn += 1;
                        break;
                    }
                    // doesn't exist yet
                    Some(ResetMenu) => (),
                    // quit the game by returning
                    Some(QuitGame) => return,
                    _ => (),
                }
            }
        }
    }
}
