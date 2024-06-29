use std::sync::Arc;

use utils::camera::Camera;
use utils::hittable_list::HittableList;
use utils::sphere::Sphere;
use utils::vec3::Point3;

mod utils;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    // world
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut cam = Camera::new(image_width, aspect_ratio, samples_per_pixel);
    cam.render(&world);
}
