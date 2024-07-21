use crate::camera::background::BackGround;
use crate::camera::Camera;
use crate::features::normal_map::{MapMap, OriginMap};
use crate::features::obj_mesh::{obj_mesh, LoadParam};
use crate::hittable::bvh::BvhNode;
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
use rand::Rng;
use std::sync::Arc;

pub fn final_scene() -> (HittableList, HittableList, Camera, BackGround) {
    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let m = Lambertian::<SolidColor>::new_color(Color::default());
    let origin_nmap = OriginMap::default();

    let ground_mat = Lambertian::<SolidColor>::new_color(Color::new(1.0, 1.0, 1.0));
    world.add(Box::new(Sphere::new_static(
        Point3::new(0.0, -1500.0, 0.0),
        1500.0,
        ground_mat,
    )));

    let axis_mat = DiffuseLight::<SolidColor>::new_color(Color::new(
        11.0 / 256.0 * 2.0,
        165.0 / 266.0 * 2.0,
        52.0 / 256.0 * 2.0,
    ));
    let xzaxis = Quad::new(
        Point3::new(10.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(-10.0, 0.0, 0.0),
        axis_mat,
        origin_nmap,
    );
    world.add(Box::new(xzaxis));

    let origin_point_mat = DiffuseLight::<SolidColor>::new_color(Color::new(
        232.0 / 256.0 * 2.0,
        234.0 / 256.0 * 2.0,
        103.0 / 256.0 * 2.0,
    ));
    let origin_point = Sphere::new_static(Point3::default(), 1.0, origin_point_mat);
    world.add(Box::new(origin_point));
    lights.add(Box::new(Sphere::new_static(
        Point3::default(),
        1.0,
        m.clone(),
    )));

    let x_cod_mat = DiffuseLight::<SolidColor>::new_color(Color::new(
        241.0 / 256.0 * 2.0,
        4.0 / 266.0 * 2.0,
        91.0 / 256.0 * 2.0,
    ));
    let z_cod_mat = DiffuseLight::<SolidColor>::new_color(Color::new(
        43.0 / 256.0 * 2.0,
        114.0 / 266.0 * 2.0,
        238.0 / 256.0 * 2.0,
    ));

    for i in -8..=8 {
        if i == 0 {
            continue;
        }
        let x_ball = Sphere::new_static(
            Point3::new(i as f64 * 10.0, 0.0, 0.0),
            1.0,
            x_cod_mat.clone(),
        );
        world.add(Box::new(x_ball));
        lights.add(Box::new(Sphere::new_static(
            Point3::new(i as f64 * 10.0, 0.0, 0.0),
            1.0,
            m.clone(),
        )));
    }
    for i in -24..=8 {
        if i == 0 {
            continue;
        }
        let z_ball = Sphere::new_static(
            Point3::new(
                0.0,
                (1500.0f64.powi(2) - (i as f64 * 10.0).powi(2)).sqrt() - 1500.0,
                i as f64 * 10.0,
            ),
            1.0,
            z_cod_mat.clone(),
        );
        world.add(Box::new(z_ball));
        lights.add(Box::new(Sphere::new_static(
            Point3::new(
                0.0,
                (1500.0f64.powi(2) - (i as f64 * 10.0).powi(2)).sqrt() - 1500.0,
                i as f64 * 10.0,
            ),
            1.0,
            m.clone(),
        )));
    }

    let fighter_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Fighter_Kirby/fighterkirby.obj",
        zoom: 1.0,
        offset: Vec3::new(-20.0, 0.0, 40.0),
        rot_x: 10.0,
        rot_y: 150.0,
        rot_z: -10.0,
    };
    let fighter_kirby = obj_mesh(fighter_kirby_load_param);
    world.add(fighter_kirby);

    let sword_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Swordkirby/DolSwrodkirby.obj",
        zoom: 1.2,
        offset: Vec3::new(-5.0, 0.0, 20.0),
        rot_x: 15.0,
        rot_y: 120.0,
        rot_z: 10.0,
    };
    let sword_kirby = obj_mesh(sword_kirby_load_param);
    world.add(sword_kirby);

    let plasma_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Plasmakirby/DolPlasmakirby.obj",
        zoom: 1.0,
        offset: Vec3::new(-45.0, 2.0, -5.0),
        rot_x: -45.0,
        rot_y: -75.0,
        rot_z: 0.0,
    };
    let plasma_kirby = obj_mesh(plasma_kirby_load_param);
    world.add(plasma_kirby);

    let ice_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Icekirby/DolIcekirby.obj",
        zoom: 1.0,
        offset: Vec3::new(-35.0, 0.0, -35.0),
        rot_x: 0.0,
        rot_y: -30.0,
        rot_z: 30.0,
    };
    let ice_kirby = obj_mesh(ice_kirby_load_param);
    world.add(ice_kirby);

    let needle_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Needlekirby/DolNeedlekirby.obj",
        zoom: 1.0,
        offset: Vec3::new(-10.0, 0.0, -30.0),
        rot_x: 0.0,
        rot_y: -10.0,
        rot_z: 5.0,
    };
    let needle_kirby = obj_mesh(needle_kirby_load_param);
    world.add(needle_kirby);

    let tornado_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Tornadokirby/DolTornadokirby.obj",
        zoom: 1.0,
        offset: Vec3::new(-50.0, 18.0, 10.0),
        rot_x: 0.0,
        rot_y: -20.0,
        rot_z: -10.0,
    };
    let tornado_kirby = obj_mesh(tornado_kirby_load_param);
    world.add(tornado_kirby);

    let wing_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Wingkirby/DolWingkirby.obj",
        zoom: 1.0,
        offset: Vec3::new(10.0, 12.0, 10.0),
        rot_x: 0.0,
        rot_y: 10.0,
        rot_z: -10.0,
    };
    let wing_kirby = obj_mesh(wing_kirby_load_param);
    world.add(wing_kirby);

    let sleep_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Sleepkirby/DolSleepkirby.obj",
        zoom: 1.0,
        offset: Vec3::new(52.0, 20.0, -14.0),
        rot_x: 0.0,
        rot_y: 0.0,
        rot_z: 0.0,
    };
    let sleep_kirby = obj_mesh(sleep_kirby_load_param);
    world.add(sleep_kirby);

    let allstar_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Allstarkirby/allstar.obj",
        zoom: 1.5,
        offset: Vec3::new(-20.0, 20.0, 0.0),
        rot_x: 30.0,
        rot_y: -50.0,
        rot_z: -60.0,
    };
    let allstar_kirby = obj_mesh(allstar_kirby_load_param);
    world.add(allstar_kirby);

    let beam_kirby_load_param = LoadParam {
        obj_file: "source/objs/kirbies/Beamkirby/DolBeamkirby.obj",
        zoom: 1.0,
        offset: Vec3::new(10.0, 0.0, -100.0),
        rot_x: 0.0,
        rot_y: -5.0,
        rot_z: 0.0,
    };
    let beam_kirby = obj_mesh(beam_kirby_load_param);
    world.add(beam_kirby);

    let mut rng = rand::thread_rng();
    let mut sky_world = HittableList::default();

    for i in -8..=8 {
        for j in -8..=8 {
            let i = (i * 10) as f64;
            let j = (j * 10) as f64;

            let height = 80.0 + rng.gen_range(-5.0..5.0);
            let center = Point3::new(
                i,
                height * (40.0 + j / 10.0) / 48.0 * (72.0 - i / 10.0) / 80.0,
                j,
            );
            let radius = rng.gen_range(6.0..9.0);
            let zoom = (24.0 - i.abs() / 10.0) / 24.0;
            let big_ball = Sphere::new_static(
                center,
                radius * zoom,
                Metal::<SolidColor>::new_color(rand_color_gen(0.3, 0.8, 0.2, 0.6, 0.5, 1.0), 0.0),
            );
            sky_world.add(Box::new(big_ball));
            let middle_balls = [
                Sphere::new_static(
                    center
                        + Vec3::new(
                            rng.gen_range(radius * 1.1..radius * 1.3),
                            rng.gen_range(-5.0..5.0),
                            rng.gen_range(radius * 1.1..radius * 1.3),
                        ),
                    rng.gen_range(3.0..5.0) * zoom,
                    DiffuseLight::<SolidColor>::new_color(
                        rand_color_gen(0.3, 1.0, 0.2, 0.6, 0.3, 1.0) * rng.gen_range(2.0..2.5),
                    ),
                ),
                Sphere::new_static(
                    center
                        + Vec3::new(
                            -rng.gen_range(radius * 1.1..radius * 1.3),
                            rng.gen_range(-5.0..5.0),
                            rng.gen_range(radius * 1.1..radius * 1.3),
                        ),
                    rng.gen_range(3.0..5.0) * zoom,
                    DiffuseLight::<SolidColor>::new_color(
                        rand_color_gen(0.3, 1.0, 0.2, 0.6, 0.3, 1.0) * rng.gen_range(2.0..2.5),
                    ),
                ),
                Sphere::new_static(
                    center
                        + Vec3::new(
                            rng.gen_range(radius * 1.1..radius * 1.3),
                            rng.gen_range(-5.0..5.0),
                            -rng.gen_range(radius * 1.1..radius * 1.3),
                        ),
                    rng.gen_range(3.0..5.0) * zoom,
                    DiffuseLight::<SolidColor>::new_color(
                        rand_color_gen(0.3, 1.0, 0.2, 0.6, 0.3, 1.0) * rng.gen_range(2.0..2.5),
                    ),
                ),
                Sphere::new_static(
                    center
                        + Vec3::new(
                            -rng.gen_range(radius * 1.1..radius * 1.3),
                            rng.gen_range(-5.0..5.0),
                            -rng.gen_range(radius * 1.1..radius * 1.3),
                        ),
                    rng.gen_range(3.0..5.0) * zoom,
                    DiffuseLight::<SolidColor>::new_color(
                        rand_color_gen(0.3, 1.0, 0.2, 0.6, 0.3, 1.0) * rng.gen_range(4.0..5.0),
                    ),
                ),
            ];
            for mb in middle_balls {
                add_glass_outfit(&mut sky_world, &mb);
                lights.add(Box::new(Sphere::new_static(
                    mb.center1,
                    mb.radius,
                    m.clone(),
                )));
                sky_world.add(Box::new(mb));
            }
        }
    }
    world.add(Box::new(BvhNode::new_list(sky_world)));

    let clouds = Metal::new_tex(ImageTexture::new_path("source/normalmaps/clouds.jpg"), 0.2);
    let clouds_nmap = MapMap::new("source/normalmaps/clouds.png");
    let apple = Lambertian::new_tex(ImageTexture::new_path("source/normalmaps/sourapple.jpg"));
    let apple_nmap = MapMap::new("source/normalmaps/sourapple.png");

    let box1 = cube(
        Point3::default(),
        Point3::new(45.0, 45.0, 45.0),
        apple.clone(),
        apple_nmap.clone(),
    );
    let box1 = RotateY::new(box1, 30.0);
    let box1 = Translate::new(box1, Vec3::new(-120.0, -6.0, -100.0));
    world.add(Box::new(box1));

    let box1 = cube(
        Point3::default(),
        Point3::new(35.0, 35.0, 35.0),
        apple,
        apple_nmap,
    );
    let box1 = RotateY::new(box1, -10.0);
    let box1 = Translate::new(box1, Vec3::new(-40.0, -3.0, -100.0));
    world.add(Box::new(box1));

    let box2 = cube(
        Point3::default(),
        Point3::new(30.0, 30.0, 30.0),
        clouds.clone(),
        clouds_nmap,
    );
    let box2 = RotateY::new(box2, 30.0);
    let box2 = Translate::new(box2, Vec3::new(15.0, -2.0, -70.0));
    world.add(Box::new(box2));

    let box2 = cube(
        Point3::default(),
        Point3::new(20.0, 20.0, 20.0),
        clouds,
        origin_nmap,
    );
    let box2 = RotateY::new(box2, -10.0);
    let box2 = Translate::new(box2, Vec3::new(45.0, -2.0, -40.0));
    world.add(Box::new(box2));

    let lighting = Sphere::new_static(
        Point3::new(35.0, 0.0, -35.0),
        3.0,
        DiffuseLight::<SolidColor>::new_color(Color::new(5.0, 5.0, 5.0)),
    );
    lights.add(Box::new(Sphere::new_static(
        lighting.center1,
        lighting.radius,
        m.clone(),
    )));
    world.add(Box::new(lighting));

    let lighting = Sphere::new_static(
        Point3::new(25.0, 0.0, -45.0),
        2.0,
        DiffuseLight::<SolidColor>::new_color(Color::new(5.0, 5.0, 5.0)),
    );
    lights.add(Box::new(Sphere::new_static(
        lighting.center1,
        lighting.radius,
        m.clone(),
    )));
    world.add(Box::new(lighting));

    let lighting = Sphere::new_static(
        Point3::new(40.0, 0.0, -15.0),
        1.5,
        DiffuseLight::<SolidColor>::new_color(Color::new(5.0, 5.0, 5.0)),
    );
    lights.add(Box::new(Sphere::new_static(
        lighting.center1,
        lighting.radius,
        m,
    )));
    world.add(Box::new(lighting));

    let mut cam = Camera::default();

    cam.aspect_ratio = ASPECT_RATIO;
    cam.image_width = IMAGE_WIDTH;
    cam.samples_per_pixel = SAMPLES_PER_PIXEL;
    cam.max_recurse_depth = MAX_RECURSE_DEPTH;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(45.0, 60.0, 200.0);
    cam.lookat = Point3::new(0.0, 20.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.initialize();

    let back_ground = BackGround::new(Arc::new(ImageTexture::new_path("source/background.jpg")));
    // let back_ground = BackGround::new(Arc::new(SolidColor::new_color(Color::default())));

    (world, lights, cam, back_ground)
}

pub fn rand_color_gen(rl: f64, rr: f64, gl: f64, gr: f64, bl: f64, br: f64) -> Color {
    let mut rng = rand::thread_rng();
    Color::new(
        rng.gen_range(rl..rr),
        rng.gen_range(gl..gr),
        rng.gen_range(bl..br),
    )
}

pub fn add_glass_outfit(world: &mut HittableList, sphere: &Sphere<DiffuseLight<SolidColor>>) {
    // 要不要中空呢？要的要的
    let mat = Dielectric::new(1.5);
    let in_mat = Dielectric::new(1.0 / 1.5);
    let outfit = Sphere::new_static(sphere.center1, sphere.radius + 0.8, mat);
    let innerfit = Sphere::new_static(sphere.center1, sphere.radius + 0.3, in_mat);
    world.add(Box::new(outfit));
    world.add(Box::new(innerfit));
}
