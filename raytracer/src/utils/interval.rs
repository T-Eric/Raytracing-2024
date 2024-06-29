use crate::utils::utility::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}

impl Clone for Interval {
    fn clone(&self) -> Self {
        Self::copy(self)
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn copy(other: &Interval) -> Self {
        Self {
            min: other.min,
            max: other.max,
        }
    }

    pub fn _size(&self) -> f64 {
        self.max - self.min
    }
    pub fn _contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn _surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
//consts

impl Interval {
    pub const _EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };
    pub const _UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
}
