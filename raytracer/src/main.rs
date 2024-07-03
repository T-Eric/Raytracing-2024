use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::utils::bvh::BvhNode;
use crate::utils::quad::Quad;
use crate::utils::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use std::time::Instant;
use utils::camera::Camera;
use utils::color::Color;
use utils::hittable_list::HittableList;
use utils::material::*;
use utils::sphere::Sphere;
use utils::vec3::Point3;
use utils::vec3::Vec3;

mod utils;

fn bouncing_spheres() -> std::io::Result<()> {
    let now = Instant::now();
    // world
    let mut world = HittableList::default();
    // materials
    let ground_mat = Arc::new(Lambertian::new_arc(Arc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
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

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.75 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_motive(
                        center,
                        center2,
                        0.2,
                        Arc::new(Lambertian::new_color(albedo)),
                    )));
                } else if choose_mat < 0.9 {
                    // metal
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rand::random::<f64>();
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_motive(
                        center,
                        center2,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_motive(
                        center,
                        center2,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::new_color(Color::new(0.6, 0.3, 0.3)));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(Color::new(0.5, 0.6, 0.7), 0.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut world_ = HittableList::default();
    world_.add(Arc::new(BvhNode::new_list(&mut world)));
    let world = world_;

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/2.png");
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn checkered_spheres() -> std::io::Result<()> {
    let now = Instant::now();
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_arc(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_arc(checker)),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/3.png");
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn earth() -> std::io::Result<()> {
    let now = Instant::now();
    let earth_texture = Arc::new(ImageTexture::new("source/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_arc(earth_texture));
    let globe = Arc::new(Sphere::new_static(Point3::default(), 2.0, earth_surface));

    let mut world = HittableList::default();
    world.add(globe);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/5.png");
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn perlin_spheres() -> std::io::Result<()> {
    let now = Instant::now();

    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_arc(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_arc(pertext)),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/15.png");
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn quads() -> std::io::Result<()> {
    let now = Instant::now();

    let mut world = HittableList::default();

    let left_red = Arc::new(Lambertian::new_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_color(Color::new(0.2, 0.2, 1.0)));
    let up_orange = Arc::new(Lambertian::new_color(Color::new(1.0, 0.5, 0.0)));
    let down_teal = Arc::new(Lambertian::new_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        up_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        down_teal,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/16.png");
    let path = Path::new(&savepath);
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn simple_light() -> std::io::Result<()> {
    let now = Instant::now();

    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_arc(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_arc(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(5.0, 5.0, 5.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/18.png");
    let path = Path::new(&savepath);
    
    if !path.exists() {
        fs::create_dir_all(path)?;
        cam.render(world, savefile);
    } else {
        cam.render(world, savefile);
    }

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
    Ok(())
}

fn main() {
    match 6 {
        1 => {
            bouncing_spheres().expect("Fail!");
        }
        2 => {
            checkered_spheres().expect("Fail!");
        }
        3 => {
            earth().expect("Fail!");
        }
        4 => {
            perlin_spheres().expect("Fail!");
        }
        5 => {
            quads().expect("Fail!");
        }
        6 => {
            simple_light().expect("Fail!");
        }
        _ => (),
    }
}
