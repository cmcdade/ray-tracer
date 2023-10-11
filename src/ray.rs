use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin() + t * self.direction()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn new() {
        let origin = vec3::Point3::new(1.0, 2.0, 3.0);
        let dir = vec3::Vec3::new(1.0, 2.0, 3.0);
        let foo = Ray::new(origin, dir);

        assert_eq!(foo.origin(), origin);
        assert_eq!(foo.direction(), dir);
    }

    #[test]
    fn at() {
        let origin = vec3::Point3::new(1.0, 2.0, 3.0);
        let dir = vec3::Vec3::new(1.0, 2.0, 3.0);
        let foo = Ray::new(origin, dir);

        let mut at = foo.at(2.0);

        assert_eq!(at.x(), 3.0);
        assert_eq!(at.y(), 6.0);
        assert_eq!(at.z(), 9.0);
    }
}
