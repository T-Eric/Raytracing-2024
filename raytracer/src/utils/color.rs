use crate::utils::interval::Interval;
use crate::utils::vec3::Vec3;

pub type Color = Vec3;

pub fn put_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    static INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rbyte = (256.000 * INTENSITY.clamp(r))as i32;
    let gbyte = (256.000 * INTENSITY.clamp(g))as i32;
    let bbyte = (256.000 * INTENSITY.clamp(b))as i32;

    println!("{rbyte} {gbyte} {bbyte}");
}
