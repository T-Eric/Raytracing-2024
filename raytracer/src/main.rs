mod utils;

use crate::utils::hittable::Hittable;
use std::sync::Arc;
use utils::color::{Color, *};
use utils::hittable::HitRecord;
use utils::hittable_list::HittableList;
use utils::ray::Ray;
use utils::sphere::Sphere;
use utils::utility::INFINITY;
use utils::vec3::{Point3, Vec3, *};
use utils::interval::Interval;

//for pic 1.3
fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, &Interval::new(0.0, INFINITY), &mut rec) {
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = {
        let tmp_height = (image_width as f64) / aspect_ratio;
        if tmp_height < 1.0 {
            1
        } else {
            tmp_height as i32
        }
    };
    // world
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    //camera
    let focal_length = 1.0;
    let view_height = 2.0;
    let view_width = view_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    // the view vectors
    let view_u = Vec3::new(view_width, 0.0, 0.0);
    let view_v = Vec3::new(0.0, -view_height, 0.0);
    //the pixel deltas
    let pixel_delta_u = &view_u / image_width as f64;
    let pixel_delta_v = &view_v / image_height as f64;
    //the view location
    let viewport_up_left =
        camera_center.clone() - Vec3::new(0.0, 0.0, focal_length) - &view_u / 2.0 - &view_v / 2.0;
    let pixel00_loc = viewport_up_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                &pixel00_loc + &(&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &camera_center;
            let r = Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&r, &world);
            put_color(&pixel_color);
        }
    }
}
