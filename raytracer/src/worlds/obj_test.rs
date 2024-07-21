use crate::camera::background::BackGround;
use crate::camera::Camera;
use crate::features::normal_map::OriginMap;
use crate::features::obj_mesh::{obj_mesh, LoadParam};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::instances::flats::Quad;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::textures::SolidColor;
use crate::util::color::Color;
use crate::util::vec3::{Point3, Vec3};
use crate::util::{ASPECT_RATIO, IMAGE_WIDTH, MAX_RECURSE_DEPTH, SAMPLES_PER_PIXEL};
use std::sync::Arc;

pub fn _obj_test() -> (HittableList, HittableList, Camera, BackGround) {
    let mut world = HittableList::default();
    let origin_nmap = OriginMap::default();

    let red = Lambertian::<SolidColor>::new_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new_color(Color::new(0.12, 0.45, 0.15));
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

    let patrick_load_param = LoadParam {
        obj_file: "source/objs/patrick.obj",
        zoom: 150.0,
        offset: Vec3::new(320.0, 30.0, 277.5),
        rot_x: 0.0,
        rot_y: 165.0,
        rot_z: 0.0,
    };
    let patrick = obj_mesh(patrick_load_param);
    world.add(patrick);

    let mut lights = HittableList::default();
    let m = Lambertian::<SolidColor>::new_color(Color::default());
    lights.add(Box::new(Quad::new(
        Point3::new(127.5, 554.0, 127.5),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 300.0),
        m,
        origin_nmap,
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
