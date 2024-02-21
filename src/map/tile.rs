use noise::NoiseFn;
use super::world::WorldParameters;

use nannou::glam::Vec2;

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
    pub x: f32,
    pub y: f32,
    pub altitude: f32,
    pub temperature: f32,
    pub humidity: f32,
    pub biome: Biome,
}

impl Tile {
    pub fn new(
        id: u32,
        x: f32,
        y: f32,
        noise: &[noise::Fbm<noise::SuperSimplex>; 3],
        parameters: &WorldParameters,
        width: &u32,
        height: &u32
    ) -> Tile {
        let mut altitude = noise[0].get([(x / parameters.altitude_scale) as f64, (y / parameters.altitude_scale) as f64]) as f32;
        let temperature = noise[1].get([(x / parameters.temperature_scale) as f64, (y / parameters.temperature_scale) as f64]) as f32;
        let humidity = noise[2].get([(x / parameters.humidity_scale) as f64, (y / parameters.humidity_scale) as f64]) as f32;
            
        let sea_level = parameters.sea_level;
        let swamp_humidity = parameters.swamp_humidity;
        let desert_humidity = parameters.desert_humidity;
        let hill_altitude = parameters.hill_altitude;
        let mountain_altitude = parameters.mountain_altitude;

        let pos_0 = Vec2::new(0.0, 0.0);
        let center = Vec2::new(
            (width / 2) as f32,
            (height / 2) as f32
        );
        let dist_0 = pos_0.distance(center);
        let position = Vec2::new(x, y);
        let distance_from_center = position.distance(center);
        altitude -= (distance_from_center/dist_0)/5.0;
        
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
