use std::sync::{
    Arc,
    mpsc::channel
};
use threadpool::ThreadPool;
use rand::Rng;
use nannou::glam::Vec2;
use crate::{
    utils::cli::Args,
    map::tile::Tile
};

#[derive(Clone)]
pub struct WorldParameters {
    pub sea_level: f64,
    pub swamp_humidity: f64,
    pub desert_humidity: f64,
    pub hill_altitude: f64,
    pub mountain_altitude: f64,
    pub altitude_scale: f64,
    pub temperature_scale: f64,
    pub humidity_scale: f64
}

pub struct World {
    pub seeds: [u32; 3],
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>, 
}

impl World {
    pub fn new(
        args: &Args,
        parameters: &WorldParameters,
    ) -> World {
        let mut rng = rand::thread_rng();
        let width = args.x;
        let height = args.y;
        let seeds = match &args.seeds {
            None => [rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>()],
            Some(j) if j.len() == 1 => [j[0], rng.gen::<u32>(), rng.gen::<u32>()],
            Some(j) if j.len() == 2 => [j[0], j[1], rng.gen::<u32>()],
            Some(j) if j.len() >= 3 => [j[0], j[1], j[2]],
            _ => unreachable!()
        };
        let noise = [
            noise::Fbm::<noise::SuperSimplex>::new(seeds[0]),
            noise::Fbm::<noise::SuperSimplex>::new(seeds[1]),
            noise::Fbm::<noise::SuperSimplex>::new(seeds[2])
        ];
        let noise = Arc::new(noise);
        let parameters = Arc::new(parameters.clone());
        let workers = 32;
        let pool = ThreadPool::new(workers);
        let (tx, rx) = channel::<Tile>();

        println!("[MapGen] Building world using seeds [{}, {}, {}]", seeds[0], seeds[1], seeds[2]);
        for x in 0..width {
            for y in 0..height {
                let tx = tx.clone();
                let noise = noise.clone();
                let parameters = parameters.clone();
                pool.execute(move || {
                    let tile = Tile::new(
                        x + width * y,
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
        Self::shape_continent(width, height, &mut tiles);

        World { 
            seeds,
            width,
            height,
            tiles,
        }
    }

    fn shape_continent(width: u32, height: u32, tiles: &mut [Tile]) {
        println!("[MapGen] Shaping continent...");
        let pos_0 = Vec2::new(0.0, 0.0);
        let center = Vec2::new(
            (width / 2) as f32,
            (height / 2) as f32
        );
        let dist_0 = pos_0.distance(center) as f64;
        for tile in tiles.iter_mut() {
            let position = Vec2::new(tile.x as f32, tile.y as f32);
            let distance_from_center = position.distance(center) as f64;
            tile.altitude *= distance_from_center / dist_0 * 5.0;
        };
        println!("[MapGen] Finished processing continent.");
    }

}

