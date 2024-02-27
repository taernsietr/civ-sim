use std::sync::{
    Arc,
    mpsc::channel
};
use threadpool::ThreadPool;
use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use crate::{
    map::tile::Biome,
    utils::cli::Args,
    utils::helpers::{adjacent, is_map_edge},
    map::tile::Tile
};

#[derive(Clone, Serialize, Deserialize)]
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
    pub river_factor: f64, // cannot be 0, lower increases rivers
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
                    adjacent(i, width, world_size).iter().for_each(|j| {
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

    pub fn generate_rivers(&self,
        params: &WorldParameters
    ) -> Vec<usize> {
        // find highest points
        // select n points
        // for every n point, find the next lowest spot L
        // find the shared adjacencies between n and L, ignore them
        // find the next lowest spot for L, repeat until we reach a sea tile
        let mut rng = rand::thread_rng();
        let world_size = self.height * self.width; 
        let points = &self.tiles.iter().filter(|pt| pt.altitude >= params.mountain_h).map(|pt| pt.id).collect::<Vec<usize>>();
        let points = points.choose_multiple(&mut rng, (self.height + self.width) / (100.0 * params.river_factor) as usize);
        let mut rivers: Vec<usize> = Vec::new();
        
        points.into_iter().for_each(|pt| {
            let previous = vec!(*pt);
            rivers.append(&mut Self::river(&self.tiles, params, self.width, world_size, &previous, &adjacent(*pt, self.width, world_size)));
        });
        rivers
    }

    fn river(
        tiles: &Vec<Tile>,
        params: &WorldParameters,
        width: usize,
        world_size: usize,
        previous: &[usize],
        _adjacencies: &[usize]
    ) -> Vec<usize> {
//      let mut rng = rand::thread_rng();
        let tile = *previous.last().unwrap();
        let adjacent_tiles: Vec<usize> = adjacent(tile, width, world_size);
        let land_adjacent: Vec<&usize> = adjacent_tiles.iter().filter(|i| {
            tiles[**i].altitude >= params.sea_level
        }).collect();

        // either the next lowest tile or the current tile if none is found
        let lowest_adjacent: usize = land_adjacent
            .iter()
            .fold(tile, |curr, j| {
            if tiles[**j].altitude < tiles[curr].altitude { **j }
            else { curr }
        });

        let mut current = previous.to_vec();

        // base case: river reached coast, end path
        if lowest_adjacent == tile && matches!(tiles[lowest_adjacent].biome, Biome::Coast) || is_map_edge(lowest_adjacent, width, world_size)
            { current }

        // case 2: continue to lowest adjacent tile
        else {
            current.push(lowest_adjacent);
            Self::river(tiles, params, width, world_size, &current, &adjacent(lowest_adjacent, width, world_size))
        }

        // case 3: no adjacent tiles are lower than previous, choose at random
//      else {
//          let selected = adjacent_tiles.choose(&mut rng).unwrap();
//          current.push(*selected);
//          Self::river(tiles, params, width, world_size, &current, &adjacent(*selected, width, world_size))
//      }
    }
}

