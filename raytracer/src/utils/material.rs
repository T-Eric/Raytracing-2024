use crate::utils::color::Color;
use crate::utils::hittable::HitRecord;
use crate::utils::pdf::{CosinePdf, Pdf, SpherePdf};
use crate::utils::ray::Ray;
use crate::utils::texture::SolidColor;
use crate::utils::texture::Texture;
use crate::utils::utility::PI;
use crate::utils::vec3::{dot, random_unit_vector, reflect, refract, unit_vector, Point3};
use rand::Rng;
use std::sync::Arc;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        ScatterRecord {
            attenuation: Color::default(),
            pdf_ptr: Arc::new(SpherePdf {}),
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

impl ScatterRecord {
    pub fn new(
        attenuation: Color,
        pdf_ptr: Arc<dyn Pdf>,
        skip_pdf: bool,
        skip_pdf_ray: Ray,
    ) -> ScatterRecord {
        ScatterRecord {
            attenuation,
            pdf_ptr,
            skip_pdf,
            skip_pdf_ray,
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
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
    tex: Arc<dyn Texture>,
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
    pub fn new_tex(tex: Arc<dyn Texture>) -> Self {
        Lambertian { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            Arc::new(CosinePdf::new(rec.normal)),
            false,
            Ray::default(),
        ))
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(scattered.direction()));
        (cos_theta / PI).max(0.0)
    }
}

impl Metal {
    pub fn new_color(albedo: Color, fuzz: f64) -> Self {
        Metal {
            tex: Arc::new(SolidColor::new_color(albedo)),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
    pub fn new_tex(tex: Arc<dyn Texture>, fuzz: f64) -> Self {
        Metal {
            tex,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        // fuzz operation
        reflected = unit_vector(&reflected) + (random_unit_vector() * self.fuzz);

        Some(ScatterRecord::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            Arc::new(SpherePdf {}),
            true,
            Ray::new(rec.p, reflected, r_in.time()),
        ))
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut rng = rand::thread_rng();
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, ri) > rng.gen_range(0.0..1.0) {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        Some(ScatterRecord::new(
            Color::new(1.0, 1.0, 1.0),
            Arc::new(SpherePdf {}),
            true,
            Ray::new(rec.p, direction, r_in.time()),
        ))
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
    // fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
    //     self.tex.value(u, v, p)
    // }
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.tex.value(u, v, p)
        } else {
            Color::default()
        }
    }
}

impl Isotropic {
    pub fn _new_color(albedo: Color) -> Isotropic {
        Isotropic {
            tex: Arc::new(SolidColor::new_color(albedo)),
        }
    }
    pub fn _new_texture(tex: Arc<dyn Texture>) -> Isotropic {
        Isotropic { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            Arc::new(SpherePdf {}),
            false,
            Ray::default(),
        ))
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
