use std::{path::PathBuf, str::FromStr};
use nannou::math::map_range;
use crate::WorldParameters;

/// Scales a f64 within [-1.0, 1.0] to a u8 within [0, 255]
/// No error handling!
pub fn scale_f64_to_u8(input: f64) -> u8 {
    map_range(input, -1.0, 1.0, 0.0, 255.0) as u8
}

pub fn adjacent(i: usize, width: usize, world_size: usize) -> Vec<usize> {
    if i == 0                                { vec!(i+1, i+width)               } // first tile 
    else if i == width - 1                   { vec!(i-1, i+width)               } // last tile of first row
    else if i == world_size - 1              { vec!(i-1, i-width)               } // last tile
    else if i == world_size - width          { vec!(i+1, i-width)               } // first tile of last row
    else if i % width == 0                   { vec!(i+1, i-width, i+width)      } // first tile of row
    else if i % width == width - 1           { vec!(i-1, i-width, i+width)      } // last tile of row
    else if i < width                        { vec!(i-1, i+1, i+width)          } // first row
    else if i > world_size - width           { vec!(i-1, i+1, i-width)          } // last row
    else                                     { vec!(i-1, i+1, i-width, i+width) } // elsewhere
}

pub fn xy_to_index(tile: &crate::map::tile::Tile, width: usize) -> usize {
    (tile.x as usize) + ((tile.y as usize) * width)
}

pub fn index_to_xy(index: usize, width: usize) -> (f32, f32) {
    let y = (index as f32 / width as f32).floor();
    let x = (index % width) as f32;
    (x, y)
}

pub fn load_parameters() -> WorldParameters {
    let file = PathBuf::from_str("/home/tsrodr/Run/civ-sim/src/parameters.json")
        .expect("[MapGen] Failed to load json file.");
    let data = std::fs::read_to_string(file)
        .expect("[MapGen] Failed to load parameters.");
    serde_json::from_str::<WorldParameters>(&data)
        .expect("[MapGen] Failed to parse parameters.")
}
