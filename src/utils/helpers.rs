// Scales a f32 within [-1.0, 1.0] to a u8 within [0, 255]
// No error handling!
pub fn scale_f32_to_u8(input: f32) -> u8 {
    (((input + 1.0) / 2.0) * 255.0) as u8
}

