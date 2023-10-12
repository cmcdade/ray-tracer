mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    let mut world: HittableList = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400.0;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
