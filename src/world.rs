// parallelize this using rayon
// 530.841.600

use crate::tile::Tile;

pub struct World {
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    // pub rotation_angle: u8,
    pub tiles: Vec<Tile>, 
    // pub border: Option<u32>,
}

impl World {
    pub fn new(seed: u32, dimensions: (u32, u32)) -> World {
        let (width, height) = dimensions;
        let size = (width * height) as usize;
        let noise = noise::OpenSimplex::new(seed);
        
        let mut tiles = Vec<Option<Tile>>::new();
        

        World { 
            seed,
            width,
            height,
            tiles,
        }
        
    }
}

