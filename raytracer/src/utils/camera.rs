use crate::utils::color::{put_color, Color};
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::hittable_list::HittableList;
use crate::utils::interval::Interval;
use crate::utils::ray::Ray;
use crate::utils::utility::{degrees_to_radians, INFINITY};
use crate::utils::vec3::{cross, unit_vector, Point3, Vec3};
use rand;

pub struct Camera {
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    u: Vec3,
    v: Vec3,
    w: Vec3, //Camera frame basis vecs

    pub image_width: i32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_recurse_depth: i32,
    pub vfov: f64, //vertical view angle
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
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
            image_height: -1,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_samples_scale: -1.0,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();

                //generate samples rays in one pixel to model more real situation
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_recurse_depth, world);
                }
                put_color(&(&pixel_color * self.pixel_samples_scale));
            }
        }
    }
}

//consts
impl Camera {
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Vec3::copy(&self.lookfrom);

        // Determine viewport dimensions
        let focal_length = (&self.lookfrom - &self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let view_height = 2.0 * h * focal_length;
        let view_width = view_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = unit_vector(&(&self.lookfrom - &self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical edges
        let view_u = &self.u * view_width;
        let view_v = -&self.v * view_height;

        // Calc the delta per pixel
        self.pixel_delta_u = &view_u / self.image_width as f64;
        self.pixel_delta_v = &view_v / self.image_height as f64;

        let view_upper_left =
            &self.center - &(&self.w * focal_length) - view_u / 2.0 - view_v / 2.0;
        self.pixel00_loc = view_upper_left + (&self.pixel_delta_u + &self.pixel_delta_v) * 0.5;
    }

    // A vector to a random point in [-0.5,-0.5]~[0.5,0.5] unit square.
    fn sample_square() -> Vec3 {
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    // Construct camera ray from the origin point and directed at randomly sampled
    // point around the pixel (i,j).
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = &self.pixel00_loc
            + &(&self.pixel_delta_u * (i as f64 + offset.x()))
            + (&self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = self.center.clone();
        let ray_direction = &pixel_sample - &ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    //Color painter
    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        let mut rec = HitRecord::default();

        if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            // before 1.6
            // return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
            // 1.9 no Lambertian
            // let direction = random_on_hemisphere(&rec.normal);
            // 1.10 Lambertian
            // let direction = rec.normal + random_unit_vector();
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            return if rec
                .clone()
                .mat
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                attenuation * Self::ray_color(&scattered, depth - 1, world)
            } else {
                Color::default()
            };
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
