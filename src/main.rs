use clap::Parser;

use crate::world::{WorldCreationParameters, World};
use crate::image::{VisualizationMode, rgb_image};

mod tile;
mod world;
mod helpers;
mod image;

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
}

fn main() {
    let args = Args::parse();
    let parameters = WorldCreationParameters { dimensions: (args.x, args.y) };
    let file_name: String = match args.file {
        Some(name) => name,
        None => "test".to_string(),
    };
    
    let mut worlds = Vec::<World>::with_capacity(args.variations as usize);
    for _ in 0..args.variations {
        worlds.push(World::new(&parameters));
    }

    for (i, world) in worlds.iter().enumerate() {
        rgb_image(world, VisualizationMode::Altitude, format!("./{}_{}.png", file_name, i));
    }
}
