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
    Player
};

mod resources;
use resources::{
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
    // world.add_resource is in the tutorials, but deprecated
    world.add_resource(RendererResource::new());
    world.add_resource(UserInput::new());
    world.add_resource(Quit(false));

    // initialize systems in the world
    // format:
    // system, "string id", dependencies (systems that must run before this one)
    let mut dispatcher = specs::DispatcherBuilder::new()
        // .with(RivalSystem, "rival_system", &[])
        // .with(DeathSystem, "death_system", &["rival_system"])
        // .with(PrintStatsSystem, "print_stats_system", &[])
        // .with(PrintEntitySystem, "print_entity_system", &["death_system"])
        // .with(UserInputSystem, "user_input", &[])
        .with_thread_local(RenderingSystem)
        .with_thread_local(UserInputSystem)
        .build();
    // dispatcher.setup(&mut world.res);

    // register all the components (setup isn't working correctly?)
    world.register::<Named>();
    world.register::<Rivals>();
    world.register::<Health>();
    world.register::<Money>();
    world.register::<Weapon>();
    world.register::<Position>();
    world.register::<CharRenderer>();
    world.register::<Player>();


    // create some entities in the world
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
        .build();
    world.create_entity()
        .with(Named::new("Dumbo"))
        .with(Rivals::new())
        .with(Health::new(10, 10))
        .with(Weapon::new(1))
        .with(Money::new(4))
        .with(Position::new(2, 10))
        .with(CharRenderer::new('D', color::Rgb(0xff, 0x00, 0x95)))
        .build();

    // for _i in 0..5 {
    loop {
        // println!("!!!!!! SIMULATION STEP {} !!!!!!", i);
        dispatcher.dispatch(&mut world);
        world.maintain();
        let quit = world.read_resource::<Quit>();
        if quit.0 == true {
            break 
        }
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
