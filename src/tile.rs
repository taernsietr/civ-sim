use noise::NoiseFn;

use crate::helpers::scale_f64_to_u8;

#[derive(Debug)]
enum Biome {
    Grassland,
}

#[derive(Debug)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub altitude: u8,
    pub temperature: u8,
    pub humidity: u8,
    pub vegetation: u8,
    pub hardness: u8,
    pub sunlight: u8,
    biome: Option<Biome>,
}

impl Tile {
    pub fn new(x: u32, y: u32, noise: impl NoiseFn<f64, 2>) -> Tile {
        let mut tile = Tile {
            x, y, biome: None,
            altitude: scale_f64_to_u8(noise.get(Tile::sample_noise(x, y, 0.0, 1000.0))),
            hardness: 127,
            temperature: 127,
            humidity: 127,
            vegetation: 127,
            sunlight: 127,
        };
        tile.calculate_biome();
        tile
    }

    // todo!
    fn calculate_biome(&mut self) {
        self.biome = Some(Biome::Grassland);
    }

    fn sample_noise(x: u32, y: u32, offset: f64, scale: f64) -> [f64; 2] {
        [
            (x as f64 + offset / scale),
            (y as f64 + offset / scale)
        ]
    }
}

