use specs::{prelude::*, Builder, World};
// Console holds the console system and resource but
// does not directly handle its own UI.
mod console;

// TODO should components module still be used for some components?
mod components;
use components::TileMap;

// TODO move all systems into folders named by their usage.
mod file_io;
mod systems;

// Handles all rendering.
mod renderer;

// TODO move all resources into folders named by their usage.
mod resources;
use resources::game_data::GameData;

// Shared data types and utility functions.
mod shared;
use shared::{
    // random::random_range,
    Vector2
};

// Handles UI layout (still rendered within renderer)
mod ui;

// Contains all actor components and any systems that
// only deal with actors (?) TODO move stuff into here.
mod actors;

//use actors::ships::make_ship;

// Prepares dispatchers for later use.
mod dispatcher_builder;
// Handles user input.
mod input;
// Testing..
mod event_channel;

mod generators;

pub const MAP_SIZE: i32 = 400;

fn main() {
    // create an ECS "world"
    let mut world = World::new();
    // register all the resources in the world
    resources::register(&mut world);
    // register all the components in the world
    components::register(&mut world);
    ui::register(&mut world);
    event_channel::register(&mut world);
    actors::register(&mut world);
    // build a map (dumb af)
    let water_level = 0.75;
    let map = TileMap::new(Vector2::new(MAP_SIZE, MAP_SIZE), water_level);
    // player
    // make_player(&mut world);
    // populate gameworld
    // for _ in 0..2000 { make_ship(&mut world); }
    // make ui windows
    ui::init::init(&mut world);
    world.create_entity().with(map).build();
    // start game loop
    run_game(world);
}

/// Main game loop.
fn run_game(mut world: World) {
    // Dispatchers determine the order systems run in.
    // Setup: Runs only once at the beginning.
    let mut setup = dispatcher_builder::setup_dispatcher();
    // Turn: Calculates the logic of a turn happening.
    let mut turn = dispatcher_builder::turn_dispatcher();
    // Input: Blocks at the end of every turn, allowing user input and UI.
    let mut input = dispatcher_builder::input_dispatcher();
    // WaitForUI
    let mut ui = dispatcher_builder::ui_dispatcher();
    // This is dumb, but makes UI work
    let mut render = dispatcher_builder::render_dispatcher();
    // Register all the components used (setup isn't working correctly?)
    // run setup state (only does an initial render for now)
    setup.dispatch(&world);

    loop {
        // compute a turn
        turn.dispatch(&world);
        // add created entities,
        // clear removed entities
        world.maintain();
        // input loop
        loop {
            use resources::game_data::StateChangeRequest::*;
            let state_change_request;
            // open GameData resource
            {
                let game_data = &mut world.write_resource::<GameData>();
                state_change_request = game_data.state_change_request;
                game_data.state_change_request = None;
            }
            // dispatch input system
            // blocking UI happens before input, has own input.
            if state_change_request == Some(WaitForUI) {
                loop {
                    ui.dispatch(&world);
                    world.maintain();
                    render.dispatch(&world);
                    let game_data = &mut world.write_resource::<GameData>();
                    if game_data.state_change_request == None {
                        break;
                    }
                }
            }
            input.dispatch(&world);
            // consider state change
            {
                // if state change requested, make it happen here.
                // NOTE currently states are simple. not sure if we'll need more
                // and might need to refactor this into a big "match" or maybe
                // state machine?
                match state_change_request {
                    // trigger next turn with break
                    Some(NextTurn) => {
                        let game_data = &mut world.write_resource::<GameData>();
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
