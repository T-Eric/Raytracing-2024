use crate::utils::hittable::{RotateY, Translate};
use crate::utils::quad::{cube, Quad};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use utils::camera::Camera;
use utils::color::Color;
use utils::hittable_list::HittableList;
use utils::material::*;
use utils::vec3::Point3;
use utils::vec3::Vec3;
mod utils;
fn cornell_box() -> std::io::Result<()> {
    let now = Instant::now();

    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::default(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::default(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        Arc::new(Metal::new(Color::new(0.8, 0.85, 0.88), 0.0)),
    ); // material=aluminum
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let lights = Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        Arc::new(Lambertian::new_color(Color::default())),
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 900;
    cam.max_recurse_depth = 50;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book3");
    let savefile = savepath.clone() + &*String::from("/12.png");
    let path = Path::new(&savepath);

    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, lights, savefile);
    } else {
        cam.render(world, lights, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn main() {
    cornell_box().expect("Fail!");
}
