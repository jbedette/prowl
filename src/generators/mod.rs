// Generators module is used in creating the game map
//  - uses randomization and perlin noise to create realistic map shapes
//  - based on the values created, tiles are assigned game functionality
pub mod terrain_generator;
// pub mod map;


pub use terrain_generator::{
    generate_heightmap,
};
