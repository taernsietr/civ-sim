use std::fmt;
use chrono::Local;
use nannou::{glam::Vec2, image::{
    save_buffer, ColorType::Rgb8, Rgb, RgbImage
}};
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

pub fn shape_continent(world: &mut World) {
    println!("[MapGen] Shaping continent...");


    let pos_0 = Vec2::new(0.0, 0.0);
    let center = Vec2::new((world.width / 2) as f32, (world.height / 2) as f32);
    let dist_0 = pos_0.distance(center) as f64;
    dbg!(&dist_0);
    for tile in world.tiles.iter_mut() {
        let position = Vec2::new(tile.x as f32, tile.y as f32);
        let distance_from_center = position.distance(center) as f64;
        tile.altitude *= distance_from_center / dist_0 * 5.0;
    };
    println!("[MapGen] Finished processing continent.");
}

pub fn create_coast(world: &World, image: &mut RgbImage) {
    println!("[MapGen] Processing coast...");
    let width = world.width as usize;
    let world_size = world.height as usize * width; 

    for (i, tile) in world.tiles.iter().enumerate() {
        if matches!(&tile.biome, Biome::Sea) {
            let indices = 
                if i == 0                                { vec!(i+1, i+width)               }  // first tile 
                else if i == width - 1                   { vec!(i-1, i+width)               }  // last tile of first row
                else if i == world_size - 1              { vec!(i-1, i-width)               }  // last tile
                else if i == world_size - width          { vec!(i+1, i-width)               }  // first tile of last row
                else if i % width == 0                   { vec!(i+1, i-width, i+width)      }  // first tile of row
                else if i % width == width - 1           { vec!(i-1, i-width, i+width)      }  // last tile of row
                else if i < width                        { vec!(i-1, i+1, i+width)          }  // first row
                else if i > world_size - width           { vec!(i-1, i+1, i-width)          }  // last row
                else                                     { vec!(i-1, i+1, i-width, i+width) }; // elsewhere
                for j in &indices {
                    if !matches!(world.tiles[*j].biome, Biome::Sea) {
                        image.put_pixel(tile.x as u32, tile.y as u32, Rgb([0,80,160]));
                    };
                }
        };
    }
    println!("[MapGen] Finished processing coast.");
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
                let color = [
                    scale_f64_to_u8(self.altitude),
                    scale_f64_to_u8(self.humidity),
                    scale_f64_to_u8(self.temperature)
                ];
                [color[0], color[1], color[2]]
            },
            VisualizationMode::Biome => {
                match self.biome {
                    Biome::Grassland => [ 50, 100,  60],
                    Biome::Swamp =>     [102, 102,   0],
                    Biome::Coast =>     [ 10,  70, 120],
                    Biome::Hills =>     [ 84,  81,  75],
                    Biome::Desert =>    [255, 200, 150],
                    Biome::Sea =>       [  0,   0, 100],
                    Biome::Mountain =>  [128, 128, 128],
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

