use rand::Rng;
use clap::Parser;
use rayon::prelude::*;
use crate::{
    map::world::World,
    utils::cli::Args,
    image::VisualizationMode,
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
        let seed = if args.seed.is_none() { 
            rng.gen::<u32>()
        } else { 
            args.seed.unwrap()
        };

        println!("[MapGen] Building world no. {} using seed [{}]...", i, &seed);
        worlds.push(World::new(seed, args.x, args.y));
    }

    // for world in worlds {
    worlds.par_iter().for_each(|world| {
        // world.save_image(VisualizationMode::Debug, args.file.clone(), args.debug);
        // world.save_image(VisualizationMode::Altitude, args.file.clone(), args.debug);
        world.save_image(VisualizationMode::Biome, args.file.clone(), args.debug);
    });
}
