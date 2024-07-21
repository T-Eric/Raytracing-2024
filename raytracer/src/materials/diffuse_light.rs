use crate::{
    hittable::HitRecord,
    materials::Material,
    textures::{SolidColor, Texture},
    util::{color::Color, ray::Ray, vec3::*},
};

#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    tex: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn _new_tex(tex: T) -> Self {
        DiffuseLight { tex }
    }
    pub fn new_color(emit: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            tex: SolidColor::new_color(emit),
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.tex.value(u, v, p)
        } else {
            Color::default()
        }
    }
}
