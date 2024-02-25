use nannou::math::map_range;
use nannou::math::num_traits::Pow;

/// Scales a f64 within [-1.0, 1.0] to a u8 within [0, 255]
/// No error handling!
pub fn scale_f64_to_u8(input: f64) -> u8 {
    map_range(input, -1.0, 1.0, 0.0, 255.0) as u8
}

/// scales a temperature input in [-1.0,1.0] based on a global scaling factor
/// global_scaling should ideally range [0,2]
pub fn adjust_temperature(t: &f64, equator: &f64, y: &f64, global_scaling: &f64) -> f64 {
    let latitude = f64::abs(equator - y) / equator;
    ((1.0f64 / (1.0f64 + (latitude * (-latitude * *t).exp()))) *
        ((1.0f64 - (2.0f64 * latitude.pow(2.0f64)) + *t) / 2.0f64) -
        (0.72f64 * *global_scaling)
    ).clamp(-1.0f64, 1.0f64)
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
