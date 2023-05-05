use rand::Rng;
// use noise::{NoiseFn, Seedable};

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
    #[allow(dead_code)]
    pub fn new(parameters: &WorldCreationParameters) -> World {
        let mut tiles = Vec::with_capacity((parameters.dimensions.0 * parameters.dimensions.1) as usize);
        let mut rng = rand::thread_rng();
        let noise = noise::OpenSimplex::new(rng.gen::<u32>());
        // let rotation_angle = rng.gen::<u8>();

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

    pub fn new_mt(parameters: &WorldCreationParameters) -> World {
        let (width, height) = parameters.dimensions;
        let mut rng = rand::thread_rng();
        let noise = noise::OpenSimplex::new(rng.gen::<u32>());
        let size = (parameters.dimensions.0 * parameters.dimensions.1) as usize;
        let mut tiles = Vec::with_capacity(size);

        // TODO: Write this properly, maybe scaling with available CPU cores?
        let mut part1 = Vec::with_capacity(size/4);
        let mut part2 = Vec::with_capacity(size/4);
        let mut part3 = Vec::with_capacity(size/4);
        let mut part4 = Vec::with_capacity(size/4);

        std::thread::scope(|s| { 
            s.spawn(|| {
                for x in 0..width/2 {
                    for y in 0..height/2 {
                        let tile = Tile::new(x, y, &noise);
                        part1.push(tile);
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..width {
                    for y in 0..height/2 {
                        let tile = Tile::new(x, y, &noise);
                        part2.push(tile);
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/2 {
                    for y in height/2..height {
                        let tile = Tile::new(x, y, &noise);
                        part3.push(tile);
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..width {
                    for y in height/2..height {
                        let tile = Tile::new(x, y, &noise);
                        part4.push(tile);
                    }
                }
            });
        });

        tiles.append(&mut part1);
        tiles.append(&mut part2);
        tiles.append(&mut part3);
        tiles.append(&mut part4);

        World { 
            width: parameters.dimensions.0,
            height: parameters.dimensions.1,
            rotation_angle: 0,
            tiles,
        }
        
    }
}

