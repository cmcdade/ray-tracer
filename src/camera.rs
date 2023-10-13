use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::{self, degrees_to_radians};
use crate::vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3};

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
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    pub vfov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
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
        self.center = self.lookfrom;

        // distance between camera center and viewport
        //let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = 2.0 * (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width / self.image_height);

        let w = unit_vector(&mut (self.lookfrom - self.lookat));
        let u = unit_vector(&mut cross(self.vup, w));
        let v = cross(w, u);
        // Calculate the vectors across the horizontal and
        // down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        // Calculate the horizontal and
        // vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;
        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::origin();
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, std::f32::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec) {
                attenuation * self.ray_color(scattered, depth - 1, world)
            } else {
                Color::origin()
            }
        } else {
            // linear interpolation (lerp) between white and blue
            // for the background gradient
            let mut unit_direction = unit_vector(&mut r.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc
            + (self.pixel_delta_u * (i as f32))
            + ((j as f32) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let mut p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + utils::random_double();
        let py = -0.5 + utils::random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
