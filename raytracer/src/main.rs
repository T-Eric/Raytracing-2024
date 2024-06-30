use std::sync::Arc;

use crate::utils::color::Color;
use crate::utils::material::*;
use utils::camera::Camera;
use utils::hittable_list::HittableList;
use utils::sphere::Sphere;
use utils::vec3::Point3;

mod utils;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_recurse_depth = 48;
    // world
    let mut world = HittableList::default();
    //materials
    let material_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_right = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(material_ground),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        Some(material_center),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));

    let mut cam = Camera::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_recurse_depth,
    );
    cam.render(&world);
}
