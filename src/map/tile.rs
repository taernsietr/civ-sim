use noise::NoiseFn;
use crate::utils::helpers::adjust_temperature;
use super::world::WorldParameters;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Tile {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub altitude: f32,
    pub temperature: f32,
    pub rainfall: f32,
    pub biome: Biome,
}

impl Tile {
    pub fn new(
        id: u32,
        x: f32,
        y: f32,
        equator: &f32,
        noise: &[noise::Fbm<noise::SuperSimplex>; 3],
        params: &WorldParameters,
    ) -> Tile {
        let h = (noise[0].get([(x / params.altitude_scale) as f64, (y / params.altitude_scale) as f64]) as f32).clamp(-1.0, 1.0);
        let mut t = (noise[1].get([(x / params.temperature_scale) as f64, (y / params.temperature_scale) as f64]) as f32).clamp(-1.0, 1.0);
        let r = (noise[2].get([(x / params.rainfall_scale) as f64, (y / params.rainfall_scale) as f64]) as f32).clamp(-1.0, 1.0);

        adjust_temperature(&mut t, equator, &y, &params.global_heat_scaling);

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
