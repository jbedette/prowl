use specs::{
    Component,
    VecStorage,
};
// #[macro_use]
use specs_derive::{
    Component
};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32
}

use specs::{
    World,
    Builder
};

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.create_entity()
        .with(Position { x: 4.0, y: 7.2 })
        .build();
    world.create_entity()
        .with(Position { x: 9.0, y: 7.2 })
        .with(Velocity { x: 100.0, y: 100.0 })
        .build();
    world.add_resource(DeltaTime(0.2));
    // let mut delta = world.write_resource::<DeltaTime>();
    // *delta = DeltaTime(0.04);

    use specs::DispatcherBuilder;

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    // let mut hello_world = HelloWorld;

    // use specs::RunNow;
    // hello_world.run_now(&world.res);
    dispatcher.dispatch(&mut world.res);
    world.maintain();
}

use specs::{
    Read,
    ReadStorage,
    WriteStorage,
    System,
};

// System
#[derive(Default)]
struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (delta, vel, mut pos) = data;

        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }

}

#[derive(Default)]
struct DeltaTime(f32);


/*
fn make_actor(world: &mut World) {
    world.create_entity().with(
        AnimalStats {
            health: 32,
            hunger: 0,
            hunger_threshold: 12,
        }
        ).build();
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Animal {
    health: i32,
    hunger: i32,
    hunger_threshold: i32,
}
*/

// Actor
//   Stats

// Actions
// " affect an actors stats
// " potentially another actors stats
// " have a cost
// " take time

/*
struct Action {
    cost: StatSet,
    affect
}
*/

// fn action<T>(
//   cost: StatGroup<T>,
//   affected: Vec<Actor>,
//   
// )

/*
trait Actor {}

// simplest actor: resource
struct Resource {
    name: String,
    gain_per_step: i32
}

impl Actor for Resource {}
*/
