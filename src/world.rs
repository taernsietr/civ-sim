use crate::tile::Tile;

pub struct World {
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    // pub rotation_angle: u8,
    pub tiles: Vec<Tile>, 
}

impl World {
    pub fn new(seed: u32, dimensions: (u32, u32)) -> World {
        // TODO: add seed branch
        let (width, height) = dimensions;
        let size = (width * height) as usize;
        let mut tiles = Vec::with_capacity(size);
        let noise = noise::OpenSimplex::new(seed);

        // TODO: Write this properly, maybe scaling with available CPU cores?
        let mut part1 = Vec::with_capacity(size/4);
        let mut part2 = Vec::with_capacity(size/4);
        let mut part3 = Vec::with_capacity(size/4);
        let mut part4 = Vec::with_capacity(size/4);

        std::thread::scope(|s| { 
            s.spawn(|| {
                for x in 0..width/2 {
                    for y in 0..height/2 {
                        let tile = Tile::new(x, y, &noise);
                        part1.push(tile);
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..width {
                    for y in 0..height/2 {
                        let tile = Tile::new(x, y, &noise);
                        part2.push(tile);
                    }
                }
            });
            s.spawn(|| {
                for x in 0..width/2 {
                    for y in height/2..height {
                        let tile = Tile::new(x, y, &noise);
                        part3.push(tile);
                    }
                }
            });
            s.spawn(|| {
                for x in width/2..width {
                    for y in height/2..height {
                        let tile = Tile::new(x, y, &noise);
                        part4.push(tile);
                    }
                }
            });
        });

        tiles.append(&mut part1);
        tiles.append(&mut part2);
        tiles.append(&mut part3);
        tiles.append(&mut part4);

        World { 
            seed,
            width,
            height,
            // rotation_angle: 0,
            tiles,
        }
        
    }
}

