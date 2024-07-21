pub mod cosine_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;
pub mod sphere_pdf;

use crate::util::vec3::*;
pub trait Pdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::default()
    }
}
