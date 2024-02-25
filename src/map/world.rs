use std::sync::{
    Arc,
    mpsc::channel
};
use threadpool::ThreadPool;
use rand::Rng;
use crate::{
    map::tile::Biome,
    utils::cli::Args,
    map::tile::Tile
};

#[derive(Clone)]
pub struct WorldParameters {
    pub sea_level: f64,
    pub peak_h: f64,
    pub mountain_h: f64,
    pub hills_h: f64,
    pub frozen_t: f64,
    pub tundra_t: f64,
    pub boreal_t: f64,
    pub boreal_r: f64,
    pub temperate_t: f64,
    pub temperate_r: f64,
    pub rainforest_t: f64,
    pub rainforest_r: f64,
    pub wetlands_r: f64,
    pub desert_cutoff: f64,
    pub plains_cutoff: f64,
    pub global_heat_scaling: f64,
    pub altitude_scale: f64,
    pub temperature_scale: f64,
    pub rainfall_scale: f64
}

pub struct World {
    pub seeds: [u32; 3],
    pub width: usize,
    pub height: usize,
    pub equator: f64,
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
        let equator = (height / 2) as f64;
        let seeds: [u32; 3] = match &args.seeds {
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
            let world_size = height * width; 
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

        //crate::utils::helpers::generate_rivers(&tiles, &parameters, width, height);

        World { 
            seeds,
            width,
            height,
            equator,
            tiles,
        }
    }
}

