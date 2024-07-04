use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::utils::bvh::BvhNode;
use crate::utils::constant_medium::ConstMedium;
use crate::utils::hittable::{RotateY, Translate};
use crate::utils::quad::{cube, Quad};
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
    cam.image_width = 300;
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
    let savefile = savepath.clone() + &*String::from("/99.png");
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
        white.clone(),
    );
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

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 64;
    cam.max_recurse_depth = 50;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book3");
    let savefile = savepath.clone() + &*String::from("/1.png");
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

fn cornell_smoke() -> std::io::Result<()> {
    let now = Instant::now();

    let mut world = HittableList::default();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));

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
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
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
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Arc::new(ConstMedium::new_color(
        box1,
        0.01,
        Color::default(),
    )));

    let box2 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Arc::new(ConstMedium::new_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_recurse_depth = 50;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/0.png");
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

fn final_scene(
    image_width: i32,
    samples_per_pixel: i32,
    max_recurse_depth: i32,
) -> std::io::Result<()> {
    let now = Instant::now();

    let mut boxes1 = HittableList::default();
    let mut rng = rand::thread_rng();

    let ground = Arc::new(Lambertian::new_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(cube(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::default();

    world.add(Arc::new(BvhNode::new_list(&mut boxes1)));

    let light = Arc::new(DiffuseLight::new_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_mat = Arc::new(Lambertian::new_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_motive(
        center1, center2, 50.0, sphere_mat,
    )));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Arc::new(Sphere::new_static(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstMedium::new_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new_static(
        Point3::default(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstMedium::new_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new_arc(Arc::new(ImageTexture::new(
        "source/earthmap.jpg",
    ))));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_arc(pertext)),
    )));

    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new_static(
            Point3::random_in(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new_list(&mut boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_recurse_depth = max_recurse_depth;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/23.png");
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
    match 7 {
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
        7 => {
            cornell_box().expect("Fail!");
        }
        8 => {
            cornell_smoke().expect("Fail!");
        }
        9 => {
            final_scene(800, 10000, 40).expect("Fail!");
        }
        10 => {
            final_scene(300, 100, 4).expect("Fail!");
        }
        _ => (),
    }
}
