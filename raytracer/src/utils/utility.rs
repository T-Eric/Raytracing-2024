// some constants

pub const INFINITY: f64 = f64::INFINITY;
pub const _PI: f64 = std::f64::consts::PI;

//some utility functions

pub fn _degrees_to_radians(degrees: f64) -> f64 {
    degrees * _PI / 180.0
}
