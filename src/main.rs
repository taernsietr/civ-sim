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

    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let parameters = WorldCreationParameters { dimensions: (args.x, args.y) };
    let world = World::new(parameters);
    
    for tile in &world.tiles {
        println!("({},{}): {}", tile.x, tile.y, tile.altitude);
    }

    rgb_image(world, VisualizationMode::Altitude, Some(args.file));
}
