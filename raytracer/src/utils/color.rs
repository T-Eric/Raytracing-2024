use crate::utils::interval::Interval;
use crate::utils::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn put_color(pixel_color: &Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Apply linear to gamma-2 transformation
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translation
    static INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rbyte = (256.000 * INTENSITY.clamp(r)) as i32;
    let gbyte = (256.000 * INTENSITY.clamp(g)) as i32;
    let bbyte = (256.000 * INTENSITY.clamp(b)) as i32;

    println!("{rbyte} {gbyte} {bbyte}");
}
