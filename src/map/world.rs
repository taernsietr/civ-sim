use std::sync::Arc;
use crate::map::tile::Tile;

pub struct WorldParameters {
    pub sea_level: f64,
    pub grassland_threshold: f64,
    pub swamp_threshold: f64,
    pub desert_threshold: f64,
    pub hill_threshold: f64,
    pub mountain_threshold: f64,
}

pub struct World {
    pub seeds: [u32; 3],
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>, 
}

impl World {
    pub fn new(
        seeds: [u32; 3],
        width: u32,
        height: u32,
        parameters: &WorldParameters,
    ) -> World {
        let size = (width * height) as usize;
        let mut tiles = Vec::with_capacity(size);

        let noise = [
            noise::Fbm::<noise::OpenSimplex>::new(seeds[0]),
            noise::Fbm::<noise::OpenSimplex>::new(seeds[1]),
            noise::Fbm::<noise::OpenSimplex>::new(seeds[2])
        ];

        let noise = Arc::new(noise);

//        for x in 0..width {
//            for y in 0..height {
//                tiles.push(Tile::new(x as f64, y as f64, &noise, parameters));
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
                        part_1.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2 {
                    for y in 0..height/4 {
                        part_2.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in 0..height/4 {
                        part_3.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in 0..height/4 {
                        part_4.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in height/4..height/2 {
                        part_5.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2 {
                    for y in height/4..height/2 {
                        part_6.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in height/4..height/2 {
                        part_7.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in height/4..height/2 {
                        part_8.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in height/2..(3*height/4) {
                        part_9.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2 {
                    for y in height/2..(3*height/4) {
                        part_10.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in height/2..(3*height/4) {
                        part_11.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in height/2..(3*height/4) {
                        part_12.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/4 {
                    for y in (3*height/4)..height {
                        part_13.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/4..width/2 {
                    for y in (3*height/4)..height {
                        part_14.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..(3*width/4) {
                    for y in (3*height/4)..height {
                        part_15.push(Tile::new(x as f64, y as f64, &noise, parameters));
                    }
                }
            });
            s.spawn(|| {
                for x in (3*width/4)..width {
                    for y in (3*height/4)..height {
                        part_16.push(Tile::new(x as f64, y as f64, &noise, parameters));
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
            seeds,
            width,
            height,
            tiles,
        }
    }
}

