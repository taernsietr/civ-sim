use std::fmt;
use chrono::Local;
use nannou::image::{
    save_buffer, ColorType::Rgb8, Rgb, RgbImage
};
use crate::map::{
    world::World,
    tile::{Tile, Biome}
};
use crate::utils::helpers::scale_f32_to_u8;

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
    EquatorDistance,
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
            VisualizationMode::EquatorDistance=> write!(f, "equator_distance"),
        }
    }
}

pub fn generate_image(world: &World, mode: &VisualizationMode,) -> RgbImage {
    let mut img = RgbImage::new(world.width, world.height);
    for tile in &world.tiles {
        img.put_pixel(tile.x as u32, tile.y as u32, tile.rgb(mode, world));
    }
    println!("[MapGen] Finished building image.");
    img
}

pub fn save_image(
    img: &RgbImage,
    world: &World,
    mode: &VisualizationMode,
    debug: bool
) {
    // TODO: refactor to PathBuf
    let file_name = format!("{}-{}", Local::now().format(DATE_FORMAT), mode);
    
    println!("[MapGen] Writing image to file {}.png", &file_name);
    _ = save_buffer(
        format!("/home/tsrodr/Run/civ-sim/images/{}.png", &file_name),
        img,
        world.width,
        world.height,
        Rgb8
    );

    if debug {
        let mut log = String::from("id,altitude,temperature,humidity\n");
        for tile in &world.tiles {
            log.push_str(&format!("{},{},{},{}\n", tile.id, tile.altitude, tile.temperature, tile.humidity));
        }
        println!("[MapGen] Writing log to file {}.log", &file_name);
        std::fs::write(format!("/home/tsrodr/Run/civ-sim/logs/{}.log", &file_name), log).unwrap();
    }
    println!("[MapGen] Map saved!");
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode, world: &World) -> Rgb<u8> {
        let rgb: [u8; 3] = match mode {
            VisualizationMode::Debug => {
                let color = [
                    scale_f32_to_u8(self.altitude),
                    scale_f32_to_u8(self.humidity),
                    scale_f32_to_u8(self.temperature)
                ];
                [color[0], color[1], color[2]]
            },
            VisualizationMode::Biome => {
                match self.biome {
                    Biome::Coast =>     [  0,  80, 160],
                    Biome::Desert =>    [255, 210, 150],
                    Biome::Forest =>    [  0,  50,   0],
                    Biome::Glacier =>   [255, 255, 255],
                    Biome::Grassland => [ 60, 100,   0],
                    Biome::Hills =>     [150, 120, 100],
                    Biome::Mountain =>  [150, 150, 150],
                    Biome::Peaks =>     [200, 200, 200],
                    Biome::Sea =>       [  0,   0, 100],
                    Biome::Swamp =>     [ 70,  90,  50],
                    Biome::Tundra =>    [160, 180, 140],
                    Biome::Debug =>     [255,   0,   0]
                }
            },
            VisualizationMode::Altitude => {
                let color = scale_f32_to_u8(self.altitude);
                [color, color, color]
            },
            VisualizationMode::Humidity => {
                let color = scale_f32_to_u8(self.humidity);
                [0, 0, color]
            },
            VisualizationMode::Temperature => {
                let color = scale_f32_to_u8(self.temperature);
                [color, 0, 0]
            },
            VisualizationMode::EquatorDistance => {
                let equator = world.height as f32/2.0;
                let distance_to_equator = f32::abs(equator - self.y) / equator;
                let color = scale_f32_to_u8(distance_to_equator);
                if color == 0 { println!("tamo vivo"); }
                [0, color, 0]
            },
            _ => unreachable!()
        };

        Rgb([rgb[0],rgb[1],rgb[2]])
    }
}

