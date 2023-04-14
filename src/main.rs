use rand::Rng;
use noise::{RidgedMulti, Perlin, NoiseFn, Seedable};

const PERLIN_FACTOR: f64 = 0.92731;
const PERLIN_OFFSET: f64 = 0.00005;

#[derive(Debug)]
enum Biome {
    Grassland,
}

#[derive(Debug)]
struct Tile {
    x: u32,
    y: u32,
    biome: Option<Biome>,
    height: u8,
    temperature: u8,
    humidity: u8,
    vegetation: u8,
    hardness: u8,
    sunlight: u8,
}

#[derive(Debug)]
struct World {
    width: u32,
    height: u32,
    tiles: Vec<Tile>, 
}

struct WorldCreationParameters {
    width: u32,
    height: u32,
}

impl Tile {
    fn calculate_biome(&mut self) {
        self.biome = Some(Biome::Grassland);
    }
}

impl World {
    fn new(parameters: WorldCreationParameters) -> World {
        let mut rng = rand::thread_rng();
        let noise = RidgedMulti::<Perlin>::new(rng.gen::<u32>());
        let mut tiles = Vec::with_capacity((parameters.width * parameters.height) as usize);

        for x in 0..parameters.width {
            for y in 0..parameters.height {
                let mut tile = Tile {
                    x, y, biome: None,
                    height:      (((noise.get([(x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET, (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET]) + 1.0) / 2.0) * 255.0) as u8,
                    temperature: (((noise.get([(x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET, (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET]) + 1.0) / 2.0) * 255.0) as u8,
                    humidity:    (((noise.get([(x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET, (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET]) + 1.0) / 2.0) * 255.0) as u8,
                    vegetation:  (((noise.get([(x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET, (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET]) + 1.0) / 2.0) * 255.0) as u8,
                    hardness:    (((noise.get([(x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET, (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET]) + 1.0) / 2.0) * 255.0) as u8,
                    sunlight:    (((noise.get([(x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET, (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET]) + 1.0) / 2.0) * 255.0) as u8
                };
                tile.calculate_biome();
                tiles.push(tile);
            }
        }

        World { 
            width: parameters.width,
            height: parameters.height,
            tiles,
        }
    }
}

fn main() {
    let parameters = WorldCreationParameters { width: 2, height: 2 };
    let world = World::new(parameters);
    dbg!(world);
}
