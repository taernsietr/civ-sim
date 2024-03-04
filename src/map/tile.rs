use noise::NoiseFn;
use crate::{
    utils::helpers::adjust_temperature,
    map::world::WorldParameters
};

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
        let t: f64 = noise[2].get([x / params.temperature_scale, y / params.temperature_scale]);
        let r: f64 = noise[3].get([x / params.rainfall_scale, y / params.rainfall_scale]);
        let t = adjust_temperature(t, equator, &y, &params.global_heat_scaling);

        let h: f64 = {
            let w = x / params.altitude_scale;
            let z = y / params.altitude_scale;
            let x = x / (params.altitude_scale * 0.5);
            let y = y / (params.altitude_scale * 0.5);
            let a = noise[0].get([w, z]);
            let b = noise[0].get([w + 0.003, z + 0.002]);
            let c = noise[0].get([w + 1.2 * a, z + 1.2 * b]) +
            noise[1].get([x, y]) +
            noise[1].get([w, z]);
            c / 3.0
        };

        let biome = {
            if      h >= params.peak_h                                   { Biome::Peak }
            else if h >= params.mountain_h                               { Biome::Mountain }
            else if h >= params.hills_h                                  { Biome::Hill }
            else if h <= params.sea_level                                { Biome::Sea }
            else if t <= params.frozen_t                                 { Biome::Frozen }
            else if t <= params.tundra_t                                 { Biome::Tundra }
            else if t <= params.boreal_t && r >= params.boreal_r         { Biome::Boreal }
            else if r >= params.wetlands_r                               { Biome::Wetland }
            else if t >= params.rainforest_t && r >= params.rainforest_r { Biome::Rainforest }
            else if t <= params.temperate_t && r >= params.temperate_r   { Biome::Temperate }
            else if t + r <= params.plains_cutoff                        { Biome::Plains }
            else if t + r <= params.desert_cutoff                        { Biome::Desert }
            else                                                         { Biome::Debug }
        };

        Tile {
            id,
            x,
            y,
            altitude: h,
            temperature: t,
            rainfall: r,
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
