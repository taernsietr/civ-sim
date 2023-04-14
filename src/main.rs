use rand::Rng;
use noise::{RidgedMulti, Perlin, NoiseFn, Seedable};


#[derive(Debug)]
enum Biome {
    Grassland,
}

#[derive(Debug)]
struct Tile {
    x: u32,
    y: u32,
    biome: Option<Biome>,
    altitude: u8,
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

    fn position_seed(x: u32, y: u32) -> [f64; 2] {
        const PERLIN_FACTOR: f64 = 0.92731;
        const PERLIN_OFFSET: f64 = 0.00005;

        [
            (x as f64 / PERLIN_FACTOR) + PERLIN_OFFSET,
            (y as f64 / PERLIN_OFFSET) + PERLIN_OFFSET
        ]
    }
}

// Scales a f64 within [-1.0, 1.0] to a u8 within [0, 255]
fn perlin_to_u8(input: f64) -> u8 {
    (((input + 1.0) / 2.0) * 255.0) as u8
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
                    altitude:    perlin_to_u8(noise.get(Tile::position_seed(x, y))),
                    temperature: perlin_to_u8(noise.get(Tile::position_seed(x, y))),
                    humidity:    perlin_to_u8(noise.get(Tile::position_seed(x, y))),
                    vegetation:  perlin_to_u8(noise.get(Tile::position_seed(x, y))),
                    hardness:    perlin_to_u8(noise.get(Tile::position_seed(x, y))),
                    sunlight:    perlin_to_u8(noise.get(Tile::position_seed(x, y)))
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
