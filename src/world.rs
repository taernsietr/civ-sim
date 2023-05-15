use std::sync::mpsc;

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

        // let mut tiles = Vec::with_capacity(size);
        let threads = 8;
        let x_per_thread = width / threads;
        let x_remainder = width % threads;
        let y_per_thread = height / threads;
        let y_remainder = height % threads;

        let tiles: Vec<Tile> = std::thread::scope(|s| {
            let mut results = Vec::<Vec<Tile>>::new();

            for thread in 1..=threads {
                s.spawn(move || {
                    let mut tiles = Vec::new();
                    for x in (thread - 1 * x_per_thread)..(thread * x_per_thread) {
                        for y in (thread - 1 * y_per_thread)..(thread * y_per_thread) {
                            let tile = Tile::new(x, y, &noise);
                            tiles.push(tile);
                        }
                    }
                    println!("[MapGen] Thread {} is pushing to the tile array.", &thread);
                    results.push(tiles);
                });
            }

            if x_remainder + y_remainder > 0 {
                s.spawn(move || {
                    let mut tiles = Vec::new();
                    for x in (threads * x_per_thread)..(threads * x_per_thread + x_remainder) {
                        for y in (threads * y_per_thread)..(threads * y_per_thread + y_remainder) {
                            let tile = Tile::new(x, y, &noise);
                            tiles.push(tile);
                        }
                    }
                    println!("[MapGen] Thread remainder is pushing to the tile array.");
                    results.push(tiles);
                });
            }

            results.clone().into_iter().flatten().collect()
        });


        World { 
            seed,
            width,
            height,
            tiles,
        }
        
    }
}

