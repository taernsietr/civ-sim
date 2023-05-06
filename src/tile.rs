use noise::NoiseFn;

use crate::noise_sampler::NoiseSampler;
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
    // pub temperature: u8,
    // pub humidity: u8,
    // pub vegetation: u8,
    // pub hardness: u8,
    // pub sunlight: u8,
    biome: Option<Biome>,
}

impl Tile {
    pub fn new<'a>(x: u32, y: u32, noise_map: &'a impl NoiseFn<f64, 3>) -> Tile {
        let values = vec!(
            (0.0f64, Some(0.0f64), Some(0.0f64), 100.0f64, Some(100.0f64), Some(100.0f64), 1.0f64, noise_map),
        );
        let samplers = NoiseSampler::build_samplers(values);

        let res = scale_f64_to_u8(NoiseSampler::get_point_value(x, Some(y), None, samplers));

        let mut tile = Tile {
            x, y, biome: None,
            altitude: res,
            // hardness: 127,
            // temperature: 127,
            // humidity: 127,
            // vegetation: 127,
            // sunlight: 127,
        };
        tile.calculate_biome();
        tile
    }

    // todo!
    fn calculate_biome(&mut self) {
        self.biome = Some(Biome::Grassland);
    }

    fn sample_noise(x: u32, y: u32, xoff: f64, yoff: f64, scale: f64) -> [f64; 2] {
        [
            ((x as f64 + xoff) / scale),
            ((y as f64 + yoff) / scale)
        ]
    }
}

