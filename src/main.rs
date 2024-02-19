use clap::Parser;
use rayon::prelude::*;
use crate::{
    image::{save_image, VisualizationMode}, utils::{cli::Args, helpers::generate_worlds}
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

    let worlds = generate_worlds(&args);

    worlds.par_iter().for_each(|world| {
        // save_image(world, VisualizationMode::Debug, args.file.clone(), args.debug);
        save_image(world, VisualizationMode::Altitude, args.file.clone(), args.debug);
        // save_image(world, VisualizationMode::Temperature, args.file.clone(), args.debug);
        // save_image(world, VisualizationMode::Humidity, args.file.clone(), args.debug);
        save_image(world, VisualizationMode::Biome, args.file.clone(), args.debug);
    });
}
