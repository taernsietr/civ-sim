use clap::Parser;

use crate::world::{WorldCreationParameters, World};
use crate::image::{VisualizationMode, rgb_image};

mod tile;
mod world;
mod helpers;
mod image;
mod noise_sampler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 256)]
    x: u32,

    #[arg(short, long, default_value_t = 256)]
    y: u32,

    #[arg(short, long, default_value_t = 1)]
    variations: u32,
    
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[arg(short, long)]
    seed: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let parameters = WorldCreationParameters { dimensions: (args.x, args.y), seed: Some(37263) };
    let file_name: String = match args.file {
        Some(name) => name,
        None => "test".to_string(),
    };

    if args.debug { println!("[MapGen] Running with debug on; logs will be generated"); };
    println!("[MapGen] We are attempting to generate {} image(s) in {}x{}.", args.variations, args.x, args.y);
    
    let mut worlds = Vec::<World>::with_capacity(args.variations as usize);
    for i in 0..args.variations {
        println!("[MapGen] Building world no. {}...", i);
        worlds.push(World::new(&parameters));
    }

    for (i, world) in worlds.iter().enumerate() {
        rgb_image(world, VisualizationMode::Biome, format!("{}_{}", file_name, i), args.debug);
    }
}
