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

// An entity with a name
impl Named {
    fn new(name: &str) -> Self {
        let name = String::from(name);
        Self { value: name }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Human {
    health: Stat,
    money: Stat,
    rival: Option<specs::Entity>
}
// An entity that is human
impl Human {
    fn new(health: i32, money: i32) -> Self {
        let health = Stat::new("Health", health);
        let money = Stat::new("Money", money);
        Self {
            health,
            money,
            rival: None,
        }
    }

    fn adjust_health(&mut self, value: i32) {
        self.health.value -= value;
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

/*
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct EntityList(Vec<specs::Entity>);
*/

fn main() {
    let mut world = World::new();

    // Register's Entity Components, not needed since setup
    // world.register::<Human>();
    // world.register::<Named>();

    // Dispaches systems into the world.
    let mut dispatcher = specs::DispatcherBuilder::new()
        // system, name, dependencies
        .with(HumanSystem, "human_system", &[])
        .build();
    // Setup so we don't need to register every component individually
    dispatcher.setup(&mut world.res);

    // Creates entities in the world
    world.create_entity()
        .with(Named::new("Mark"))
        .with(Human::new(10, 12))
        .build();
    world.create_entity()
        .with(Named::new("Melissa"))
        .with(Human::new(4, 1))
        .build();


    // runs 20 simulation steps
    for _i in 0..20 {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}

struct HumanSystem;

impl<'a> specs::System<'a> for HumanSystem {
    type SystemData = (
        specs::WriteStorage<'a, Human>,
        specs::ReadStorage<'a, Named>,
        specs::Entities<'a>,
        );

    fn run(&mut self, (mut humans, names, entities): Self::SystemData) {
        use specs::Join;
        // let (mut human, name, entities) = data;

        /*
        for (human, name, entity) in (&mut humans, &names, &entities).join() {
            println!("----------------------");
            println!("Name   : {}", name.value);
            println!("Money  : {}", human.money.value);
            println!("Health : {}", human.health.value);
            human.health.value -= 1;
            /*
            if name.value == "Melissa" {
                human.health.value -= 1;
            }
            */
            for entity1 in (&entities).join() {
                if human.rival == None {
                    if entity != entity1 {
                        human.rival = Some(entity1);
                        println!("{} has become rivals with {}",
                                 name.value,
                                 names.get(human.rival.unwrap()).unwrap().value);
                        break;
                    }
                } else if Some(entity1) == human.rival {
                    let mut rival_human = humans.get(human.rival.unwrap()).unwrap();
                }
            }
            if human.health.value <= 0 {
                let _result = entities.delete(entity);
                println!("----------------------");
                println!("EVENT: {} has died", name.value);
            }
            if human.health.value < 5 {
                if human.money.value > 0 {
                    human.money.value -= 1;
                    human.health.value += 2;
                }
            } else {
                human.money.value += 1;
            }
        }
    */

        for (human, name, entity) in (&mut humans, &names, &entities).join() {
            println!("----------------------");
            println!("Name   : {}", name.value);
            println!("Money  : {}", human.money.value);
            println!("Health : {}", human.health.value);
            // let human = &humans.get_mut(entity).unwrap();
            // human.health.value -= 1;
            human.adjust_health(-1);
            for entity1 in (&entities).join() {
                if human.rival == None {
                    if entity != entity1 {
                        human.rival = Some(entity1);
                        println!("{} has become rivals with {}",
                                 name.value,
                                 names.get(human.rival.unwrap()).unwrap().value);
                        break;
                    }
                } else if Some(entity1) == human.rival {
                    /*
                    let rival_human = humans.get_mut(human.rival.unwrap()).unwrap();
                    rival_human.adjust_health(-1);
                    */
                    /*
                    |humans| {
                        let rival_human = humans.get_mut(human.rival.unwrap()).unwrap();
                        rival_human.adjust_health(-1);
                        ()
                    }
                    */
                }
            }
            if human.health.value <= 0 {
                let _result = entities.delete(entity);
                println!("----------------------");
                println!("EVENT: {} has died", name.value);
            }
            if human.health.value < 5 {
                if human.money.value > 0 {
                    human.money.value -= 1;
                    human.health.value += 2;
                }
            } else {
                human.money.value += 1;
            }
        }
    }
}
