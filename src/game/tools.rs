pub fn tune(base: u32, addition_percentage: i32, addend: i32) -> u32 {
    (base as f32 * (1.0 + addition_percentage as f32 / 100.0) + addend as f32 + 0.5).floor() as u32
}