use noise::NoiseFn;
use super::world::WorldParameters;

const ALTITUDE_SCALE: f64 = 1200.0;
const TEMPERATURE_SCALE: f64 = 700.0;
const HUMIDITY_SCALE: f64 = 800.0;

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
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub altitude: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub biome: Biome,
}

impl Tile {
    pub fn new(
        id: u32,
        x: f64,
        y: f64,
        noise: &[noise::Fbm<noise::SuperSimplex>; 3],
        parameters: &WorldParameters
    ) -> Tile {
        let altitude = noise[0].get([x / ALTITUDE_SCALE, y / ALTITUDE_SCALE]);
        let temperature = noise[1].get([x / TEMPERATURE_SCALE, y / TEMPERATURE_SCALE]);
        let humidity = noise[2].get([x / HUMIDITY_SCALE, y / HUMIDITY_SCALE]);
            
        let sea_level = parameters.sea_level;
        let swamp_humidity = parameters.swamp_humidity;
        let desert_humidity = parameters.desert_humidity;
        let hill_altitude = parameters.hill_altitude;
        let mountain_altitude = parameters.mountain_altitude;

        let biome = {
            if altitude <= sea_level { Biome::Sea }
            else if mountain_altitude > altitude && altitude >= hill_altitude { Biome::Hills }
            else if altitude >= mountain_altitude { Biome::Mountain }
            else if humidity <= desert_humidity { Biome::Desert }
            else if swamp_humidity < humidity && humidity > desert_humidity { Biome::Grassland}
            else if swamp_humidity >= humidity { Biome::Swamp }
            else { Biome::Coast }
        };

        Tile {
            id,
            x,
            y,
            altitude,
            temperature,
            humidity,
            biome
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
