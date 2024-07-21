#[allow(dead_code)]
mod camera;
#[allow(dead_code)]
mod features;
#[allow(dead_code)]
mod hittable;
#[allow(dead_code)]
mod materials;
#[allow(dead_code)]
mod pdf;
#[allow(dead_code)]
mod textures;
#[allow(dead_code)]
mod util;
#[allow(dead_code)]
mod worlds;

use crate::camera::background::BackGround;
use crate::camera::Camera;
use crate::features::edge_detect::combination;
use crate::features::edge_detect::edge_detecting;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::Hittable;
use crate::pdf::hittable_pdf::HittablePdf;
use crate::pdf::Pdf;
use crate::util::color::{put_color, Color};
use crate::util::interval::Interval;
use crate::util::ray::Ray;
use crate::util::{
    OutputParam, IMAGE_HEIGHT, IMAGE_WIDTH, INFINITY, MAX_RECURSE_DEPTH, THREAD_NUM,
};
use crate::worlds::final_scene::final_scene;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

fn ray_color(
    r: &Ray,
    depth: i32,
    world: &HittableList,
    lights: &HittableList,
    background: Color,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let mut rec = None;
    if !world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
        return background;
    }
    let rec = if let Some(data) = rec {
        data
    } else {
        panic!("No hit record!");
    };
    let emission_color = rec.mat.emitted(r, &rec, rec.u, rec.v, &rec.p);
    let mut srec = None;
    if !rec.mat.scatter(r, &rec, &mut srec) {
        return emission_color;
    }
    let srec = if let Some(data) = srec {
        data
    } else {
        panic!("No scatter record!");
    };
    if srec.skip_pdf {
        // do not need to consider scatter
        return srec.attenuation
            * ray_color(&srec.skip_pdf_ray, depth - 1, world, lights, background);
    }

    let light_pdf = HittablePdf::new(lights, rec.p);

    let p = (&light_pdf, &*srec.pdf_ptr.expect("No pdf defined!"));
    let mut rng = rand::thread_rng();
    let p_generate = if rng.gen_range(0.0..1.0) < 0.5 {
        p.0.generate()
    } else {
        p.1.generate()
    };
    let scattered = Ray::new(rec.p, p_generate, r.time());
    let pdf_val = 0.5 * p.0.value(scattered.direction()) + 0.5 * p.1.value(scattered.direction());
    // 直接将mixturePdf拆了，要不然dyn无法去掉

    let scatter_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);

    let sample_color = ray_color(&scattered, depth - 1, world, lights, background);
    let scatter_color = (sample_color * srec.attenuation * scatter_pdf) / pdf_val;

    emission_color + scatter_color
}

fn render(
    world: HittableList,
    lights: HittableList,
    cam: Camera,
    back_ground: BackGround,
) -> RgbImage {
    let progress_bar = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64)
    };
    let progress_bar = Arc::new(Mutex::new(progress_bar));

    let img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let img = Arc::new(Mutex::new(img));

    let world = Arc::new(world);
    let lights = Arc::new(lights);
    let cam = Arc::new(cam);
    let back_ground = Arc::new(back_ground);

    let pool = ThreadPool::new(THREAD_NUM);

    for j in 0..IMAGE_HEIGHT {
        let img = Arc::clone(&img);
        let progress_bar = Arc::clone(&progress_bar);
        let world = Arc::clone(&world);
        let lights = Arc::clone(&lights);
        let cam = Arc::clone(&cam);
        let back_ground = Arc::clone(&back_ground);
        pool.execute(move || {
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::default();
                let background_color = back_ground.value(i, j);

                for s_j in 0..cam.sqrt_spp {
                    for s_i in 0..cam.sqrt_spp {
                        let r = cam.get_ray(i, j, s_i, s_j);
                        pixel_color +=
                            ray_color(&r, MAX_RECURSE_DEPTH, &world, &lights, background_color);
                    }
                }
                pixel_color *= cam.pixel_samples_scale;

                let mut img = img.lock().unwrap();
                let pixel = img.get_pixel_mut(i, j);
                let (r, g, b) = put_color(&pixel_color);
                *pixel = Rgb([r as u8, g as u8, b as u8]);

                let progress_bar = progress_bar.lock().unwrap();
                progress_bar.inc(1);
            }
        });
    }
    pool.join();
    progress_bar.lock().unwrap().finish();
    Arc::try_unwrap(img).unwrap().into_inner().unwrap()
}

fn process_and_output(img: ImageBuffer<Rgb<u8>, Vec<u8>>, param: OutputParam) {
    // do edge, gauss and gray here
    let savedir = String::from(param.savedir);
    let savefile = savedir.clone() + &*String::from(param.savefile);

    let dir = Path::new(&savedir);
    if !dir.exists() {
        fs::create_dir_all(dir).expect("Couldn't create directory");
    }

    if param.enable_edge_detect {
        let edged_img = edge_detecting(&img);
        let img = combination(&img, edged_img);
        img.save(savefile).expect("Failed to save!");
        return;
    }

    img.save(savefile).expect("Failed to save!");
}

fn main() {
    let now = Instant::now();

    let (world, lights, cam, back_ground) = final_scene();
    let raw_img = render(world, lights, cam, back_ground);

    let output_param = OutputParam {
        enable_edge_detect: false,
        savefile: "/201.png",
        savedir: "output/book0",
    };

    process_and_output(raw_img, output_param);

    let now = now.elapsed().as_millis();
    eprintln!();
    eprintln!("duration:{:?}ms", now);
}
