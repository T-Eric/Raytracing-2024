use rand::rngs::ThreadRng;
use rand::Rng;

use crate::util::ray::Ray;
use crate::util::{degrees_to_radians, vec3::*, IMAGE_HEIGHT};

pub mod background;

pub struct Camera {
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    pub sqrt_spp: u32,            // sqrt of samples per pixel
    pub recip_sqrt_spp: f64,      // 1/sqrt_spp
    u: Vec3,
    v: Vec3,
    w: Vec3, //Camera frame basis vecs
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    pub image_width: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_recurse_depth: i32,

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
            vfov: 90.0,
            lookfrom: Point3::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 1,
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
    pub fn initialize(&mut self) {
        self.image_height = IMAGE_HEIGHT;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as u32;
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
    pub fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
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
    fn sample_square_stratified(&self, s_i: u32, s_j: u32, rng: &mut ThreadRng) -> Vec3 {
        // return a vec to the specified sub-pixel
        let px = (s_i as f64 + rng.gen_range(0.0..1.0)) * self.recip_sqrt_spp - 0.5;
        let py = (s_j as f64 + rng.gen_range(0.0..1.0)) * self.recip_sqrt_spp - 0.5;
        Vec3::new(px, py, 0.0)
    }
}
