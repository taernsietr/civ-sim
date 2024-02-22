use nannou::math::num_traits::Pow;
use noise::NoiseFn;
use super::world::WorldParameters;

#[derive(Debug)]
pub enum Biome {
    Coast,
    Desert,
    Forest,
    Glacier,
    Grassland,
    Hills,
    Mountain,
    Peaks,
    Sea,
    Swamp,
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
        noise: &[noise::Fbm<noise::SuperSimplex>; 3],
        params: &WorldParameters,
        _width: &u32,
        height: &u32
    ) -> Tile {
        let h = (noise[0].get([(x / params.altitude_scale) as f64, (y / params.altitude_scale) as f64]) as f32).clamp(-1.0, 1.0);
        //let t = (noise[1].get([(x / params.temperature_scale) as f64, (y / params.temperature_scale) as f64]) as f32).clamp(-1.0, 1.0);
        let w = (noise[2].get([(x / params.humidity_scale) as f64, (y / params.humidity_scale) as f64]) as f32).clamp(-1.0, 1.0);

        let equator = *height as f32 / 2.0;
        let distance_to_equator = f32::abs(equator - y) / equator;
        let t = -distance_to_equator;

        let h2 = h.pow(2.0);
        let t2 = t.pow(2.0);
        let w2 = w.pow(2.0);

        let biome = {
            if      h >= params.peak_height                                                { Biome::Peaks }
            else if h >= params.mountain_height                                            { Biome::Mountain }
            else if h >= params.hills_height                                               { Biome::Hills }
            else if h <= params.sea_level                                                  { Biome::Sea }
            else if t <  params.glacier_temp                                               { Biome::Glacier }
            else if t >= params.tundra_low_t && t <= params.tundra_high_t && (w2 + t2) < h { Biome::Tundra }
            else if (h2 + t2 + w2) <= params.grassland_threshold                           { Biome::Grassland }
            else if w >= 0.0 && (h2 + t2)/2.0 + w2 <= params.forest_threshold              { Biome::Forest }
            else if w >= 0.0 && (h2 + t2)/2.0 + (4.0 * w2) <= params.swamp_threshold       { Biome::Swamp }
            else                                                                           { Biome::Desert }
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
