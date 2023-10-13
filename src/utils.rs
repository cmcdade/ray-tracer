pub const PI: f32 = 3.14;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    rand::random::<f32>()
}

pub fn random_double_bounded(min: f32, max: f32) -> f32 {
    min + (max - min) * random_double()
}
