use std::fmt;
use std::ops;

use crate::utils::{random_double, random_double_bounded};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    points: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { points: [x, y, z] }
    }

    pub fn origin() -> Vec3 {
        Vec3 {
            points: [0.0, 0.0, 0.0],
        }
    }

    pub fn x(&mut self) -> f32 {
        self.points[0]
    }

    pub fn y(&mut self) -> f32 {
        self.points[1]
    }

    pub fn z(&mut self) -> f32 {
        self.points[2]
    }

    pub fn length_squared(&mut self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&mut self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_bounded(min: f32, max: f32) -> Vec3 {
        Vec3::new(random_double_bounded(min, max), random_double_bounded(min, max), random_double_bounded(min, max))
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.points[0] * v.points[0] + u.points[1] * v.points[1] + u.points[2] * v.points[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        points: [
            u.points[1] * v.points[2] - u.points[2] * v.points[1],
            u.points[2] * v.points[0] - u.points[0] * v.points[2],
            u.points[0] * v.points[1] - u.points[1] * v.points[0],
        ],
    }
}

pub fn unit_vector(u: &mut Vec3) -> Vec3 {
    let length = u.length();
    Vec3 {
        points: [
            u.points[0] / length,
            u.points[1] / length,
            u.points[2] / length,
        ],
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let mut p = Vec3::random_bounded(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&mut random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub type Point3 = Vec3;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.points[0], self.points[1], self.points[2]
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            points: [
                self.points[0] + other.points[0],
                self.points[1] + other.points[1],
                self.points[2] + other.points[2],
            ],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            points: [
                self.points[0] - other.points[0],
                self.points[1] - other.points[1],
                self.points[2] - other.points[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            points: [
                self.points[0] * other.points[0],
                self.points[1] * other.points[1],
                self.points[2] * other.points[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            points: [self.points[0] * t, self.points[1] * t, self.points[2] * t],
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            points: [self * v.points[0], self * v.points[1], self * v.points[2]],
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            points: [
                self.points[0] / other.points[0],
                self.points[1] / other.points[1],
                self.points[2] / other.points[2],
            ],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3 {
            points: [self.points[0] / t, self.points[1] / t, self.points[2] / t],
        }
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, t: Vec3) -> Vec3 {
        Vec3 {
            points: [t.points[0] / self, t.points[1] / self, t.points[2] / self],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            points: [-self.points[0], -self.points[1], -self.points[2]],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.points[0] += other.points[0];
        self.points[1] += other.points[1];
        self.points[2] += other.points[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.points[0] *= t;
        self.points[1] *= t;
        self.points[2] *= t;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.points[0] /= t;
        self.points[1] /= t;
        self.points[2] /= t;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.points[0] == other.points[0]
            && self.points[1] == other.points[1]
            && self.points[2] == other.points[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let new = Vec3::new(0.0, 0.0, 0.0);
        let origin = Vec3::origin();

        assert_eq!(new, origin);
    }

    #[test]
    fn add() {
        let foo = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(4.0, 5.0, 6.0);

        let added = foo + other;

        assert_eq!(added.points[0], 5.0);
        assert_eq!(added.points[1], 7.0);
        assert_eq!(added.points[2], 9.0);
    }

    #[test]
    fn sub() {
        let foo = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(4.0, 5.0, 6.0);

        let subbed = other - foo;

        assert_eq!(subbed.points[0], 3.0);
        assert_eq!(subbed.points[1], 3.0);
        assert_eq!(subbed.points[2], 3.0);
    }

    #[test]
    fn mul() {
        let foo = Vec3::new(1.0, 2.0, 3.0);
        let other = Vec3::new(4.0, 5.0, 6.0);

        let mul = other * foo;

        assert_eq!(mul.points[0], 4.0);
        assert_eq!(mul.points[1], 10.0);
        assert_eq!(mul.points[2], 18.0);

        let bar = Vec3::new(2.0, 4.0, 8.0);
        let mul = bar * 2.0;

        assert_eq!(mul.points[0], 4.0);
        assert_eq!(mul.points[1], 8.0);
        assert_eq!(mul.points[2], 16.0);
    }

    #[test]
    fn div() {
        let foo = Vec3::new(1.0, 2.0, 3.0);
        let other = foo / 2.0;

        assert_eq!(other.points[0], 0.5);
        assert_eq!(other.points[1], 1.0);
        assert_eq!(other.points[2], 1.5);
    }

    #[test]
    fn add_assign() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);

        vec1 += vec2;

        assert_eq!(vec1.x(), 5.0);
        assert_eq!(vec1.y(), 7.0);
        assert_eq!(vec1.z(), 9.0);
    }

    #[test]
    fn multiply_assign() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec *= 2.0;

        assert_eq!(vec.x(), 2.0);
        assert_eq!(vec.y(), 4.0);
        assert_eq!(vec.z(), 6.0);
    }

    #[test]
    fn divide_assign() {
        let mut vec = Vec3::new(2.0, 4.0, 8.0);
        vec /= 2.0;

        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 4.0);
    }

    #[test]
    fn negative() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let neg = -vec;

        assert_eq!(neg.points[0], -1.0);
        assert_eq!(neg.points[1], -2.0);
        assert_eq!(neg.points[2], -3.0);
    }

    #[test]
    fn length_squared() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.length_squared(), 14.0);
    }

    #[test]
    fn length() {
        let mut vec = Vec3::new(2.0, 2.0, 2.0);
        let expected = 12 as f32;
        assert_eq!(vec.length(), expected.sqrt());
    }
}
