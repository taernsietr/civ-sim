use nannou::math::num_traits::Pow;
use noise::NoiseFn;
use crate::utils::helpers::adjust_temperature;
use super::world::WorldParameters;

#[derive(Debug)]
pub enum Biome {
    Coast,
    ColdDesert,
    ColdForest,
    Desert,
    Forest,
    Glacier,
    Grassland,
    Hill,
    Mountain,
    Peak,
    Sea,
    Wetland,
    Tundra,
    Debug,
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
        equator: &f32,
        noise: &[noise::Fbm<noise::SuperSimplex>; 3],
        params: &WorldParameters,
    ) -> Tile {
        let h = (noise[0].get([(x / params.altitude_scale) as f64, (y / params.altitude_scale) as f64]) as f32).clamp(-1.0, 1.0);
        let mut t = (noise[1].get([(x / params.temperature_scale) as f64, (y / params.temperature_scale) as f64]) as f32).clamp(-1.0, 1.0);
        let w = (noise[2].get([(x / params.humidity_scale) as f64, (y / params.humidity_scale) as f64]) as f32).clamp(-1.0, 1.0);

        adjust_temperature(&mut t, equator, &y, &params.global_heat_scaling);

        let biome = {
            if      h >= params.peak_height                                         { Biome::Peak }
            else if h >= params.mountain_height                                     { Biome::Mountain }
            else if h >= params.hills_height                                        { Biome::Hill }
            else if h <= params.sea_level                                           { Biome::Sea }
            else if t <  params.glacier_temp                                        { Biome::Glacier }
            else if w >= params.wetlands_humidity                                   { Biome::Wetland }
            else if w <  params.desert_humidity && t <= params.cold_desert_temp     { Biome::ColdDesert }
            else if w <  params.desert_humidity                                     { Biome::Desert }
            else if t <  params.grassland_high_t && t >= params.grassland_low_t     { Biome::Grassland }
            else if t <  params.tundra_high_t && t >= params.tundra_low_t           { Biome::Tundra }
            else if t <  params.forest_high_t && t >= params.forest_low_t           { Biome::Forest }
            else if t <  params.cold_forest_high_t && t >= params.cold_forest_low_t { Biome::ColdForest }
            else                                                                    { Biome::Debug }
            //else if t >  params.tundra_low_t && t <= params.tundra_high_t && (w2 + t2) < h { Biome::Tundra }
            //else if (h2 + t2 + w2) <= params.grassland_threshold                           { Biome::Grassland }
            //else if w >= 0.0 && (h2 + t2)/2.0 + (4.0 * w2) <= params.swamp_threshold       { Biome::Swamp }
            //else if w >= 0.0 && (h2 + t2)/2.0 + w2 <= params.forest_threshold              { Biome::Forest }
        };

        Tile {
            id,
            x,
            y,
            altitude: h,
            temperature: t,
            humidity: w,
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
