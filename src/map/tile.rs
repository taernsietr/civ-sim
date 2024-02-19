use noise::NoiseFn;
use super::world::WorldParameters;

const ALTITUDE_SCALE: f64 = 1000.0;
const TEMPERATURE_SCALE: f64 = 500.0;
const HUMIDITY_SCALE: f64 = 500.0;

#[derive(Debug)]
pub enum Biome {
    Grassland,
    Swamp,
    Desert,
    Sea,
    Hills,
    Mountain,
    Coast,
}

#[derive(Debug)]
pub struct Tile {
    pub x: f64,
    pub y: f64,
    pub altitude: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub biome: Biome,
}

impl Tile {
    pub fn new(
        x: f64,
        y: f64,
        noise: &[noise::Fbm<noise::OpenSimplex>; 3],
        parameters: &WorldParameters
    ) -> Tile {
        let altitude = noise[0].get([x / ALTITUDE_SCALE, y / ALTITUDE_SCALE]);
        let temperature = noise[1].get([x / TEMPERATURE_SCALE, y / TEMPERATURE_SCALE]);
        let humidity = noise[2].get([x / HUMIDITY_SCALE, y / HUMIDITY_SCALE]);
            
        let biome = {
            if altitude <= parameters.sea_level { Biome::Sea }
            else if parameters.swamp_threshold >= humidity && humidity > parameters.grassland_threshold { Biome::Swamp }
            else if parameters.grassland_threshold >= humidity && humidity > parameters.desert_threshold { Biome::Grassland}
            else if humidity <= parameters.desert_threshold { Biome::Desert }
            else if parameters.mountain_threshold > altitude && altitude >= parameters.hill_threshold { Biome::Hills }
            else if altitude >= parameters.mountain_threshold { Biome::Mountain }
            else { Biome::Coast }
        };
            
        Tile {
            x,
            y,
            altitude,
            temperature,
            humidity,
            biome
        }
    }
}

