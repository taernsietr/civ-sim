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
    let mut log = String::new();
    let mut img = RgbImage::new(world.width, world.height);

    for tile in &world.tiles {
        img.put_pixel(tile.x, tile.y, tile.rgb(&mode));
        if debug { log.push_str(&format!("{}\n", tile.altitude)); };
    }

    println!("[MapGen] Writing image to file {}.png", &file_name);
    _ = image::save_buffer(format!("./images/{}.png", file_name), &img, world.width, world.height, Rgb8);

    if debug {
        let log_file: String = format!("./logs/{}.log", file_name);
        std::fs::write(log_file, log).unwrap();
        println!("[MapGen] Writing log to file {}.log", &file_name);
    }
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode) -> Rgb<u8> {
        match mode {
            VisualizationMode::Biome => {
                if                             self.altitude <= 098 { Rgb([150, 150, 150]) } // mountains
                else if 098 < self.altitude && self.altitude <= 112 { Rgb([025, 075, 000]) } // hills
                else if 112 < self.altitude && self.altitude <= 127 { Rgb([050, 100, 000]) } // plains
                else if 127 < self.altitude && self.altitude <= 130 { Rgb([100, 100, 000]) } // beaches
                else                                                { Rgb([000, 000, 100]) } // water
            },
            VisualizationMode::Altitude => { Rgb([self.altitude, self.altitude, self.altitude]) },
            _ => { Rgb([self.altitude, self.altitude, self.altitude]) },
        }
    }
}

