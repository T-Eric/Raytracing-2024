use crate::{
    hittable::HitRecord,
    materials::{Material, ScatterRecord},
    textures::{SolidColor, Texture},
    util::{color::Color, ray::Ray, vec3::*},
};

#[derive(Clone)]
pub struct Metal<T: Texture> {
    tex: T,
    fuzz: f64,
}

impl<T: Texture> Metal<T> {
    pub fn new_color(albedo: Color, fuzz: f64) -> Metal<SolidColor> {
        Metal {
            tex: SolidColor::new_color(albedo),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
    pub fn new_tex(tex: T, fuzz: f64) -> Self {
        Metal {
            tex,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        // fuzz operation
        reflected = unit_vector(&reflected) + (random_unit_vector() * self.fuzz);

        *srec = Some(ScatterRecord::new(
            self.tex.value(rec.u, rec.v, &rec.p),
            None,
            true,
            Ray::new(rec.p, reflected, r_in.time()),
        ));
        true
    }
}
