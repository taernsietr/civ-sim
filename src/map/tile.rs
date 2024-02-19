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
        _parameters: &WorldParameters
    ) -> Tile {
        let altitude = noise[0].get([x / ALTITUDE_SCALE, y / ALTITUDE_SCALE]);
        let temperature = noise[1].get([x / TEMPERATURE_SCALE, y / TEMPERATURE_SCALE]);
        let humidity = noise[2].get([x / HUMIDITY_SCALE, y / HUMIDITY_SCALE]);
            
        let sea_level = 0.0;
        let swamp_humidity = 0.6;
        let desert_humidity = 0.0;
        let hill_altitude = 0.3;
        let mountain_altitude = 0.5;

        let biome = {
            if altitude <= sea_level { Biome::Sea }
            else if mountain_altitude > altitude && altitude >= hill_altitude { Biome::Hills }
            else if altitude >= mountain_altitude { Biome::Mountain }
            else if humidity <= desert_humidity { Biome::Desert }
            else if swamp_humidity < humidity && humidity > desert_humidity { Biome::Grassland}
            else if swamp_humidity >= humidity { Biome::Swamp }
            else { dbg!(&altitude, &temperature, &humidity); Biome::Coast }
        };

        /*
           altitude = 0.12957644594934173
           temperature = -0.5813534064277605
           humidity = 0.5008365702790374
        */
            
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

