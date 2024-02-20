use rand::Rng;
use super::cli::Args;
use crate::map::world::{World, WorldParameters};

// Scales a f64 within [-1.0, 1.0] to a u8 within [0, 255]
// No error handling!
pub fn scale_f64_to_u8(input: f64) -> u8 {
    (((input + 1.0) / 2.0) * 255.0) as u8
}

pub fn scale_f64_to_u16(input: f64) -> u16 {
    (((input + 1.0) / 2.0) * 65536.0) as u16
}

pub fn generate_worlds(args: &Args) -> Vec<World> {
    let mut rng = rand::thread_rng();
    let mut worlds = Vec::<World>::with_capacity(args.variations as usize);

    let parameters = WorldParameters {
        sea_level: 0.0,
        grassland_threshold: 0.1,
        swamp_threshold: 0.5,
        desert_threshold: 0.0,
        hill_threshold: 0.4,
        mountain_threshold: 0.5,
    };
    
    for i in 0..args.variations {
        let seeds = match &args.seeds {
                None => [rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>()],
                Some(j) if j.len() == 1 => [j[0], rng.gen::<u32>(), rng.gen::<u32>()],
                Some(j) if j.len() == 2 => [j[0], j[1], rng.gen::<u32>()],
                Some(j) if j.len() >= 3 => [j[0], j[1], j[2]],
                _ => unreachable!()
        };
        println!("[MapGen] Building world no. {} using seeds [{}, {}, {}]", i, seeds[0], seeds[1], seeds[2]);
        worlds.push(World::new(seeds, args.x, args.y, parameters.clone()));
    }
    worlds
}
