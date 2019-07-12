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
struct Human {
    health: Stat,
    money: Stat,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Named {
    value: String
}

impl Named {
    fn new(name: &str) -> Self {
        let name = String::from(name);
        Self { value: name }
    }
}

impl Human {
    fn new(health: i32, money: i32) -> Self {
        let health = Stat::new("Health", health);
        let money = Stat::new("Money", money);
        Self {
            health,
            money
        }
    }
}

#[derive(Debug)]
struct Stat {
    name: String,
    value: i32
}

impl Stat {
    fn new(name: &str, value: i32) -> Self {
        let name = String::from(name);
        Self {
            name,
            value
        }
    }
}

fn main() {
    let mut world = World::new();

    // Register's Entity Components
    world.register::<Human>();
    world.register::<Named>();

    // Creates entities in the world
    world.create_entity()
        .with(Named::new("Mark"))
        .with(Human::new(10, 12))
        .build();
    world.create_entity()
        .with(Named::new("Melissa"))
        .with(Human::new(10, 12))
        .build();

    // Dispaches systems into the world.
    let mut dispatcher = specs::DispatcherBuilder::new()
        // system, name, dependencies
        .with(HumanSystem, "human_system", &[])
        .build();

    for _i in 0..20 {
        dispatcher.dispatch(&mut world.res);
    }
    world.maintain();
}

struct HumanSystem;

impl<'a> specs::System<'a> for HumanSystem {
    type SystemData = (
        specs::WriteStorage<'a, Human>,
        specs::ReadStorage<'a, Named>,
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (mut human, name) = data;

        for (human, name) in (&mut human, &name).join() {
            println!("----------------------");
            println!("Name   : {}", name.value);
            println!("Money  : {}", human.money.value);
            println!("Health : {}", human.health.value);
            human.health.value -= 1;
            if name.value == "Melissa" {
                human.health.value -= 1;
            }
            if human.health.value < 5 {
                if human.money.value > 0 {
                    human.money.value -= 1;
                    human.health.value += 5;
                }
            } else {
                human.money.value += 1;
            }
        }
    }
}
