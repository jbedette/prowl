use specs::prelude::*;
use crate::file_io;

use crate::{
    components::{
        map::TileMap,
        Named,
        game_resources::{
            Wood,
            Metal,
            Food,
            Water,
            GameResource,
            // GameResourceType,
        }
    },
    actors::Island,
    shared::{
        Vector2,
        random::random_range,
    }
};

pub struct IslandSetupSystem;

#[allow(unused_must_use)]
#[allow(dead_code)]
impl<'a> System<'a> for IslandSetupSystem {
    type SystemData = (
        WriteStorage<'a, Island>,
        WriteStorage<'a, Named>,
        WriteStorage<'a, TileMap>,
        // resources
        WriteStorage<'a, GameResource<Water>>,
        WriteStorage<'a, GameResource<Food>>,
        WriteStorage<'a, GameResource<Metal>>,
        WriteStorage<'a, GameResource<Wood>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut islands,
            mut names,
            mut maps,
            waters,
            foods,
            metals,
            woods,
            entities
        ) = data;
        let island_names = file_io::read_file("isoles.txt");

        let mut resources = (
            waters,
            foods,
            metals,
            woods
            );

        for map in (&mut maps).join() {
            let water_level = map.get_water_level();
            for x in 0..map.size.x {
                for y in 0..map.size.y {
                    let position = Vector2::new(x, y);
                    // if this position is already claimed by another thing, just move along.
                    if map.get_entity(position).is_some() { continue; }
                    // otherwise check the tile.
                    if let Some(tile) = map.get_tile(position) {
                        if tile.height < water_level { continue; }
                        // if island is above water,
                        // add island to tile
                        let island_entity = entities.create();
                        let name = &(island_names[(random_range(0, island_names.len()))]);
                        names.insert(island_entity, Named::new(name));
                        let island_positions = map.add_island(position, island_entity);
                        // Add resources in clusters.
                        // TODO this is super gross and kind of broken
                        {
                            let size = island_positions.len();
                            let resource_spaces = size / random_range(8, 12);
                            const TOTAL_COUNT : usize = 4;
                            let mut resource_counts = [0; TOTAL_COUNT];
                            for _ in 0..resource_spaces {
                                let which = random_range(0, 4);
                                resource_counts[which] += 1;
                            }
                            // scarcity TODO this is a total nasty hack...
                            // water
                            resource_counts[0] = (resource_counts[0] as f32 * 1.1) as i32;
                            // food
                            resource_counts[1] = (resource_counts[1] as f32 * 1.0) as i32;
                            // metal
                            resource_counts[2] = (resource_counts[2] as f32 * 0.3) as i32;
                            // wood
                            resource_counts[3] = (resource_counts[3] as f32 * 1.4) as i32;
                            // let all_visited = vec![];
                            for i in 0..TOTAL_COUNT {
                                for _ in 0..(resource_counts[i]) {
                                    let root_position = island_positions[random_range(0, size)];
                                    add_resource(map, root_position, i, &mut resources, island_entity);
                                    let mut visited = vec![root_position];
                                    let spread_to = [
                                        Vector2::north(),
                                        Vector2::south(),
                                        Vector2::east(),
                                        Vector2::west()
                                    ];
                                    for _ in 0..resource_counts[i] {
                                        // horribly hacky do..while loop
                                        // since rust doesn't have them
                                        let mut pos;
                                        while {
                                            pos = visited[random_range(0, visited.len())];
                                            pos = pos + spread_to[random_range(0, spread_to.len())];
                                            visited.contains(&pos)
                                        } {}
                                        if island_positions.contains(&pos) {
                                            add_resource(map, pos, i, &mut resources, island_entity);
                                            visited.push(pos);
                                        }
                                    }
                                }
                            }
                        }
                        islands.insert(island_entity, Island::new(island_positions));
                    } else {
                        eprintln!("ERROR: INVALID POSITION IN MAP GENERATION");
                    }
                }
            }
        }
    }
}

// TODO any way to make this less repetitive?? ugh
fn add_resource(
    map: &mut TileMap,
    position: Vector2,
    identifier: usize,
    resources: &mut (
        WriteStorage<'_, GameResource<Water>>,
        WriteStorage<'_, GameResource<Food>>,
        WriteStorage<'_, GameResource<Metal>>,
        WriteStorage<'_, GameResource<Wood>>,
    ),
    entity: Entity,
    )
{
    match identifier {
        0 => {
            if map.add_water(position).is_ok() {
                if let Some(resource) = resources.0.get_mut(entity) {
                    resource.adjust_count(1000);
                } else {
                    let mut resource = GameResource::<Water>::new();
                    resource.adjust_count(1000);
                    resources.0.insert(entity, resource);
                }
            }
        },
        1 => {
            if map.add_food(position).is_ok() {
                if let Some(resource) = resources.1.get_mut(entity) {
                    resource.adjust_count(1000);
                } else {
                    let mut resource = GameResource::<Food>::new();
                    resource.adjust_count(1000);
                    resources.1.insert(entity, resource);
                }
            }
        },
        2 => {
            if map.add_metal(position).is_ok() {
                if let Some(resource) = resources.2.get_mut(entity) {
                    resource.adjust_count(1000);
                } else {
                    let mut resource = GameResource::<Metal>::new();
                    resource.adjust_count(1000);
                    resources.2.insert(entity, resource);
                }
            }
        },
        3 => {
            if map.add_wood(position).is_ok() {
                if let Some(resource) = resources.3.get_mut(entity) {
                    resource.adjust_count(1000);
                } else {
                    let mut resource = GameResource::<Wood>::new();
                    resource.adjust_count(1000);
                    resources.3.insert(entity, resource);
                }
            }
        },
        _ => (),
    };
}
