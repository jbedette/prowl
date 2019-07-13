use specs::{
    World,
    Builder,
};

mod systems;
use systems::{
    RivalSystem,
    DeathSystem,
    PrintStatsSystem,
    // PrintEntitySystem
};

mod components;
use components::{
    Named,
    Rivals,
    Health,
    Weapon,
    Money
};

mod renderer;
use renderer::TermionRenderer;

fn main() {
    TermionRenderer::render();
    return;
    // create a world
    let mut world = World::new();

    // initialize systems in the world
    // format:
    // system, "string id", dependencies (systems that must run before this one)
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(RivalSystem, "rival_system", &[])
        .with(DeathSystem, "death_system", &["rival_system"])
        .with(PrintStatsSystem, "print_stats_system", &[])
        // .with(PrintEntitySystem, "print_entity_system", &["death_system"])
        .build();
    dispatcher.setup(&mut world.res);

    // register all the components (setup isn't working correctly?)
    world.register::<Named>();
    world.register::<Rivals>();
    world.register::<Health>();
    world.register::<Money>();
    world.register::<Weapon>();

    // create some entities in the world
    world.create_entity()
        .with(Named::new("Mark"))
        .with(Rivals::new())
        .with(Health::new(8, 10))
        .with(Weapon::new(1))
        .with(Money::new(4))
        .build();
    world.create_entity()
        .with(Named::new("Melissa"))
        .with(Rivals::new())
        .with(Health::new(9, 12))
        .with(Weapon::new(2))
        .with(Money::new(4))
        .build();
    world.create_entity()
        .with(Named::new("Dumbo"))
        .with(Rivals::new())
        .with(Health::new(10, 10))
        .with(Weapon::new(1))
        .with(Money::new(4))
        .build();

    // run 20 steps
    for i in 0..20 {
        println!("!!!!!! SIMULATION STEP {} !!!!!!", i);
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
