use noise::{NoiseFn, Perlin, OpenSimplex};

use crate::shared::{
    Vector2,
};

pub fn generate_heightmap(bounds: Vector2) -> Vec<f64> {
    // let perlin = Perlin::new();
    let perlin = OpenSimplex::new();
    let (width, height) = bounds.to_tuple();
    // perlin::get([4.0, 4.0])
    // println!("size: [{},{}]", width, height);
    let mut heightmap = vec![];
    for y in 0..width {
        for x in 0..height {
            let x = x as f64;
            let y = y as f64;
            let x = x / 10.0;
            let y = y / 10.0;
            let mut height = perlin.get([x, y]);
            height += 0.5;
            println!("noise: [{},{}]: {:?}", x, y, height);
            heightmap.push(height);
        }
    }
    // println!("{:?}", heightmap);
    heightmap
}
