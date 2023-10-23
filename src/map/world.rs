use std::sync::Arc;
use crate::{
    tile::{TileBuilder, Tile},
    noise_sampler::NoiseSampler
};

pub struct World {
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    // pub rotation_angle: u8,
    pub tiles: Vec<Tile>, 
}

impl World {
    pub fn new(seed: u32, width:: u32, height: u32) -> World {
        // TODO: add seed branch
        let size = (width * height) as usize;
        let mut tiles = Vec::with_capacity(size);
        let noise = noise::OpenSimplex::new(seed);

        let mut sampler = NoiseSampler::new(noise)
            .add_values([0.0, 0.0, 0.0, 0050.0, 0050.0, 0050.0, 2.0])
            .add_values([0.0, 0.0, 0.0, 0100.0, 0100.0, 0100.0, 2.0])
            .add_values([0.0, 0.0, 0.0, 0250.0, 0250.0, 0250.0, 2.0])
            .add_values([0.0, 0.0, 0.0, 0500.0, 0500.0, 0500.0, 6.0])
            .add_values([0.0, 0.0, 0.0, 1000.0, 1000.0, 1000.0, 6.0]);
        let sampler = Arc::new(sampler);

//        for x in 0..width {
//            for y in 0..height {
//                let tile_builder = TileBuilder::new(x, y, &sampler);
//                tiles.push(tile_builder.build());
//            }
//        }

        // TODO: Write this properly, maybe scaling with available CPU cores?
        // Rayon?
        let mut part_1 = Vec::with_capacity(size/16);
        let mut part_2 = Vec::with_capacity(size/16);
        let mut part_3 = Vec::with_capacity(size/16);
        let mut part_4 = Vec::with_capacity(size/16);
        let mut part_5 = Vec::with_capacity(size/16);
        let mut part_6 = Vec::with_capacity(size/16);
        let mut part_7 = Vec::with_capacity(size/16);
        let mut part_8 = Vec::with_capacity(size/16);
        let mut part_9 = Vec::with_capacity(size/16);
        let mut part_10 = Vec::with_capacity(size/16);
        let mut part_11 = Vec::with_capacity(size/16);
        let mut part_12 = Vec::with_capacity(size/16);
        let mut part_13 = Vec::with_capacity(size/16);
        let mut part_14 = Vec::with_capacity(size/16);
        let mut part_15 = Vec::with_capacity(size/16);
        let mut part_16 = Vec::with_capacity(size/16);

        std::thread::scope(|s| { 
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in 0..height/4 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_1.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2) {
                    for y in 0..height/4 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_2.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in 0..height/4 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_3.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in 0..height/4 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_4.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in height/4..height/2 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_5.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2) {
                    for y in height/4..height/2 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_6.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in height/4..height/2 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_7.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in height/4..height/2 {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_8.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in height/2..(3*height/4) {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_9.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2) {
                    for y in height/2..(3*height/4) {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_10.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in height/2..(3*height/4) {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_11.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in height/2..(3*height/4) {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_12.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in (3*height/4)..height {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_13.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2) {
                    for y in (3*height/4)..height {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_14.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in (3*height/4)..height {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_15.push(tile.build());
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in (3*height/4)..height {
                        let tile = TileBuilder::new(x, y, &sampler);
                        part_16.push(tile.build());
                    }
                }
            });
        });

        tiles.append(&mut part_1);
        tiles.append(&mut part_2);
        tiles.append(&mut part_3);
        tiles.append(&mut part_4);
        tiles.append(&mut part_5);
        tiles.append(&mut part_6);
        tiles.append(&mut part_7);
        tiles.append(&mut part_8);
        tiles.append(&mut part_9);
        tiles.append(&mut part_10);
        tiles.append(&mut part_11);
        tiles.append(&mut part_12);
        tiles.append(&mut part_13);
        tiles.append(&mut part_14);
        tiles.append(&mut part_15);
        tiles.append(&mut part_16);

        World { 
            seed,
            width,
            height,
            // rotation_angle: 0,
            tiles,
        }
    }
}

