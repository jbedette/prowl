use specs::{
    World,
    Builder,
    Component,
    VecStorage
};

use specs_derive::{
    Component
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Named {
    value: String
}

impl Named {
    fn new(value: &str) -> Self {
        let value = String::from(value);
        Self {
            value
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Weapon {
    damage: u64,
}

impl Weapon {
    fn new(damage: u64) -> Self {
        Self { damage }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Health {
    current: i64,
    max: i64,
}

impl Health {
    fn new(current: i64, max: i64) -> Self {
        Self {
            current,
            max
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Money {
    current: u64,
}

impl Money {
    fn new(current: u64) -> Self {
        Self {
            current,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Rivals {
    entities: Vec<specs::Entity>
}

impl Rivals {
    fn new() -> Self {
        Self {
            entities: vec![]
        }
    }
}

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

struct RivalSystem;

impl<'a> specs::System<'a> for RivalSystem {
    type SystemData = (
        specs::ReadStorage<'a, Named>,
        specs::WriteStorage<'a, Rivals>,
        specs::WriteStorage<'a, Health>,
        specs::WriteStorage<'a, Weapon>,
        specs::Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            names,
            mut rivalses,
            mut healths,
            weapons,
            entities
        ) = data;

        for (entity, rivals) in (&entities, &mut rivalses).join() {
            let name = names.get(entity).unwrap();
            // must have at least one rival
            if rivals.entities.is_empty() {
                for rival_entity in (entities).join() {
                    if entity != rival_entity {
                        rivals.entities.push(rival_entity);
                        let rival_name = names.get(rival_entity).unwrap();
                        println!("{} has become rivals with {}", name.value, rival_name.value);
                        break;
                    }
                }
            } else {
                let weapon = weapons.get(entity).unwrap();
                // hurt each rival
                for rival_entity in &rivals.entities {
                    let rival_health = healths.get_mut(*rival_entity);
                    let rival_name = names.get(*rival_entity);
                    match rival_health {
                        Some(health) => {
                            let damage = weapon.damage;
                            health.current -= damage as i64;
                            println!("{} attacked their rival {} for {} dmg",
                                     name.value, rival_name.unwrap().value, damage);
                        },
                        None => { println!("ERROR: RIVAL NOT FOUND"); println!("Entity Val:{:?}", rival_entity) }
                    }
                }
            }
        }
    }
}

struct PrintStatsSystem;

impl<'a> specs::System<'a> for PrintStatsSystem {
    type SystemData = (
        specs::ReadStorage<'a, Named>,
        specs::ReadStorage<'a, Health>,
        // specs::ReadStorage<'a, Weapon>,
        // specs::Entities<'a>
        );
    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            names,
            healths,
            ) = data;

        for (name, health) in (&names, &healths).join() {
            println!("NAME: {}", name.value);
            println!("HP  : {}/{}", health.current, health.max);
        }
    }
}

struct DeathSystem;

impl<'a> specs::System<'a> for DeathSystem {
    type SystemData = (
        specs::ReadStorage<'a, Named>,
        specs::ReadStorage<'a, Health>,
        specs::Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            names,
            healths,
            entities
            ) = data;

        for (health, entity) in (&healths, &entities).join() {
            if health.current <= 0 {
                let name = names.get(entity);
                let name_string = match name {
                    Some(name) => name.value.to_owned(),
                    None => String::from("UNNAMED ENTITY"),
                };
                let _result = entities.delete(entity);
                println!("{} has died!", name_string);
            }
        }
    }
}

struct PrintEntitySystem;

impl<'a> specs::System<'a> for PrintEntitySystem {
    type SystemData = (
        specs::Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let entities = data;

        let mut active_entities = 0;
        for entity in (entities).join() {
            active_entities += 1;
            println!("{:?}", entity);
        }
        println!("TOTAL: {}", active_entities);
    }
}
