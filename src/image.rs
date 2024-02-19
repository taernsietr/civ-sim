// use std::path::PathBuf;
use std::fmt;
use chrono::Local;
use nannou::image::{
    RgbImage,
    Rgb,
    ColorType::Rgb8,
    save_buffer
};
use crate::map::{
    world::World,
    tile::{Tile, Biome}
};
use crate::utils::helpers::scale_f64_to_u8;

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

pub fn generate_image(
    world: &World,
    mode: VisualizationMode,
) -> RgbImage {
    let mut img = RgbImage::new(world.width, world.height);
    for tile in &world.tiles {
        img.put_pixel(tile.x as u32, tile.y as u32, tile.rgb(&mode));
    }
    println!("[MapGen] Finished building image.");
    img
}

pub fn save_image(
    world: &World,
    mode: VisualizationMode,
    file_name: Option<String>,
    debug: bool
) {
    let mut log = String::from("altitude,temperature,humidity\n");
    let mut img = RgbImage::new(world.width, world.height);

    // TODO: refactor to PathBuf
    let file_name: String = match file_name {
        Some(name) => format!("{}-{}-{}", Local::now().format(DATE_FORMAT), name, mode),
        None => format!("{}-{}-{}", Local::now().format(DATE_FORMAT), world.seeds[0], mode), // change later
    };
    
    for tile in &world.tiles {
        img.put_pixel(tile.x as u32, tile.y as u32, tile.rgb(&mode));
        if debug { log.push_str(&format!("{},{},{}\n", tile.altitude, tile.temperature, tile.humidity)); };
    }

    if debug {
        let log_file: String = format!("/home/tsrodr/Run/civ-sim/logs/{}.log", &file_name);
        std::fs::write(log_file, log).unwrap();
        println!("[MapGen] Writing log to file {}.log", &file_name);
    }

    println!("[MapGen] Writing image to file {}.png", &file_name);
    _ = save_buffer(
        format!("/home/tsrodr/Run/civ-sim/images/{}.png", &file_name),
        &img,
        world.width,
        world.height,
        Rgb8
    );
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode) -> Rgb<u8> {
        let rgb: [u8; 3] = match mode {
            VisualizationMode::Debug => {
                let color = [scale_f64_to_u8(self.altitude), scale_f64_to_u8(self.humidity), scale_f64_to_u8(self.temperature)];
                [color[0], color[1], color[2]]
            },
            VisualizationMode::Biome => {
                match self.biome {
                    Biome::Grassland => [  0, 153,   0],
                    Biome::Swamp =>     [ 75, 100,   0],
                    // Biome::Coast =>     [100, 100,  30],
                    Biome::Coast =>     [  0,   0,   0],
                    Biome::Hills =>     [ 96,  96,  64],
                    Biome::Desert =>    [255, 204, 153],
                    Biome::Sea =>       [  0,   0, 100],
                    Biome::Mountain =>  [128, 128, 196],
                }
            },
            VisualizationMode::Altitude => {
                let color = scale_f64_to_u8(self.altitude);
                [color, color, color]
            },
            VisualizationMode::Humidity => {
                let color = scale_f64_to_u8(self.humidity);
                [0, 0, color]
            },
            VisualizationMode::Temperature=> {
                let color = scale_f64_to_u8(self.temperature);
                [color, 0, 0]
            },
            _ => unreachable!()
        };

        Rgb([rgb[0],rgb[1],rgb[2]])
    }
}

