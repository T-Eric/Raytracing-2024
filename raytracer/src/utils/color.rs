use crate::utils::vec3::Vec3;

pub type Color = Vec3;

pub fn put_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte: i32 = (255.999 * r) as i32;
    let gbyte: i32 = (255.999 * g) as i32;
    let bbyte: i32 = (255.999 * b) as i32;

    println!("{rbyte} {gbyte} {bbyte}");
}
