use std::sync::{
    Arc,
    mpsc::channel
};
use serde::Deserialize;
use threadpool::ThreadPool;
use rand::{Rng, seq::SliceRandom};
use crate::{
    map::tile::{Tile, Biome},
    utils::{cli::Args, helpers::adjacent},
};

#[derive(Clone, Deserialize)]
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

pub struct WorldBuilder<'a> {
    pub args: &'a Args,
    pub params: &'a WorldParameters,
    pub seeds: [u32; 3],
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub equator: f64,
    pub tiles: Vec<Tile>, 
    pub rivers: Vec<usize>,
}

pub struct World {
    pub seeds: [u32; 3],
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub equator: f64,
    pub tiles: Vec<Tile>, 
    pub rivers: Vec<usize>,
}

impl<'a> From<&mut WorldBuilder<'a>> for World {
    fn from(builder: &mut WorldBuilder<'a>) -> Self {
        World {
            seeds: builder.seeds,
            width: builder.width,
            height: builder.height,
            size: builder.size,
            equator: builder.equator,
            tiles: builder.tiles.clone(),
            rivers: builder.rivers.clone()
        }
    }
}

impl<'a> WorldBuilder<'a> {
    pub fn new(args: &'a Args, params: &'a WorldParameters) -> WorldBuilder<'a> {
        let mut rng = rand::thread_rng();
        let seeds: [u32; 3] = match &args.seeds {
            None => [rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>()],
            Some(j) => [j[0], j[1], j[2]],
        };

        WorldBuilder { 
            args,
            params,
            seeds,
            width: args.x,
            height: args.y,
            size: args.x * args.y,
            equator: (args.y / 2) as f64,
            tiles: Vec::with_capacity(args.x * args.y),
            rivers: Vec::new()
        }
    }

    pub fn build(&mut self) -> World {
        println!("[MapGen] Building world using seeds [{}, {}, {}]", self.seeds[0], self.seeds[1], self.seeds[2]);
        self.generate_tiles()
            .generate_coast()
            //.generate_rivers()
            .into()
    }

    fn generate_tiles(&mut self) -> &mut Self {
        println!("[MapGen] Building tiles.");
        let noise = [
            noise::Fbm::<noise::SuperSimplex>::new(self.seeds[0]),
            noise::Fbm::<noise::SuperSimplex>::new(self.seeds[1]),
            noise::Fbm::<noise::SuperSimplex>::new(self.seeds[2])
        ];

        let noise = Arc::new(noise);
        let parameters = Arc::new(self.params.clone());
        let workers = 32;
        let pool = ThreadPool::new(workers);
        let (tx, rx) = channel::<Tile>();

        let (dim_x, dim_y) = { ((0..self.width), (0..self.height)) };
        let width = self.width;
        let equator = self.equator;
        for x in dim_x {
            for y in dim_y.clone() {
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
        self.tiles = tiles;
        println!("[MapGen] Tiles built.");
        self
    }

    fn generate_coast(&mut self) -> &mut Self {
        println!("[MapGen] Finding coast tiles.");
        let mut coast_tiles = Vec::<usize>::new();
        self.tiles.iter().enumerate().for_each(|(i, tile)| {
            if matches!(&tile.biome, Biome::Sea) {
                adjacent(i, self.width, self.size).iter().for_each(|j| {
                    if !matches!(&self.tiles[*j].biome, Biome::Sea) {
                        coast_tiles.push(*j);
                    };
                });
            };
        });
        coast_tiles.iter().for_each(|t| {
            self.tiles[*t].biome = Biome::Coast;
        });
        println!("[MapGen] Coast processed.");
        self
    }

    fn generate_rivers(&mut self) -> &mut Self {
        todo!();
        // find highest points
        // select n points
        // for every n point, find the next lowest spot L
        // find the shared adjacencies between n and L, ignore them
        // find the next lowest spot for L, repeat until we reach a sea tile
    }
}

