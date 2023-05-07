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
    // TODO: enable different types of noise to be used
    pub fn new(x: u32, y: u32, noise_map: &impl NoiseFn<f64, 3>) -> Tile {
        let values = vec!(
            (0.0, Some(0.0), Some(0.0), 0050.0, Some(0050.0), Some(0050.0), 2.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 0100.0, Some(0100.0), Some(0100.0), 2.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 0250.0, Some(0250.0), Some(0250.0), 2.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 0500.0, Some(0500.0), Some(0500.0), 6.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 1000.0, Some(1000.0), Some(1000.0), 6.0, noise_map),
        );
        let samplers = NoiseSampler::build_samplers(values);
        let res = scale_f64_to_u8(NoiseSampler::get_point_value(x, Some(y), None, samplers));

        Tile {
            x, y, biome: None,
            altitude: res,
        }
    }

    // TODO
    #[allow(dead_code)]
    fn calculate_biome(&mut self) {
        self.biome = Some(Biome::Grassland);
    }
}

