use specs::{
    prelude::*,
    Builder,
    World
};
use tcod::colors::*;

mod systems;
use systems::{
    AISystem,
    DeathSystem,
    ExecuteActionSystem,
    UserInputSystem,
};

mod renderer;
use renderer::{
    rendering_system::RenderingSystem,
};

mod components;
use components::{
    CharRenderer,
    TileMap,
};

mod resources;
use resources::{
    game_data::GameData,
};

mod shared;
use shared::{
    Vector2,
    random::random_range,
};

mod ui;
use ui::panel::Panel;

mod entity_builder;
use entity_builder::{
    ship::make_ship,
};

pub const MAP_SIZE: i32 = 500;

fn main() {
    // create an ECS "world"
    let mut world = World::new();
    resources::add_all(&mut world);
    // dispatchers determine the order systems run in
    let mut setup = build_setup_dispatcher();
    let input = build_input_dispatcher();
    let turn = build_turn_dispatcher();
    // Register all the components used (setup isn't working correctly?)
    components::register(&mut world);
    ui::register(&mut world);
     // player
    make_ship(&mut world, true);
    // populate gameworld
    for _ in 0..2000 { make_ship(&mut world, false); }
    // build a map (dumb af)
    let mut map = TileMap::new(Vector2::new(MAP_SIZE, MAP_SIZE));
    for _ in 0..2000 {
        map.place_island(Vector2::new(
                random_range(0, MAP_SIZE as usize) as i32,
                random_range(0, MAP_SIZE as usize) as i32),
                Vector2::new( random_range(3,10) as i32,
                    random_range(3,10) as i32)
            );
    }
    world.create_entity()
        .with(Panel::new(
                "Panel Test",
                Vector2::new(1, 1),
                Vector2::new(14, 6),
                CharRenderer::new(' ', Color::new(12, 24, 32)),
                CharRenderer::new(' ', Color::new(45, 42, 90)),
                ))
        .build();
    world.create_entity()
        .with(Panel::new(
                "Panel Test",
                Vector2::new(44, 29),
                Vector2::new(35, 20),
                CharRenderer::new(' ', Color::new(12, 24, 32)),
                CharRenderer::new(' ', Color::new(45, 42, 90)),
                ))
        .build();

    world.create_entity()
        .with(map)
        .build();
    // run setup state (only does an initial render for now)
    setup.dispatch(&world);
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
            input.dispatch(&world);
            let game_data = &mut world.write_resource::<GameData>();
            use resources::game_data::StateChangeRequest::*;
            let state_change_request = game_data.state_change_request;
            game_data.state_change_request = None;
            match state_change_request {
                // trigger next turn
                Some(NextTurn) => {
                    game_data.current_turn += 1;
                    break;
                }
                // doesn't exist yet
                Some(ResetMenu) => (),
                // quit the game
                Some(QuitGame) => return,
                _ => (),
            }
        }
    }
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

fn build_input_dispatcher() -> Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .with(UserInputSystem, "input", &[])
        .build()
}

fn build_turn_dispatcher() -> Dispatcher<'static, 'static> {
    specs::DispatcherBuilder::new()
        // system, "string id", &[dependencies]
        .with_thread_local(RenderingSystem)
        .with(AISystem, "ai", &[])
        .with(ExecuteActionSystem, "execute_actions", &[])
        .with(DeathSystem, "deaths", &[])
        .build()
}
