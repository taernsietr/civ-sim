use nannou::math::num_traits::Pow;
use noise::NoiseFn;
use super::world::WorldParameters;

#[derive(Debug)]
pub enum Biome {
    Coast,
    Desert,
    Forest,
    Grassland,
    Hills,
    Glacier,
    Mountain,
    Peaks,
    Sea,
    Swamp,
    Tundra,
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
        params: &WorldParameters,
        _width: &u32,
        _height: &u32
    ) -> Tile {
        let h = noise[0].get([(x / params.altitude_scale) as f64, (y / params.altitude_scale) as f64]) as f32;
        let t = noise[1].get([(x / params.temperature_scale) as f64, (y / params.temperature_scale) as f64]) as f32;
        let w = noise[2].get([(x / params.humidity_scale) as f64, (y / params.humidity_scale) as f64]) as f32;

        let biome = {
            if      h >= params.peak_height                                                                  { Biome::Peaks }
            else if h >= params.mountain_height                                                              { Biome::Mountain }
            else if h >= params.hills_height                                                                 { Biome::Hills }
            else if t <  params.glacier_temp                                                                 { Biome::Glacier }
            else if h <= params.sea_level                                                                    { Biome::Sea }
            else if (h.pow(2.0) + t.pow(2.0) + w.pow(2.0)) <= params.grassland_threshold                     { Biome::Grassland }
            else if (h.pow(2.0) + t.pow(2.0)) == w && t > params.tundra_low_t && t < params.tundra_high_t    { Biome::Tundra }
            else if (h.pow(2.0) + t.pow(2.0))/2.0 + w.pow(2.0) <= params.forest_threshold && w >= 0.0        { Biome::Forest }
            else if (h.pow(2.0) + t.pow(2.0))/2.0 + (4.0 * w.pow(2.0)) <= params.swamp_threshold && w >= 0.0 { Biome::Swamp }
            else                                                                                             { Biome::Desert }
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
