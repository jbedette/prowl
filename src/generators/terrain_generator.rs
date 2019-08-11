use noise::{
    NoiseFn,
    // Perlin, // acting funny?
    OpenSimplex
};

use crate::shared::{
    Vector2,
    normalize_vec,
};

pub fn generate_heightmap(bounds: Vector2) -> Vec<f64> {
    // let perlin = Perlin::new();
    let perlin = OpenSimplex::new();
    let (width, height) = bounds.to_tuple();
    // perlin::get([4.0, 4.0])
    // println!("size: [{},{}]", width, height);
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
            let mut height = perlin.get([x, y]);
            // big areas
            let x = x / 140.0;
            let y = y / 140.0;
            height += perlin.get([x, y]) * 2.0;
            // fine noise
            let x = x * 400.0;
            let y = y * 400.0;
            height += (perlin.get([x, y])) / 8.0;
            let x = x * 200.0;
            let y = y * 200.0;
            height -= (perlin.get([x, y])) / 16.0;
            // height /= 2.0;
            // height += 0.5;
            // println!("noise: [{},{}]: {:?}", x, y, height);
            if height > max { max = height; }
            if height < min { min = height; }
            heightmap.push(height);
        }
    }
    println!("MAX: {}, MIN: {}", max, min);
    // println!("{:?}", heightmap);
    normalize_vec(heightmap, min, max)
}
