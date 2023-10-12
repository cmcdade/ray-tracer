use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;

        if !ray_t.surrounds(root) {
            return None;
        }

        let p = r.at(root);
        let rec = HitRecord {
            t: root,p, front_face: false, normal: (p - self.center) / self.radius,
        };

        Some(rec)
    }
}
