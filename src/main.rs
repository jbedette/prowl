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
    ai,
    TileMap,
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
        MoveableEntity,
    }
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

const MAP_SIZE: i32 = 500;

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
    make_person(&mut world, true);
    // populate gameworld
    for _ in 0..2000 { make_person(&mut world, false); }
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
        // allow input
        // std::thread::sleep(std::time::Duration::from_millis(20));
        input.dispatch(&world);
        // check if state change is requested (only quit works for now)
        let game_data = &mut world.write_resource::<GameData>();
        use resources::game_data::StateChangeRequest::*;
        match game_data.state_change_request {
            Some(ResetMenu) => (),
            // quit
            Some(QuitGame) => break,
            _ => (),
        }
    }
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
            .with(MoveableEntity::default())
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
            .with(MoveableEntity::default())
            .with(AI::with_goal(ai::Goal::MoveRandom))
            .build();
    }
}
