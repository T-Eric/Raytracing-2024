use std::sync::Arc;

use crate::utils::color::Color;
use crate::utils::material::*;
use crate::utils::vec3::Vec3;
use utils::camera::Camera;
use utils::hittable_list::HittableList;
use utils::sphere::Sphere;
use utils::vec3::Point3;

mod utils;

fn main() {
    // world
    let mut world = HittableList::default();
    // materials
    let material_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
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
        &Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Some(material_bubble),
    )));
    world.add(Arc::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));
    //
    // //for 1.19
    // let rr = (PI / 4.0).cos();
    //
    // let material_left = Arc::new(Lambertian::new(&Color::new(0.0, 0.0, 1.0)));
    // let material_right = Arc::new(Lambertian::new(&Color::new(1.0, 0.0, 0.0)));
    //
    // world.add(Arc::new(Sphere::new(
    //     &Point3::new(-rr, 0.0, -1.0),
    //     rr,
    //     Some(material_left),
    // )));
    // world.add(Arc::new(Sphere::new(
    //     &Point3::new(rr, 0.0, -1.0),
    //     rr,
    //     Some(material_right),
    // )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;

    cam.vfov = 90.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.render(&world);
}
