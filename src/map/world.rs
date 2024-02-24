use std::sync::{
    Arc,
    mpsc::channel
};
use threadpool::ThreadPool;
use rand::Rng;
use nannou::glam::Vec2;
use crate::{
    map::tile::Biome,
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
    pub wetlands_humidity: f32,
    pub desert_humidity: f32,
    pub cold_desert_temp: f32,
    pub grassland_low_t: f32,
    pub grassland_high_t: f32,
    pub tundra_low_t: f32,
    pub tundra_high_t: f32,
    pub forest_low_t: f32,
    pub forest_high_t: f32,
    pub cold_forest_low_t: f32,
    pub cold_forest_high_t: f32,
    pub global_heat_scaling: f32,
    pub altitude_scale: f32,
    pub temperature_scale: f32,
    pub humidity_scale: f32
}

pub struct World {
    pub seeds: [u32; 3],
    pub width: u32,
    pub height: u32,
    pub equator: f32,
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
        let equator = (height / 2) as f32;
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
                        &equator,
                        &noise,
                        &parameters,
                    );
                    tx.send(tile).unwrap();
                });
            }
        }

        drop(tx);
        let mut tiles = rx.iter().collect::<Vec<Tile>>();
        tiles.sort();

        {
            let mut coast_tiles = Vec::<usize>::new();
            let width = width as usize;
            let world_size = height as usize * width; 
            tiles.iter().enumerate().for_each(|(i, tile)| {
                if matches!(&tile.biome, Biome::Sea) {
                    let indices = 
                        if i == 0                                { vec!(i+1, i+width)               }  // first tile 
                        else if i == width - 1                   { vec!(i-1, i+width)               }  // last tile of first row
                        else if i == world_size - 1              { vec!(i-1, i-width)               }  // last tile
                        else if i == world_size - width          { vec!(i+1, i-width)               }  // first tile of last row
                        else if i % width == 0                   { vec!(i+1, i-width, i+width)      }  // first tile of row
                        else if i % width == width - 1           { vec!(i-1, i-width, i+width)      }  // last tile of row
                        else if i < width                        { vec!(i-1, i+1, i+width)          }  // first row
                        else if i > world_size - width           { vec!(i-1, i+1, i-width)          }  // last row
                        else                                     { vec!(i-1, i+1, i-width, i+width) }; // elsewhere
                        indices.iter().for_each(|j| {
                            if !matches!(&tiles[*j].biome, Biome::Sea) {
                                coast_tiles.push(*j);
                            };
                        });
                };
            });
            coast_tiles.iter().for_each(|t| {
                tiles[*t].biome = Biome::Coast;
            });
        }

        World { 
            seeds,
            width,
            height,
            equator,
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

