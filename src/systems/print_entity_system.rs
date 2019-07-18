/// Prints all entities in the world.

use specs::{
    Entities,
    System
};

pub struct PrintEntitySystem;

impl<'a> System<'a> for PrintEntitySystem {
    type SystemData = (
        Entities<'a>
        );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let entities = data;

        println!("ACTIVE ENTITIES:");
        let mut active_entities = 0;
        for entity in (entities).join() {
            active_entities += 1;
            println!("{:?}", entity);
        }
        println!("TOTAL: {}", active_entities);
    }
}
