use std::fs::File;
use image::{RgbImage, Rgb};
use image::ColorType::Rgb8;

use crate::world::World;
use crate::tile::Tile;

#[allow(dead_code)]
pub enum VisualizationMode {
    Biome,
    Altitude,
    Temperature,
    Humidity,
    Vegetation,
    Hardness,
    Sunlight,
}

pub fn rgb_image(world: &World, mode: VisualizationMode, file_name: String, debug: bool) {
    let mut log_file = File::create(format!("{}.log", file_name));
    let mut img = RgbImage::new(world.width, world.height);
    // let mut img = Rgb32FImage::new(world.width, world.height);

    for tile in &world.tiles {
        img.put_pixel(tile.x, tile.y, tile.rgb(&mode));
        if debug {
            let r = tile.rgb(&mode).0[0];
            file.write!("{} {} {}", r, g, b)
        };
    }
    _ = image::save_buffer(&file_name, &img, world.width, world.height, Rgb8);
    // _ = image::save_buffer(file_name, &img, world.width, world.height, Rgb8);
    println!("[MapGen] Writing image to file {}", &file_name);
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode) -> Rgb<u8> {
        match mode {
            VisualizationMode::Biome => {
                if                              self.altitude <= 98 { Rgb([150, 150, 150]) }   // mountains
                else if 98 < self.altitude && self.altitude <= 109 { Rgb([255, 255, 204]) }     // hills
                else if 109 < self.altitude && self.altitude <= 130 { Rgb([50, 100, 0]) }   // plains
                else if 130 < self.altitude && self.altitude <= 135 { Rgb([100, 100, 0]) }  // beaches
                else                                                 { Rgb([0, 0, 100]) }   // water
            },
            VisualizationMode::Altitude => { Rgb([self.altitude, self.altitude, self.altitude]) },
            _ => { Rgb([self.altitude, self.altitude, self.altitude]) },
        }
    }
}
