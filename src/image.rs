use image::{RgbImage, Rgb, Rgb32FImage};
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

pub fn rgb_image(world: World, mode: VisualizationMode, name: Option<String>) {
    let file_name: String = match name {
        Some(name) => format!("./{}.png", name),
        None => "./test.png".to_string(),
    };

    let mut img = RgbImage::new(world.width, world.height);
    // let mut img = Rgb32FImage::new(world.width, world.height);

    for tile in &world.tiles {
        img.put_pixel(tile.x, tile.y, tile.rgb(&mode))
    }
    _ = image::save_buffer(file_name, &img, world.width, world.height, Rgb8);
    // _ = image::save_buffer(file_name, &img, world.width, world.height, Rgb8);
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode) -> Rgb<u8> {
        match mode {
            /*
            VisualizationMode::Biome => {
                if                              self.altitude <= 100 { Rgb([32, 32, 255]) }
                else if 100 <  self.altitude && self.altitude <= 150 { Rgb([255, 80, 00]) }
                else if 150 <  self.altitude && self.altitude <= 180 { Rgb([128, 255, 128]) }
                else if 180 <  self.altitude && self.altitude <= 200 { Rgb([32, 255, 32]) }
                else                                                 { Rgb([255, 255, 255]) }
            },
            */
            VisualizationMode::Altitude => { Rgb([self.altitude, self.altitude, self.altitude]) },
            _ => { Rgb([self.altitude, self.altitude, self.altitude]) },
        }
    }
}
