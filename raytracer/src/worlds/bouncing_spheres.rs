use crate::camera::background::BackGround;
use crate::camera::Camera;
use crate::hittable::bvh::BvhNode;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::instances::sphere::Sphere;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::textures::SolidColor;
use crate::util::color::Color;
use crate::util::vec3::{Point3, Vec3};
use crate::util::{ASPECT_RATIO, IMAGE_WIDTH, MAX_RECURSE_DEPTH, SAMPLES_PER_PIXEL};
use rand::Rng;
use std::sync::Arc;

pub fn _bouncing_spheres() -> (HittableList, HittableList, Camera, BackGround) {
    // world
    let mut world = HittableList::default();
    // materials
    let ground_mat = Lambertian::<SolidColor>::new_color(Color::new(1.0, 1.0, 1.0));
    world.add(Box::new(Sphere::new_static(
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
                    world.add(Box::new(Sphere::new_static(
                        center,
                        0.2,
                        Lambertian::<SolidColor>::new_color(albedo),
                    )));
                } else if choose_mat < 0.9 {
                    // metal
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rand::random::<f64>();
                    world.add(Box::new(Sphere::new_static(
                        center,
                        0.2,
                        Metal::<SolidColor>::new_color(albedo, fuzz),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new_static(
                        center,
                        0.2,
                        Dielectric::new(1.5),
                    )));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Lambertian::<SolidColor>::new_color(Color::new(0.6, 0.3, 0.3));
    world.add(Box::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Metal::<SolidColor>::new_color(Color::new(0.5, 0.6, 0.7), 0.0);
    world.add(Box::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut world_ = HittableList::default();
    world_.add(Box::new(BvhNode::new_list(world)));
    let world = world_;

    let lights = HittableList::default();

    let mut cam = Camera::default();

    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.max_recurse_depth = MAX_RECURSE_DEPTH;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::default();
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.gauss_fuzzing_scale = 2.0;
    cam.edge_detect = true;

    cam.initialize();

    let back_ground = BackGround::new(Arc::new(SolidColor::new_color(Color::new(0.5, 0.7, 1.0))));

    (world, lights, cam, back_ground)
}
