use crate::utils::color::{put_color, Color};
use crate::utils::edge_detect::*;
use crate::utils::hittable::Hittable;
use crate::utils::hittable_list::HittableList;
use crate::utils::interval::Interval;
use crate::utils::pdf::{HittablePdf, MixturePdf, Pdf};
use crate::utils::ray::Ray;
use crate::utils::utility::{degrees_to_radians, INFINITY};
use crate::utils::vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3};
use image::{self, Rgb};
use indicatif::ProgressBar;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Camera {
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    sqrt_spp: i32,            // sqrt of samples per pixel
    recip_sqrt_spp: f64,      // 1/sqrt_spp
    u: Vec3,
    v: Vec3,
    w: Vec3, //Camera frame basis vecs
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    pub image_width: i32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_recurse_depth: i32,
    pub background: Color,

    pub vfov: f64, //vertical view angle
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from lookfrom to focus plane

    pub gauss_fuzzing_scale: f64, // do not fuzz if 0.0
    pub edge_detect: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 100,
            max_recurse_depth: 50,
            background: Color::default(),
            vfov: 90.0,
            lookfrom: Point3::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: -1,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_samples_scale: -1.0,
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
            gauss_fuzzing_scale: 0.0,
            edge_detect: false,
        }
    }
}

impl Camera {
    // must multi-thread it!
    pub fn render(&mut self, world: HittableList, lights: Arc<dyn Hittable>, savefile: String) {
        self.initialize();
        // Check environment param
        let progress_bar = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let progress_bar = Arc::new(Mutex::new(progress_bar));
        let imgbuf = image::RgbImage::new(self.image_width as u32, self.image_height as u32);
        let imgbuf = Arc::new(Mutex::new(imgbuf));
        let mut thread_lines = vec![];

        for j in 0..self.image_height {
            let progress_bar = Arc::clone(&progress_bar);
            let imgbuf = Arc::clone(&imgbuf);
            let world = world.clone(); //inside, thus don't consume the param world
            let lights = lights.clone();
            let self_copy = CameraCopy::new(self);
            let image_width = self.image_width;
            let thread_line = thread::spawn(move || {
                for i in 0..image_width {
                    let mut pixel_color = Color::default();

                    for s_j in 0..self_copy.sqrt_spp {
                        for s_i in 0..self_copy.sqrt_spp {
                            let r = self_copy.get_ray(i, j, s_i, s_j);
                            pixel_color += self_copy.ray_color(
                                &r,
                                self_copy.max_recurse_depth,
                                &world,
                                lights.clone(),
                            );
                        }
                    }
                    pixel_color *= self_copy.pixel_samples_scale;

                    let mut imgbuf = imgbuf.lock().unwrap();
                    let pixel = imgbuf.get_pixel_mut(i as u32, j as u32);
                    let (r, g, b) = put_color(&pixel_color);
                    *pixel = Rgb([r as u8, g as u8, b as u8]);

                    let progress_bar = progress_bar.lock().unwrap();
                    progress_bar.inc(1);
                }
            }); //self escapes the method body here, all outside params must copy here!
            thread_lines.push(thread_line);
        }

        for thread_line in thread_lines {
            thread_line.join().unwrap();
        }
        progress_bar.lock().unwrap().finish();
        let imgbuf = Arc::try_unwrap(imgbuf).unwrap().into_inner().unwrap();

        // edge detection
        if self.edge_detect {
            let fuzz_img = gauss_fuzzing(&imgbuf, self.gauss_fuzzing_scale);
            let sketch_img = edge_detecting(&fuzz_img);
            let imgbuf = combination(&imgbuf, sketch_img);
            imgbuf.save(savefile).unwrap()
        } else if self.gauss_fuzzing_scale > 0.001 {
            let imgbuf = gauss_fuzzing(&imgbuf, self.gauss_fuzzing_scale);
            imgbuf.save(savefile).unwrap()
        } else {
            imgbuf.save(savefile).unwrap()
        }
    }
}

// Seems that I must make a lite copy struct here, and move some funcs away
struct CameraCopy {
    pixel_samples_scale: f64,
    sqrt_spp: i32,
    recip_sqrt_spp: f64,
    max_recurse_depth: i32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    pub background: Color,
}

impl CameraCopy {
    pub fn new(camera: &Camera) -> Self {
        Self {
            pixel_samples_scale: camera.pixel_samples_scale,
            sqrt_spp: camera.sqrt_spp,
            recip_sqrt_spp: camera.recip_sqrt_spp,
            max_recurse_depth: camera.max_recurse_depth,
            pixel00_loc: camera.pixel00_loc,
            pixel_delta_u: camera.pixel_delta_u,
            pixel_delta_v: camera.pixel_delta_v,
            center: camera.center,
            defocus_angle: camera.defocus_angle,
            defocus_disk_u: camera.defocus_disk_u,
            defocus_disk_v: camera.defocus_disk_v,
            background: camera.background,
        }
    }
}

impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.center = Vec3::copy(&self.lookfrom);

        // Determine viewport dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let view_height = 2.0 * h * self.focus_dist;
        let view_width = view_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical edges
        let view_u = self.u * view_width;
        let view_v = -&self.v * view_height;

        // Calc the delta per pixel
        self.pixel_delta_u = view_u / self.image_width as f64;
        self.pixel_delta_v = view_v / self.image_height as f64;

        let view_upper_left = self.center - self.w * self.focus_dist - view_u / 2.0 - view_v / 2.0;
        self.pixel00_loc = view_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calc the defocus disk basis vecs
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
}

impl CameraCopy {
    // Construct camera ray from the origin point and directed at randomly sampled
    // point around the pixel (i,j).
    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        let mut rng = rand::thread_rng();
        // new changes: we stratify one pixel, and make the samples
        // distribute more averagely
        let offset = self.sample_square_stratified(s_i, s_j, &mut rng);
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rng.gen_range(0.0..1.0); //send rays in a shutter period

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    //Returns a random point in the camera defocus disk
    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y()
    }
    fn ray_color(
        &self,
        r: &Ray,
        depth: i32,
        world: &HittableList,
        lights: Arc<dyn Hittable>,
    ) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001, INFINITY)) {
            let emission_color = rec.mat.emitted(r, &rec, rec.u, rec.v, &rec.p);
            if let Some(srec) = rec.mat.scatter(r, &rec) {
                if srec.skip_pdf {
                    // do not need to consider scatter
                    return srec.attenuation
                        * self.ray_color(&srec.skip_pdf_ray, depth - 1, world, lights.clone());
                }

                let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec.p));
                let p = MixturePdf::new(light_ptr, srec.pdf_ptr);
                let scattered = Ray::new(rec.p, p.generate(), r.time());
                let pdf_val = p.value(scattered.direction());
                // 原因：p.generate出现nan，是因origin在球里面

                let scatter_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);

                let sample_color = self.ray_color(&scattered, depth - 1, world, lights.clone());
                let scatter_color = (sample_color * srec.attenuation * scatter_pdf) / pdf_val;

                emission_color + scatter_color
            } else {
                emission_color
            }
        } else {
            // self.background
            // old background
            let unit_direction = unit_vector(r.direction()) + self.background;
            let a = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
        }
    }

    fn sample_square_stratified(&self, s_i: i32, s_j: i32, rng: &mut ThreadRng) -> Vec3 {
        // return a vec to the specified sub-pixel
        let px = (s_i as f64 + rng.gen_range(0.0..1.0)) * self.recip_sqrt_spp - 0.5;
        let py = (s_j as f64 + rng.gen_range(0.0..1.0)) * self.recip_sqrt_spp - 0.5;
        Vec3::new(px, py, 0.0)
    }
}

// A vector to a random point in [-0.5,-0.5]~[0.5,0.5] unit square.
fn _sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(0.0..1.0) - 0.5,
        rng.gen_range(0.0..1.0) - 0.5,
        0.0,
    )
}
