use noise::NoiseFn;
use crate::map::world::WorldParameters;

#[derive(Debug, Clone)]
pub enum Biome {
    Boreal,
    Coast,
    Desert,
    Frozen,
    Hill,
    Mountain,
    Peak,
    Plains,
    Sea,
    Rainforest,
    Temperate,
    Tundra,
    Wetland,
    Debug,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub altitude: f64,
    pub temperature: f64,
    pub rainfall: f64,
    pub biome: Biome,
}

impl Tile {
    pub fn new(
        id: usize,
        x: f64,
        y: f64,
        equator: &f64,
        noise: &[noise::Fbm<noise::SuperSimplex>; 4],
        params: &WorldParameters,
    ) -> Tile {
        let temperature: f64 = {
            ((-(f64::abs(equator - y) / equator) * 8.0) * params.global_heat_scaling +
            noise[0].get([x / params.temperature_scale, y / params.temperature_scale]) * 2.0)
            / 10.0
        };

        let altitude: f64 = {
            let w = x / params.altitude_scale;
            let z = y / params.altitude_scale;
            let x = x / (params.altitude_scale * 0.5);
            let y = y / (params.altitude_scale * 0.5);

            let a = noise[1].get([w, z]);
            let b = noise[1].get([w + 0.003, z + 0.002]);
            let c = noise[1].get([w + 1.2 * a, z + 1.2 * b]) + noise[1].get([x, y]);
            c / 2.0
        };

        let rainfall: f64 = {
            let a: f64 = noise[2].get([x / params.rainfall_scale, y / params.rainfall_scale]);
            let b: f64 = -(7.0 * (f64::abs(equator - y) / equator)).cos();
            let c: f64 = f64::abs(temperature);
            (a + b + c) / 3.0
        };

        let biome = {
            if      altitude    >= params.peak_h                                          { Biome::Peak }
            else if altitude    >= params.mountain_h                                      { Biome::Mountain }
            else if altitude    >= params.hills_h                                         { Biome::Hill }
            else if altitude    <= params.sea_level                                       { Biome::Sea }
            else if temperature <= params.frozen_t                                        { Biome::Frozen }
            else if temperature <= params.tundra_t                                        { Biome::Tundra }
            else if temperature <= params.boreal_t && rainfall >= params.boreal_r         { Biome::Boreal }
            else if rainfall    >= params.wetlands_r                                      { Biome::Wetland }
            else if temperature >= params.rainforest_t && rainfall >= params.rainforest_r { Biome::Rainforest }
            else if temperature <= params.temperate_t && rainfall >= params.temperate_r   { Biome::Temperate }
            else if rainfall    <= params.desert_r                                        { Biome::Desert }
            else if temperature + rainfall <= params.plains_cutoff                        { Biome::Plains }
            else                                                                          { Biome::Debug }
        };

        Tile {
            id,
            x,
            y,
            altitude,
            temperature,
            rainfall,
            biome
        }
    }

    pub fn is_sea(&self) -> bool { matches!(self.biome, Biome::Sea) }
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
