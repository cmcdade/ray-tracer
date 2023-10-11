pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    fn default() -> Interval {
        Interval {
            min: -std::f32::INFINITY,
            max: std::f32::INFINITY,
        }
    }
}
