use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel: &mut Color, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f32);

    let r = pixel.x() * scale;
    let g = pixel.y() * scale;
    let b = pixel.z() * scale;

    let intensity = Interval::new(0.000, 0.999);

    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32,
    );
}
