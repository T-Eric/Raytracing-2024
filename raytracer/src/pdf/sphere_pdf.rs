// uniform density
use crate::{
    pdf::Pdf,
    util::{vec3::*, PI},
};

pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}
