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

impl World {
    pub fn save_image(&self, mode: VisualizationMode, file_name: &Option<String>, debug: bool) {
        let mut log = String::new();
        let mut img = RgbImage::new(self.width, self.height);

        let file_name: String = match file_name {
            Some(name) => name.to_string(),
            None => self.seed.to_string(),
        };

        for tile in &self.tiles {
            img.put_pixel(tile.x, tile.y, tile.rgb(&mode));
            if debug { log.push_str(&format!("{}\n", tile.altitude)); };
        }

        if debug {
            let log_file: String = format!("./logs/{}.log", &file_name);
            std::fs::write(log_file, log).unwrap();
            println!("[MapGen] Writing log to file {}.log", &file_name);
        }

        println!("[MapGen] Writing image to file {}.png", &file_name);
        _ = image::save_buffer(format!("./images/{}.png", &file_name), &img, self.width, self.height, Rgb8);
    }
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode) -> Rgb<u8> {
        match mode {
            VisualizationMode::Biome => {
                if                             self.altitude <=  98 { Rgb([150, 150, 150]) } // mountains
                else if  98 < self.altitude && self.altitude <= 110 { Rgb([ 25,  75,   0]) } // hills
                else if 110 < self.altitude && self.altitude <= 134 { Rgb([ 50, 100,   0]) } // plains
                else if 134 < self.altitude && self.altitude <= 136 { Rgb([100, 100,   0]) } // beaches
                else                                                { Rgb([  0,   0, 100]) } // water
            },
            VisualizationMode::Altitude => { Rgb([self.altitude, self.altitude, self.altitude]) },
            _ => { Rgb([self.altitude, self.altitude, self.altitude]) },
        }
    }
}

