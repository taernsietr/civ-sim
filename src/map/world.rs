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
    pub sea_level: f32,
    pub peak_height: f32,
    pub mountain_height: f32,
    pub hills_height: f32,
    pub glacier_temp: f32,
    pub grassland_threshold: f32,
    pub forest_threshold: f32,
    pub swamp_threshold: f32,
    pub tundra_low_t: f32,
    pub tundra_high_t: f32,
    pub altitude_scale: f32,
    pub temperature_scale: f32,
    pub humidity_scale: f32
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
                        x as f32,
                        y as f32,
                        &noise,
                        &parameters,
                        &width,
                        &height
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

    fn _shape_continent(width: u32, height: u32, tiles: &mut [Tile]) {
        println!("[MapGen] Shaping continent...");
        let pos_0 = Vec2::new(0.0, 0.0);
        let center = Vec2::new(
            (width / 2) as f32,
            (height / 2) as f32
        );
        let dist_0 = pos_0.distance(center);
        for tile in tiles.iter_mut() {
            let position = Vec2::new(tile.x, tile.y);
            let distance_from_center = position.distance(center);
            //tile.altitude *= distance_from_center / dist_0 * 5.0;
            tile.altitude *= distance_from_center / dist_0;
        };
        println!("[MapGen] Finished processing continent.");
    }

}

