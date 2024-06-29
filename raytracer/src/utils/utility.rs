// some constants

use rand::random;

pub const INFINITY: f64 = f64::INFINITY;
pub const _PI: f64 = std::f64::consts::PI;

//some utility functions

pub fn _degrees_to_radians(degrees: f64) -> f64 {
    degrees * _PI / 180.0
}

pub fn _random_double(mn: f64, mx: f64) -> f64 {
    mn + (mx - mn) * random::<f64>()
}

// if u want rand double in [0,1), call random::<f64>() directly
