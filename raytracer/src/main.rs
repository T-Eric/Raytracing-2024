use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use utils::camera::Camera;
use utils::color::Color;
use utils::hittable_list::HittableList;
use utils::material::*;
use utils::sphere::Sphere;
use utils::vec3::Point3;
use utils::vec3::Vec3;

mod utils;

fn main() -> std::io::Result<()> {
    // world
    let mut world = HittableList::default();
    // materials
    let ground_mat = Arc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    // rand balls
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.75 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    world.add(Arc::new(Sphere::new(
                        &center,
                        0.2,
                        Arc::new(Lambertian::new(&albedo)),
                    )));
                } else if choose_mat < 0.9 {
                    // metal
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rand::random::<f64>();
                    world.add(Arc::new(Sphere::new(
                        &center,
                        0.2,
                        Arc::new(Metal::new(&albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new(
                        &center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(&Color::new(0.5, 0.6, 0.7), 0.0));
    world.add(Arc::new(Sphere::new(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/Book2");
    let savefile = savepath.clone() + &*String::from("/0.png");
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }
    Ok(())
}
