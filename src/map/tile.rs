use crate::{
    noise_sampler::NoiseSampler,
    helpers::scale_f64_to_u8
};

#[derive(Debug)]
pub enum Biome {
    Grassland,
    Desert,
    Sea,
    Mountain,
    Unset,
}

#[derive(Debug)]
pub struct TileBuilder {
    pub x: u32,
    pub y: u32,
    pub altitude: u8,
    pub temperature: u8,
    pub humidity: u8,
    //pub vegetation: u8,
    //pub hardness: u8,
    //pub sunlight: u8,
}

impl TileBuilder {
    // TODO: enable different types of noise to be used
    pub fn new(x: u32, y: u32, sampler: &NoiseSampler) -> TileBuilder {
        TileBuilder {
            x,
            y,
            altitude: scale_f64_to_u8(sampler.get_point_value(x, y, 0)),
            temperature: scale_f64_to_u8(sampler.get_point_value(x, y, 5000)),
            humidity: scale_f64_to_u8(sampler.get_point_value(x, y, 10000))
        }
    }

    fn resolve_biome(&self) -> Biome {
        match (self.altitude, self.temperature, self.humidity) {
            (128..,      _,    _) => Biome::Mountain,
            (64..=128, 64.., ..=63) => Biome::Grassland,
            (64..=128, 64.., 64..) => Biome::Desert,
            (..=63,       _,    _) => Biome::Sea,
            (_,           _,    _) => Biome::Unset,
        }
    }

    pub fn build(self) -> Tile {
        let biome: Biome = self.resolve_biome();
        Tile {
            x: self.x,
            y: self.y,
            altitude: self.altitude,
            temperature: self.temperature,
            humidity: self.humidity,
            //vegetation: self.vegetation,
            //hardness: self.hardness,
            //sunlight: self.sunlight,
            biome
        }
    }
    
}

#[derive(Debug)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub altitude: u8,
    pub temperature: u8,
    pub humidity: u8,
    //pub vegetation: u8,
    //pub hardness: u8,
    //pub sunlight: u8,
    pub biome: Biome,
}

