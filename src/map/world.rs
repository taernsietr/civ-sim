use std::sync::Arc;
use crate::map::tile::Tile;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

#[derive(Clone)]
pub struct WorldParameters {
    pub sea_level: f64,
    pub grassland_threshold: f64,
    pub swamp_threshold: f64,
    pub desert_threshold: f64,
    pub hill_threshold: f64,
    pub mountain_threshold: f64,
}

pub struct World {
    pub seeds: [u32; 3],
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>, 
}

impl World {
    pub fn new(
        seeds: [u32; 3],
        width: u32,
        height: u32,
        parameters: WorldParameters,
    ) -> World {
        let noise = [
            noise::Fbm::<noise::OpenSimplex>::new(seeds[0]),
            noise::Fbm::<noise::OpenSimplex>::new(seeds[1]),
            noise::Fbm::<noise::OpenSimplex>::new(seeds[2])
        ];
        let noise = Arc::new(noise);
        let parameters = Arc::new(parameters);
        let workers = 32;
        let pool = ThreadPool::new(workers);
        let (tx, rx) = channel::<Tile>();

        for x in 0..width {
            for y in 0..height {
                let tx = tx.clone();
                let noise = noise.clone();
                let parameters = parameters.clone();
                pool.execute(move || {
                    let tile = Tile::new(
                        x as u32 + width * y as u32,
                        x as f64,
                        y as f64,
                        &noise,
                        &parameters
                    );
                    tx.send(tile).unwrap();
                });
            }
        }

        drop(tx);
        let mut tiles = rx.iter().collect::<Vec<Tile>>();
        tiles.sort();

        World { 
            seeds,
            width,
            height,
            tiles,
        }
    }
}

