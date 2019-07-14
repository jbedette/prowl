use specs::{
    World,
    Builder,
    prelude::*
};
use termion::color;

mod systems;
use systems::{
    // RivalSystem,
    // DeathSystem,
    // PrintStatsSystem,
    // PrintEntitySystem
    RenderingSystem,
    UserInputSystem,
    AISystem,
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
    PendingActions,
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
    RendererResource,
    input::UserInput,
    Quit,
};

// mod renderer;
// use renderer::TermionRenderer;

fn main() {
    // TermionRenderer::render();
    // return;
    // create a world
    let mut world = World::new();
    resources::add_all(&mut world);

    // initialize systems in the world
    // format:
    // system, "string id", dependencies (systems that must run before this one)
    let mut dispatcher = specs::DispatcherBuilder::new()
        // .with(RivalSystem, "rival_system", &[])
        // .with(DeathSystem, "death_system", &[])
        // .with(PrintStatsSystem, "print_stats_system", &[])
        // .with(PrintEntitySystem, "print_entity_system", &["death_system"])
        .with(AISystem, "ai_system", &[])
        .with_thread_local(RenderingSystem)
        .with_thread_local(UserInputSystem)
        .build();
    // TODO why doesn't this work?
    // dispatcher.setup(&mut world.res);

    // Register all the components (setup isn't working correctly?)
    components::register(&mut world);


    // create some entities in the world
    // TODO make generator functions:
    // create_human() ?
    // Creator object ?
    // how to make random ?
    // what determines parameters ?
    world.create_entity()
        .with(Named::new("Mark"))
        .with(Rivals::new())
        .with(Health::new(8, 10))
        .with(Weapon::new(1))
        .with(Money::new(4))
        .with(Position::new(4, 8))
        .with(CharRenderer::new('M', color::Rgb(0x00, 0x95, 0xff)))
        .with(Player)
        .build();
    world.create_entity()
        .with(Named::new("Lysa"))
        .with(Rivals::new())
        .with(Health::new(9, 12))
        .with(Weapon::new(2))
        .with(Money::new(4))
        .with(Position::new(4, 10))
        .with(CharRenderer::new('L', color::Rgb(0x20, 0x76, 0xbb)))
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
        .with(CharRenderer::new('D', color::Rgb(0xff, 0x00, 0x95)))
        .with(AI::with_goal(ai::Goal::MoveRandom))
        .with(PendingActions::default())
        .build();

    run(world, dispatcher);
}

/// Main game loop.
fn run(mut world: World, mut dispatcher: Dispatcher) {
    let mut i = 0;
    loop {
        // must be in block, world can't be borrowed or dispatch will
        // be upset.
        {
            let mut console = world.write_resource::<Console>();
            // for _ in 0..10 {
                console.log(Log {
                    level: LogLevel::Debug,
                    message: format!("Simulation Step {}", i),
                });
            //}
        }
        dispatcher.dispatch(&mut world);
        world.maintain();
        let quit = world.read_resource::<Quit>();
        if quit.0 == true { break; }
        i += 1;
    }
}
