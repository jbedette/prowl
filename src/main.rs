use specs::{
    World,
    Builder,
};

mod systems;
use systems::{
    RivalSystem,
    DeathSystem,
    PrintStatsSystem,
    PrintEntitySystem
};

mod components;
use components::{
    Named,
    Rivals,
    Health,
    Weapon,
    Money
};

fn main() {
    let mut world = World::new();
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(RivalSystem, "rival_system", &[])
        .with(PrintStatsSystem, "print_stats_system", &[])
        .with(DeathSystem, "death_system", &["rival_system"])
        .with(PrintEntitySystem, "print_entity_system", &["death_system"])
        .build();
    dispatcher.setup(&mut world.res);

    world.register::<Named>();
    world.register::<Rivals>();
    world.register::<Health>();
    world.register::<Money>();
    world.register::<Weapon>();

    // create entities in the world
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
        .with(Health::new(8, 10))
        .with(Weapon::new(2))
        .with(Money::new(4))
        .build();

    for i in 0..20 {
        println!("!!!!!! SIMULATION STEP {} !!!!!!", i);
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
