use rand::Rng;
use clap::Parser;
use rayon::prelude::*;
use crate::{
    map::world::World,
    utils::cli::Args,
    image::{save_image, VisualizationMode}
};

pub mod utils;
pub mod map;
pub mod image;
pub mod noise_sampler;

fn main() {
    let args = Args::parse();

    if args.debug { println!("[MapGen] Running with debug on; logs will be generated"); };
    println!(
        "[MapGen] We are attempting to generate {} map(s) in {} x {}.",
        args.variations,
        args.x,
        args.y
    );

    let mut worlds = Vec::<World>::with_capacity(args.variations as usize);
    let mut rng = rand::thread_rng();
    
    for i in 0..args.variations {
        let seeds = if args.seeds.is_none() { 
            [
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>()
            ]
        } else { 
            [
                args.seeds.unwrap(),
                args.seeds.unwrap() + 1,
                args.seeds.unwrap() + 2
            ]
        };

        println!("[MapGen] Building world no. {} using given seed(s)...", i);
        worlds.push(World::new(seeds, args.x, args.y));
    }

    worlds.par_iter().for_each(|world| {
        // save_image(world, VisualizationMode::Debug, args.file.clone(), args.debug);
        save_image(world, VisualizationMode::Altitude, args.file.clone(), args.debug);
        save_image(world, VisualizationMode::Temperature, args.file.clone(), args.debug);
        save_image(world, VisualizationMode::Humidity, args.file.clone(), args.debug);
        save_image(world, VisualizationMode::Biome, args.file.clone(), args.debug);
    });
}
