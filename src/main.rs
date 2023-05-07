use clap::Parser;
use rand::Rng;

use crate::world::World;
use crate::image::VisualizationMode;
use crate::cli::Args;

mod cli;
mod helpers;
mod image;
mod noise_sampler;
mod tile;
mod world;

fn main() {
    let args = Args::parse();

    println!("[MapGen] We are attempting to generate {} map(s) in {} x {}.", args.variations, args.x, args.y);
    if args.debug { println!("[MapGen] Running with debug on; logs will be generated"); };
    let mut worlds = Vec::<World>::with_capacity(args.variations as usize);
    let mut rng = rand::thread_rng();
    
    for i in 0..args.variations {
        let seed = if args.seed.is_none() { 
            rng.gen::<u32>()
        } else { 
            args.seed.unwrap()
        };
        println!("[MapGen] Building world no. {} using seed [{}]...", i, &seed);
        worlds.push(World::new(seed, (args.x, args.y)));
    }

    for world in worlds {
        world.save_image(VisualizationMode::Biome, &args.file, args.debug);
    }
}
