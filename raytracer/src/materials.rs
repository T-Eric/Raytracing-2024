pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::{
    hittable::HitRecord,
    pdf::Pdf,
    util::{color::Color, ray::Ray, vec3::*},
};

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Box<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl ScatterRecord {
    pub fn new(
        attenuation: Color,
        pdf_ptr: Option<Box<dyn Pdf>>,
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
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut Option<ScatterRecord>) -> bool {
        false
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}
