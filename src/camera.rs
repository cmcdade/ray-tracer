use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{random_unit_vector, unit_vector, Point3, Vec3};

#[derive(Default)]
pub struct Camera {
    // image width / image height
    pub aspect_ratio: f32,
    pub image_width: f32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: f32,
    // point where all rays originate from, eye point
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for row in 0..self.image_height as i32 {
            for col in 0..self.image_width as i32 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(col, row);
                    pixel_color += self.ray_color(r, self.max_depth, world);
                }
                write_color(&mut pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn initialize(&mut self) {
        self.image_height = self.image_width / self.aspect_ratio;
        self.center = Point3::origin();
        // distance between camera center and viewport
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width / self.image_height);
        // Calculate the vectors across the horizontal and
        // down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        // Calculate the horizontal and
        // vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;
        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut temp_record: HitRecord = HitRecord::default();
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(r, Interval::new(0.001, std::f32::INFINITY), &mut temp_record) {
            let direction = temp_record.normal + random_unit_vector();
            return 0.1 * self.ray_color(Ray::new(temp_record.p, direction), depth-1, world);
        }
        // linear interpolation (lerp) between white and blue
        // for the background gradient
        let mut unit_direction = unit_vector(&mut r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (i as f32)) + ((j as f32) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + utils::random_double();
        let py = -0.5 + utils::random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
