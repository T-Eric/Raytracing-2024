pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;

use crate::util::{color::Color, vec3::*};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new_color(albedo: Color) -> SolidColor {
        SolidColor { albedo }
    }
    pub fn _new_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}
