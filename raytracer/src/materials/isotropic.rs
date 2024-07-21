use crate::{
    hittable::HitRecord,
    materials::{Material, ScatterRecord},
    pdf::sphere_pdf::SpherePdf,
    textures::{SolidColor, Texture},
    util::{color::Color, ray::Ray, PI},
};

pub struct Isotropic {
    tex: SolidColor,
}

impl Isotropic {
    pub fn new_color(albedo: Color) -> Isotropic {
        Isotropic {
            tex: SolidColor::new_color(albedo),
        }
    }
    pub fn _new_tex(tex: SolidColor) -> Self {
        Isotropic { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        *srec = Some(ScatterRecord::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            Some(Box::new(SpherePdf {})),
            false,
            Ray::default(),
        ));
        true
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
