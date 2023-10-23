// Scales a f64 within [-1.0, 1.0] to a u8 within [0, 255]
pub fn scale_f64_to_u8(input: f64) -> u8 {
    (((input + 1.0) / 2.0) * 255.0) as u8
}

