
mod utils;

use utils::color::{put_color, Color};
use utils::ray::Ray;
use utils::vec3::{Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
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

    //camera
    let focal_length = 1.0;
    let view_height = 2.0;
    let view_width = view_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    // the view vectors
    let view_u = Vec3::new(view_width, 0.0, 0.0);
    let view_v = Vec3::new(0.0, -view_height, 0.0);
    //the pixel deltas
    let pixel_delta_u = view_u.clone() / image_width as f64;
    let pixel_delta_v = view_v.clone() / image_height as f64;
    //the view location
    let viewport_up_left = camera_center.clone()
        - Vec3::new(0.0, 0.0, focal_length)
        - view_u.clone() / 2.0
        - view_v.clone() / 2.0;
    let pixel00_loc = viewport_up_left + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

    println!("P3\n{0} {1}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone()
                + (pixel_delta_u.clone() * i as f64)
                + (pixel_delta_v.clone() * j as f64);
            let ray_direction = pixel_center - camera_center.clone();
            let r = Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&r);
            put_color(&pixel_color);
        }
    }
}
