use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

pub trait Hittable: Sync {
    fn hit(&self, r: Ray, interval: Interval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            p: Point3::origin(),
            normal: Vec3::origin(),
            t: 0.0,
            front_face: false,
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}
