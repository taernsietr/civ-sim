use crate::{
    noise_sampler::NoiseSampler,
    utils::helpers::scale_f64_to_u8
};

#[derive(Debug)]
pub enum Biome {
    Grassland,
    Swamp,
    Desert,
    Coast,
    Sea,
    Hills,
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
    pub fn new(x: u32, y: u32, sampler: &[NoiseSampler; 3]) -> TileBuilder {
        TileBuilder {
            x,
            y,
            altitude: scale_f64_to_u8(sampler[0].get_point_value(x, y, 0)),
            temperature: scale_f64_to_u8(sampler[1].get_point_value(x, y, u32::MAX/2)),
            humidity: scale_f64_to_u8(sampler[2].get_point_value(x, y, u32::MAX))
        }
    }

    fn resolve_biome(&self) -> Biome {
        match (self.altitude, self.temperature, self.humidity) {
            (          255, 255,       255) => Biome::Unset,
            (    192..=255,   _,         _) => Biome::Mountain,
            (    160..=191,   _,         _) => Biome::Hills,
            (     96..=159,   _, 128..=255) => Biome::Swamp,
            (     96..=159,   _,  64..=127) => Biome::Grassland,
            (     96..=159,   _,     ..=63) => Biome::Desert,
            (      80..=95,   _,         _) => Biome::Coast,
            (        ..=79,   _,         _) => Biome::Sea,
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

