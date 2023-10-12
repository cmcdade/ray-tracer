use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    linear_component.sqrt()
}

pub fn write_color(pixel: &mut Color, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f32);

    let mut r = pixel.x() * scale;
    let mut g = pixel.y() * scale;
    let mut b = pixel.z() * scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);

    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32,
    );
}
