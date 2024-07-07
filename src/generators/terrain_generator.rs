use noise::{
    NoiseFn,
    OpenSimplex,
    Seedable,
};

use crate::shared::{
    Vector2,
    normalize_vec,
};

// generate_heightmap
// uses OpenSimplex to set bounds of height,
// goes through map assigning values to tiles,
// used later in determining map tile properties

pub fn generate_heightmap(bounds: Vector2, seed: u32) -> Vec<f64> {
    let perlin = OpenSimplex::new(seed).set_seed(seed);
    let (width, height) = bounds.to_tuple();
    let mut heightmap = vec![];
    let mut max = 0.0;
    let mut min = 1000.0;
    for y in 0..width {
        for x in 0..height {
            let x = x as f64;
            let y = y as f64;
            // medium areas
            let x = x / 10.0;
            let y = y / 10.0;
            let mut height = perlin.get([x, y, 1.0]) * 1.5;
            // mediumer areas
            let x = x / 10.0;
            let y = y / 10.0;
            height += perlin.get([x, y, 80.0]) * 2.4;
            // big areas
            let x = x / 140.0;
            let y = y / 140.0;
            height += perlin.get([x, y, -700.0]) * 2.0;
            // fine noise
            let x = x * 600.0;
            let y = y * 600.0;
            height += (perlin.get([x, y])) * 5.0;
            let x = x * 8.0;
            let y = y * 8.0;
            height += (perlin.get([x, y])) * 0.3;
            let x = x * 1.2;
            let y = y * 1.2;
            height += (perlin.get([x, y, 1000.0])) * 0.4;
            let x = x * 1.2;
            let y = y * 1.2;
            height += (perlin.get([x, y, 7000.0])) * 0.1;
            if height > max { max = height; }
            if height < min { min = height; }
            heightmap.push(height);
        }
    }
    println!("MAX: {}, MIN: {}", max, min);
    normalize_vec(heightmap, min, max)
}
