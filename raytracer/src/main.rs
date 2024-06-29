mod utils;
use utils::color::{Color, *};
use utils::ray::Ray;
use utils::vec3::{Point3, Vec3, *};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2.0 * dot(r.direction(), &oc);
    let c = dot(&oc, &oc) - radius * radius;
    let delta = b * b - 4.0 * a * c;
    delta >= 0f64
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
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

    //camera
    let focal_length = 1.0;
    let view_height = 2.0;
    let view_width = view_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    // the view vectors
    let view_u = Vec3::new(view_width, 0.0, 0.0);
    let view_v = Vec3::new(0.0, -view_height, 0.0);
    //the pixel deltas
    let pixel_delta_u = &view_u / image_width as f64;
    let pixel_delta_v = &view_v / image_height as f64;
    //the view location
    let viewport_up_left = camera_center.clone()
        - Vec3::new(0.0, 0.0, focal_length)
        - &view_u / 2.0
        - &view_v / 2.0;
    let pixel00_loc = viewport_up_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                &pixel00_loc + &(&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &camera_center;
            let r = Ray::new(&camera_center, &ray_direction);
            let pixel_color = ray_color(&r);
            put_color(&pixel_color);
        }
    }
}
