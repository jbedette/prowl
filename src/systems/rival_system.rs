use specs::{
    ReadStorage,
    WriteStorage,
    Entities,
    System
};
use crate::components::{
    Named,
    Rivals,
    Health,
    Weapon
};

pub struct RivalSystem;

impl<'a> System<'a> for RivalSystem {
    type SystemData = (
        ReadStorage<'a, Named>,
        WriteStorage<'a, Rivals>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Weapon>,
        Entities<'a>
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

