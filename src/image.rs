use std::{
    str::FromStr,
    path::PathBuf,
    fmt::{Formatter, Result, Display}
};
use chrono::Local;
use nannou::image::{
    save_buffer, Rgba, ColorType::Rgb8, RgbaImage, DynamicImage
};
use crate::{
    map::{
        world::World,
        tile::{Tile, Biome}
    },
    utils::helpers::scale_f64_to_u8
};

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

impl Display for VisualizationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl World {
    pub fn generate_image(&self, mode: &VisualizationMode) -> DynamicImage {
        let mut img = RgbaImage::new(self.width as u32, self.height as u32);

        for tile in &self.tiles {
            img.put_pixel(tile.x as u32, tile.y as u32, tile.rgb(mode, self));
        }

        self.rivers.iter().for_each(|river| {
            img.put_pixel(self.tiles[*river].x as u32, self.tiles[*river].y as u32, Rgba([255,0,0,255]));
        });

        println!("[MapGen] Finished building image.");
        DynamicImage::ImageRgba8(img)
    }

    pub fn save_image(
        &self,
        mode: &VisualizationMode,
        debug: bool
    ) {
        let (imagefile, logfile) = {
            let file = PathBuf::from_str("/home/tsrodr/Run/civ-sim/").expect("[MapGen] Could not find project folder.");
            let file_name = format!("{}-{}", Local::now().format(DATE_FORMAT), mode);
            let mut imagefile = file.join("images/");
            let mut logfile = file.join("logs/");
            imagefile.set_file_name(&file_name);
            imagefile.set_extension("png");
            logfile.set_file_name(&file_name);
            logfile.set_extension("log");

            (imagefile, logfile)
        };
        
        println!("[MapGen] Writing image to file {}", &imagefile.display());
        _ = save_buffer(
            imagefile,
            &self.generate_image(mode).to_rgba8(),
            self.width as u32,
            self.height as u32,
            Rgb8
        );

        if debug {
            let mut log = String::from("id,altitude,temperature,rainfall\n");
            for tile in &self.tiles {
                log.push_str(&format!("{},{},{},{}\n", tile.id, tile.altitude, tile.temperature, tile.rainfall));
            }
            println!("[MapGen] Writing log to file {}", &logfile.display());
            std::fs::write(logfile, log).unwrap();
        }
        println!("[MapGen] Map saved!");
    }
}

impl Tile {
    pub fn rgb(&self, mode: &VisualizationMode, world: &World) -> Rgba<u8> {
        let rgb: [u8; 4] = match mode {
            VisualizationMode::Debug => {
                let color = [
                    scale_f64_to_u8(self.altitude),
                    scale_f64_to_u8(self.rainfall),
                    scale_f64_to_u8(self.temperature),
                    255
                ];
                [color[0], color[1], color[2], color[3]]
            },
            VisualizationMode::Biome => {
                let alpha: u8 = scale_f64_to_u8(self.altitude);
                match self.biome {
                    Biome::Frozen =>     [255, 255, 255, alpha],
                    Biome::Tundra =>     [140, 140, 150, alpha],
                    Biome::Boreal=>      [130, 130, 140, alpha],
                    Biome::Temperate =>  [105, 135,  55, alpha],
                    Biome::Rainforest => [ 65, 130,  40, alpha],
                    Biome::Wetland =>    [ 50,  50,  30, alpha],
                    Biome::Plains =>     [ 90,  85,  40, alpha],
                    Biome::Desert =>     [165, 140,  85, alpha],
                    Biome::Hill =>       [130, 120, 125, alpha],
                    Biome::Mountain =>   [140, 145, 145, alpha],
                    Biome::Peak =>       [215, 215, 215, alpha],
                    Biome::Coast =>      [ 30,  75, 220, alpha],
                    Biome::Sea =>        [ 25,  25, 200, alpha],
                    Biome::Debug =>      [255,   0,   0, alpha]
                }
            },
            VisualizationMode::Altitude => {
                let color = scale_f64_to_u8(self.altitude);
                if self.altitude < 0.0  { [0, 0, color, 255] }
                else { [color, color, color, 255] }
            },
            VisualizationMode::Rainfall => {
                let color = scale_f64_to_u8(self.rainfall);
                [0, 0, color, 255]
            },
            VisualizationMode::Temperature => {
                let color = scale_f64_to_u8(self.temperature);
                [color, 0, 0, 255]
            },
            VisualizationMode::EquatorDistance => {
                let equator = world.height as f64/2.0;
                let distance_to_equator = f64::abs(equator - self.y) / equator;
                let color = scale_f64_to_u8(-distance_to_equator);
                [color, color, color, 255]
            },
            _ => unreachable!()
        };

        Rgba([rgb[0],rgb[1],rgb[2],rgb[3]])
    }
}

