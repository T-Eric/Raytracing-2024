use crate::utils::bvh::BvhNode;
use crate::utils::constant_medium::ConstMedium;
use crate::utils::hittable::{RotateY, Translate};
use crate::utils::quad::{cube, Quad};
use crate::utils::sphere::Sphere;
use crate::utils::texture::{ImageTexture, NoiseTexture};
use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use utils::camera::Camera;
use utils::color::Color;
use utils::hittable_list::HittableList;
use utils::material::*;
use utils::normal_map::*;
use utils::vec3::Point3;
use utils::vec3::Vec3;

fn bouncing_spheres() -> std::io::Result<()> {
    let now = Instant::now();
    // world
    let mut world = HittableList::default();
    // materials
    let ground_mat = Arc::new(Lambertian::new_color(Color::new(1.0, 1.0, 1.0)));
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
                    world.add(Arc::new(Sphere::new_static(
                        center,
                        0.2,
                        Arc::new(Lambertian::new_color(albedo)),
                    )));
                } else if choose_mat < 0.9 {
                    // metal
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rand::random::<f64>();
                    world.add(Arc::new(Sphere::new_static(
                        center,
                        0.2,
                        Arc::new(Metal::new_color(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Arc::new(Sphere::new_static(
                        center,
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
    let material3 = Arc::new(Metal::new_color(Color::new(0.5, 0.6, 0.7), 0.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // let mut world_ = HittableList::default();
    // world_.add(Arc::new(BvhNode::new_list(&mut world)));
    // let world = world_;

    let lights = HittableList::default();
    let lights = Arc::new(lights);

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1000;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.gauss_fuzzing_scale = 2.0;
    cam.edge_detect = true;

    let savepath = String::from("output/book0");
    let savefile = savepath.clone() + &*String::from("/25.png");
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

fn final_scene(
    image_width: i32,
    samples_per_pixel: i32,
    max_recurse_depth: i32,
) -> std::io::Result<()> {
    let now = Instant::now();

    let mut boxes1 = HittableList::default();
    let mut rng = rand::thread_rng();

    let ground = Arc::new(Lambertian::new_color(Color::new(0.48, 0.83, 0.53)));
    let initial_nmap = Arc::new(OriginMap::default());

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
                initial_nmap.clone(),
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
        initial_nmap,
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
        Arc::new(Metal::new_color(Color::new(0.8, 0.8, 0.9), 1.0)),
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

    let emat = Arc::new(Lambertian::new_tex(Arc::new(ImageTexture::new(
        "source/earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new_tex(pertext)),
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

    let lights = Arc::new(HittableList::default());

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

    cam.edge_detect = true;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/23.png");
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

mod utils;
fn cornell_box() -> std::io::Result<()> {
    let now = Instant::now();

    let mut world = HittableList::default();
    let origin_nmap = Arc::new(OriginMap::default());

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    // let blue = Arc::new(Lambertian::new_color(Color::new(0.4, 0.6, 0.8)));
    // let pink = Arc::new(Lambertian::new_color(Color::new(0.8, 0.4, 0.4)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    let apple = Arc::new(Lambertian::new_tex(Arc::new(ImageTexture::new(
        "source/normalmaps/sourapple.jpg",
    ))));
    let apple_nmap = Arc::new(MapMap::new("source/normalmaps/sourapple.png"));
    let weavey = Arc::new(Lambertian::new_tex(Arc::new(ImageTexture::new(
        "source/normalmaps/R.jpg",
    ))));
    let weavey_nmap = Arc::new(MapMap::new("source/normalmaps/R.png"));
    let clouds = Arc::new(Metal::new_tex(
        Arc::new(ImageTexture::new("source/normalmaps/clouds.jpg")),
        0.2,
    ));
    let clouds_nmap = Arc::new(MapMap::new("source/normalmaps/clouds.png"));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
        origin_nmap.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::default(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
        origin_nmap.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
        origin_nmap.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::default(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
        origin_nmap.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white,
        origin_nmap.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        apple,
        apple_nmap,
    )));

    let box1 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        weavey,
        weavey_nmap,
    ); // material=aluminum
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    // let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(295.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        clouds,
        clouds_nmap,
    );
    // let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(RotateY::new(box2, -40.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 300.0, 65.0)));
    world.add(box2);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let mut lights = HittableList::default();
    let m = Arc::new(Lambertian::new_color(Color::default()));
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        m.clone(),
        origin_nmap,
    )));
    lights.add(Arc::new(Sphere::new_static(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        m,
    )));
    let lights = Arc::new(lights);

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 400;
    cam.max_recurse_depth = 50;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book0");
    let savefile = savepath.clone() + &*String::from("/36.png");
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

fn earth() -> std::io::Result<()> {
    let now = Instant::now();
    let earth_texture = Arc::new(ImageTexture::new("source/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_tex(earth_texture));
    let globe = Arc::new(Sphere::new_static(Point3::default(), 2.0, earth_surface));

    let mut world = HittableList::default();
    world.add(globe);

    let mut lights = HittableList::default();
    let m = Arc::new(Lambertian::new_color(Color::default()));
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        m,
        Arc::new(OriginMap::default()),
    )));
    let lights = Arc::new(lights);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book2");
    let savefile = savepath.clone() + &*String::from("/0.png");
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

fn debugger() -> std::io::Result<()> {
    let now = Instant::now();
    let globe = Arc::new(Sphere::new_static(
        Point3::default(),
        1.5,
        Arc::new(Lambertian::new_color(Color::new(0.8, 0.4, 0.5))),
    ));

    let mut world = HittableList::default();
    world.add(globe);
    world.add(Arc::new(Sphere::new_static(
        Point3::new(1.5, 1.5, 1.5),
        0.5,
        Arc::new(DiffuseLight::new_color(Color::new(3.0, 3.0, 5.0))),
    )));

    let mut lights = HittableList::default();
    lights.add(Arc::new(Sphere::new_static(
        Point3::new(1.5, 1.5, 1.5),
        0.5,
        Arc::new(Lambertian::new_color(Color::default())),
    )));
    let lights = Arc::new(lights);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_recurse_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    let savepath = String::from("output/book0");
    let savefile = savepath.clone() + &*String::from("/0.png");
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
    match 3 {
        1 => {
            bouncing_spheres().expect("Fail!");
        }
        2 => {
            final_scene(400, 100, 50).expect("Fail!");
        }
        3 => {
            cornell_box().expect("Fail!");
        }
        4 => {
            earth().expect("Fail!");
        }
        5 => {
            debugger().expect("Fail!");
        }
        _ => (),
    }
}
