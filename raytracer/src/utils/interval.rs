use crate::utils::utility::INFINITY;
use std::ops::Add;

#[derive(Copy)]
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
    pub fn new_interval(a: &Interval, b: &Interval) -> Interval {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
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
    pub fn expand(&self, delta: f64) -> Interval {
        Interval::new(self.min - delta / 2.0, self.max + delta / 2.0)
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

//consts
impl Interval {
    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };
    pub const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
}
