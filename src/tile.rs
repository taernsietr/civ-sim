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
    pub fn new<'a>(x: u32, y: u32, noise_map: &'a impl NoiseFn<f64, 3>) -> Tile {
        let values = vec!(
            (0.0, Some(0.0), Some(0.0), 00010.0, Some(00010.0), Some(00010.0), 2.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 00100.0, Some(00100.0), Some(00100.0), 4.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 01000.0, Some(01000.0), Some(01000.0), 8.0, noise_map),
            (0.0, Some(0.0), Some(0.0), 10000.0, Some(10000.0), Some(10000.0), 24.0, noise_map),
        );
        let samplers = NoiseSampler::build_samplers(values);
        let res = scale_f64_to_u8(NoiseSampler::get_point_value(x, Some(y), None, samplers));
        let tile = Tile {
            x, y, biome: None,
            altitude: res,
            // hardness: 127,
            // temperature: 127,
            // humidity: 127,
            // vegetation: 127,
            // sunlight: 127,
        };
        // tile.calculate_biome();
        tile
    }

    // todo!
    #[allow(dead_code)]
    fn calculate_biome(&mut self) {
        self.biome = Some(Biome::Grassland);
    }
}

