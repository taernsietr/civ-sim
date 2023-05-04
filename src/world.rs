use rand::Rng;
use noise::{NoiseFn, Seedable};

use crate::tile::Tile;

pub struct WorldCreationParameters {
    pub dimensions: (u32, u32)
}

#[derive(Debug)]
pub struct World {
    pub width: u32,
    pub height: u32,
    pub rotation_angle: u8,
    pub tiles: Vec<Tile>, 
}

impl World {
    pub fn new(parameters: &WorldCreationParameters) -> World {
        let mut tiles = Vec::with_capacity((parameters.dimensions.0 * parameters.dimensions.1) as usize);
        let mut rng = rand::thread_rng();

        // let rotation_angle = rng.gen::<u8>();
        let noise = noise::OpenSimplex::new(rng.gen::<u32>());

        for x in 0..parameters.dimensions.0 {
            for y in 0..parameters.dimensions.1 {
                let tile = Tile::new(x, y, &noise);
                tiles.push(tile);
            }
        }

        World { 
            width: parameters.dimensions.0,
            height: parameters.dimensions.1,
            rotation_angle: 0,
            tiles,
        }
    }
}

