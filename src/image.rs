use std::fmt;
use chrono::Local;
use nannou::image::{
    save_buffer, Rgba, ColorType::Rgb8, RgbaImage
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
    Rainfall,
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
            VisualizationMode::Rainfall => write!(f, "rainfall"),
            VisualizationMode::Vegetation => write!(f, "vegetation"),
            VisualizationMode::Hardness => write!(f, "hardness"),
            VisualizationMode::Sunlight => write!(f, "sunlight"),
            VisualizationMode::Debug=> write!(f, "debug"),
            VisualizationMode::EquatorDistance=> write!(f, "equator_distance"),
        }
    }
}

pub fn generate_image(
    world: &World,
    rivers: &[usize],
    mode: &VisualizationMode
) -> RgbaImage {
    let mut img = RgbaImage::new(world.width as u32, world.height as u32);

    for tile in &world.tiles {
        img.put_pixel(tile.x as u32, tile.y as u32, tile.rgb(mode, world));
    }

    rivers.iter().for_each(|river| {
        img.put_pixel(world.tiles[*river].x as u32, world.tiles[*river].y as u32, Rgba([255,0,0,255]));
    });

    println!("[MapGen] Finished building image.");
    img
}

pub fn save_image(
    img: &RgbaImage,
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
        world.width as u32,
        world.height as u32,
        Rgb8
    );

    if debug {
        let mut log = String::from("id,altitude,temperature,rainfall\n");
        for tile in &world.tiles {
            log.push_str(&format!("{},{},{},{}\n", tile.id, tile.altitude, tile.temperature, tile.rainfall));
        }
        println!("[MapGen] Writing log to file {}.log", &file_name);
        std::fs::write(format!("/home/tsrodr/Run/civ-sim/logs/{}.log", &file_name), log).unwrap();
    }
    println!("[MapGen] Map saved!");
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode, world: &World) -> Rgba<u8> {
        let alpha: u8 = scale_f64_to_u8(self.altitude);
        let rgb: [u8; 3] = match mode {
            VisualizationMode::Debug => {
                let color = [
                    scale_f64_to_u8(self.altitude),
                    scale_f64_to_u8(self.rainfall),
                    scale_f64_to_u8(self.temperature)
                ];
                [color[0], color[1], color[2]]
            },
            VisualizationMode::Biome => {
                match self.biome {
                    Biome::Frozen =>     [255, 255, 255],
                    Biome::Tundra =>     [140, 180, 160],
                    Biome::Boreal=>      [120, 150, 130],
                    Biome::Temperate =>  [ 20,  50,  20],
                    Biome::Rainforest => [  0, 100,   0],
                    Biome::Wetland =>    [ 40,  60,  20],
                    Biome::Plains =>     [ 60,  80,  20],
                    Biome::Desert =>     [220, 210, 150],
                    Biome::Hill =>       [150, 150, 115],
                    Biome::Mountain =>   [150, 150, 150],
                    Biome::Peak =>       [200, 200, 200],
                    Biome::Sea =>        [  0,   0, 100],
                    Biome::Coast =>      [  0,  80, 160],
                    Biome::Debug =>      [255,   0,   0]
                }
            },
            VisualizationMode::Altitude => {
                let color = scale_f64_to_u8(self.altitude);
                [color, color, color]
            },
            VisualizationMode::Rainfall => {
                let color = scale_f64_to_u8(self.rainfall);
                [0, 0, color]
            },
            VisualizationMode::Temperature => {
                let color = scale_f64_to_u8(self.temperature);
                [color, 0, 0]
            },
            VisualizationMode::EquatorDistance => {
                let equator = world.height as f64/2.0;
                let distance_to_equator = f64::abs(equator - self.y) / equator;
                let color = scale_f64_to_u8(-distance_to_equator);
                [0, color, 0]
            },
            _ => unreachable!()
        };

        Rgba([rgb[0],rgb[1],rgb[2],alpha])
    }
}

