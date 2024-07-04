use crate::utils::color::Color;
use crate::utils::hittable::HitRecord;
// use crate::utils::onb::Onb;
use crate::utils::ray::Ray;
use crate::utils::texture::SolidColor;
use crate::utils::texture::Texture;
use crate::utils::utility::PI;
use crate::utils::vec3::{
    dot, random_on_hemisphere, random_unit_vector, reflect, refract, unit_vector, Point3, Vec3,
};
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        None
    } // attenuation and scattered
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

// Lambertian material which always do fraction reflection
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

// Metal, full reflection
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

pub struct Dielectric {
    refraction_index: f64,
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_color(albedo: Color) -> Self {
        Lambertian {
            tex: Arc::new(SolidColor::new_color(albedo)),
        }
    }
    pub fn new_arc(tex: Arc<dyn Texture>) -> Self {
        Lambertian { tex }
    }
}

impl Material for Lambertian {
    // * from vector
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        // let mut scatter_direction = rec.normal + random_unit_vector();

        // use hemisphere, not full sphere scattering
        let mut scatter_direction = random_on_hemisphere(&rec.normal);

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        Some((attenuation, scattered, 0.0))
    }

    // * from orthonormal basis
    // fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
    //     let mut uvw = Onb::default();
    //     uvw.build_from_w(&rec.normal);
    //     let scatter_direction = uvw.local_vec(&random_cosine_direction());
    //
    //     let scattered = Ray::new(rec.p, unit_vector(&scatter_direction), r_in.time());
    //     let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
    //     let pdf = dot(uvw.w(), scattered.direction()) / PI;
    //     Some((attenuation, scattered, pdf))
    // }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * PI)
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        // fuzz operation
        reflected = unit_vector(&reflected) + (random_unit_vector() * self.fuzz);
        let scattered = Ray::new(rec.p, reflected, r_in.time());
        let attenuation = self.albedo;

        if dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered, 0.0))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for Fresnel reflection
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        //consider full reflection
        let cannot_reflect = ri * sin_theta > 1.0;
        let direction: Vec3 =
            if cannot_reflect || self.reflectance(cos_theta, ri) > rand::random::<f64>() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        let scattered = Ray::new(rec.p, direction, r_in.time());

        Some((attenuation, scattered, 0.0))
    }
}

impl DiffuseLight {
    pub fn _new_arc(tex: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { tex }
    }
    pub fn new_color(emit: Color) -> DiffuseLight {
        DiffuseLight {
            tex: Arc::new(SolidColor::new_color(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

impl Isotropic {
    pub fn new_color(albedo: Color) -> Isotropic {
        Isotropic {
            tex: Arc::new(SolidColor::new_color(albedo)),
        }
    }
    pub fn _new_texture(tex: Arc<dyn Texture>) -> Isotropic {
        Isotropic { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let scattered = Ray::new(rec.p, random_unit_vector(), r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered, 1.0 / (4.0 * PI)))
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
