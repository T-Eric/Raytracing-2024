use crate::camera::background::BackGround;
use crate::camera::Camera;
use crate::features::normal_map::{MapMap, OriginMap};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::instances::flats::{cube, Quad};
use crate::hittable::instances::sphere::Sphere;
use crate::hittable::transforms::rotate_y::RotateY;
use crate::hittable::transforms::translate::Translate;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::textures::image_texture::ImageTexture;
use crate::textures::SolidColor;
use crate::util::color::Color;
use crate::util::vec3::{Point3, Vec3};
use crate::util::{ASPECT_RATIO, IMAGE_WIDTH, MAX_RECURSE_DEPTH, SAMPLES_PER_PIXEL};
use std::sync::Arc;

pub fn _cornell_box_normal() -> (HittableList, HittableList, Camera, BackGround) {
    let mut world = HittableList::default();
    let origin_nmap = OriginMap::default();

    let red = Lambertian::<SolidColor>::new_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new_color(Color::new(0.12, 0.45, 0.15));
    let blue = Lambertian::<SolidColor>::new_color(Color::new(0.4, 0.6, 0.8));
    let pink = Lambertian::<SolidColor>::new_color(Color::new(0.8, 0.4, 0.4));
    let light = DiffuseLight::<SolidColor>::new_color(Color::new(6.0, 6.0, 6.0));

    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::default(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(127.5, 554.0, 127.5),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        light,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::default(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
        origin_nmap.clone(),
    )));

    let box1 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        blue,
        origin_nmap.clone(),
    ); // material=aluminum
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        pink,
        origin_nmap.clone(),
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 300.0, 65.0)));
    world.add(box2);

    let glass = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new_static(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let mut lights = HittableList::default();
    let m = Lambertian::<SolidColor>::new_color(Color::default());
    lights.add(Box::new(Quad::new(
        Point3::new(127.5, 554.0, 127.5),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        m.clone(),
        origin_nmap,
    )));
    lights.add(Box::new(Sphere::new_static(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        m,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.max_recurse_depth = MAX_RECURSE_DEPTH;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.initialize();

    let back_ground = BackGround::new(Arc::new(SolidColor::new_color(Color::new(0.5, 0.7, 1.0))));

    (world, lights, cam, back_ground)
}

pub fn _cornell_box_nmap() -> (HittableList, HittableList, Camera, BackGround) {
    let mut world = HittableList::default();
    let origin_nmap = OriginMap::default();

    let red = Lambertian::<SolidColor>::new_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::<SolidColor>::new_color(Color::new(6.0, 6.0, 6.0));

    let apple = Lambertian::new_tex(ImageTexture::new_path("source/normalmaps/sourapple.jpg"));
    let apple_nmap = MapMap::new("source/normalmaps/sourapple.png");
    let weavey = Metal::new_tex(ImageTexture::new_path("source/normalmaps/R.jpg"), 0.0);
    let weavey_nmap = MapMap::new("source/normalmaps/R.png");
    let clouds = Metal::new_tex(ImageTexture::new_path("source/normalmaps/clouds.jpg"), 0.2);
    let clouds_nmap = MapMap::new("source/normalmaps/clouds.png");

    world.add(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::default(),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(127.5, 554.0, 127.5),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        light,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::default(),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white,
        origin_nmap.clone(),
    )));
    world.add(Box::new(Quad::new(
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
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Box::new(Translate::new(box1, Vec3::new(295.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = cube(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        clouds,
        clouds_nmap,
    );
    // let box2 = Box::new(RotateY::new(box2, -18.0));
    let box2 = RotateY::new(box2, -40.0);
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 300.0, 65.0)));
    world.add(box2);

    let glass = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new_static(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let mut lights = HittableList::default();
    let m = Lambertian::<SolidColor>::new_color(Color::default());
    lights.add(Box::new(Quad::new(
        Point3::new(127.5, 554.0, 127.5),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        m.clone(),
        origin_nmap,
    )));
    lights.add(Box::new(Sphere::new_static(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        m,
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.max_recurse_depth = MAX_RECURSE_DEPTH;

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.initialize();

    let back_ground = BackGround::new(Arc::new(SolidColor::new_color(Color::new(0.2, 0.1, 0.2))));

    (world, lights, cam, back_ground)
}
