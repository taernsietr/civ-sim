use nannou::math::map_range;
use nannou::math::num_traits::Pow;

/// Scales a f32 within [-1.0, 1.0] to a u8 within [0, 255]
/// No error handling!
pub fn scale_f32_to_u8(input: f32) -> u8 {
    map_range(input, -1.0, 1.0, 0.0, 255.0) as u8
}

/// scales a temperature input in [-1.0,1.0] based on a global scaling factor
/// global_scaling should ideally range [0,2]
pub fn adjust_temperature(t: &mut f32, equator: &f32, y: &f32, global_scaling: &f32) {
    let latitude = f32::abs(equator - y) / equator;
    *t = (
        (1.0 / (1.0 + (latitude * (-latitude * *t).exp()))) *
        ((1.0 - (2.0 * latitude.pow(2.0)) + *t) / 2.0) -
        (0.72 * *global_scaling)
    ).clamp(-1.0, 1.0);
}
