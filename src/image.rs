// use std::path::PathBuf;
use std::fmt;
use chrono::Local;
use image::{
    RgbImage,
    Rgb,
    ColorType::Rgb8
};
use crate::map::{
    world::World,
    tile::{Tile, Biome}
};

const DATE_FORMAT: &str = "%y%m%d-%Hh%M";

#[allow(dead_code)]
pub enum VisualizationMode {
    Biome,
    Altitude,
    Temperature,
    Humidity,
    Vegetation,
    Hardness,
    Sunlight,
    Debug,
}

impl fmt::Display for VisualizationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VisualizationMode::Biome => write!(f, "biome"),
            VisualizationMode::Altitude => write!(f, "altitude"),
            VisualizationMode::Temperature => write!(f, "temperature"),
            VisualizationMode::Humidity => write!(f, "humidity"),
            VisualizationMode::Vegetation => write!(f, "vegetation"),
            VisualizationMode::Hardness => write!(f, "hardness"),
            VisualizationMode::Sunlight => write!(f, "sunlight"),
            VisualizationMode::Debug=> write!(f, "debug"),
        }
    }
}

impl World {
    pub fn save_image(
        &self,
        mode: VisualizationMode,
        file_name: Option<String>,
        debug: bool
    ) -> () {
        let mut log = String::from("altitude,temperature,humidity\n");
        let mut img = RgbImage::new(self.width, self.height);

        // TODO: refactor to use PathBuf and if-let syntax ?
        let file_name: String = match file_name {
            Some(name) => format!("{}-{}-{}", Local::now().format(DATE_FORMAT), name, mode),
            None => format!("{}-{}-{}", Local::now().format(DATE_FORMAT), self.seed, mode),
        };
        
        for tile in &self.tiles {
            img.put_pixel(tile.x, tile.y, tile.rgb(&mode));
            if debug { log.push_str(&format!("{},{},{}\n", tile.altitude, tile.temperature, tile.humidity)); };
        }

        if debug {
            let log_file: String = format!("/home/tsrodr/Run/civ-sim/logs/{}.log", &file_name);
            std::fs::write(log_file, log).unwrap();
            println!("[MapGen] Writing log to file {}.log", &file_name);
        }

        println!("[MapGen] Writing image to file {}.png", &file_name);
        _ = image::save_buffer(
            format!("/home/tsrodr/Run/civ-sim/images/{}.png", &file_name),
            &img,
            self.width,
            self.height,
            Rgb8
        );
    }
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode) -> Rgb<u8> {
        match mode {
            VisualizationMode::Altitude => {
                if                             self.altitude <=  98 { Rgb([150, 150, 150]) } // mountains
                else if  98 < self.altitude && self.altitude <= 110 { Rgb([ 25,  75,   0]) } // hills
                else if 110 < self.altitude && self.altitude <= 134 { Rgb([ 50, 100,   0]) } // plains
                else if 134 < self.altitude && self.altitude <= 136 { Rgb([100, 100,   0]) } // beaches
                else                                                { Rgb([  0,   0, 100]) } // water
            },
            VisualizationMode::Biome => {
                match self.biome {
                    Biome::Grassland => Rgb([ 50, 100,   0]),
                    Biome::Swamp =>     Rgb([ 30, 120,  30]),
                    Biome::Coast =>     Rgb([100, 100,  30]),
                    Biome::Hills =>     Rgb([ 92,  51,  23]),
                    Biome::Desert =>    Rgb([100, 100,   0]),
                    Biome::Sea =>       Rgb([  0,   0, 100]),
                    Biome::Mountain =>  Rgb([150, 150, 150]),
                    Biome::Unset =>     Rgb([255,   0,   0]),
                }
            },
            VisualizationMode::Debug => {
                Rgb([self.altitude, self.temperature, self.humidity])
            }
            _ => { Rgb([self.altitude, self.altitude, self.altitude]) },
        }
    }
}

