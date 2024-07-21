use crate::{
    hittable::HitRecord,
    materials::{Material, ScatterRecord},
    pdf::cosine_pdf::CosinePdf,
    textures::{SolidColor, Texture},
    util::{color::Color, ray::Ray, vec3::*, PI},
};

#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    tex: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new_color(albedo: Color) -> Lambertian<SolidColor> {
        Lambertian {
            tex: SolidColor::new_color(albedo),
        }
    }
    pub fn new_tex(tex: T) -> Self {
        Lambertian { tex }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        *srec = Some(ScatterRecord::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            Some(Box::new(CosinePdf::new(rec.normal))),
            false,
            Ray::default(),
        ));
        true
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(scattered.direction()));
        (cos_theta / PI).max(0.0)
    }
}
