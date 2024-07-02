// Paint the surface

use crate::utils::color::Color;
use crate::utils::vec3::Point3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> &Color;
}

pub struct SolidColor {
    albedo: Color,
}

#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
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
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> &Color {
        &self.albedo
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new_color(c1)),
            odd: Arc::new(SolidColor::new_color(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> &Color {
        let (xi, yi, zi) = (
            (self.inv_scale * p.x()).floor() as i32,
            (self.inv_scale * p.y()).floor() as i32,
            (self.inv_scale * p.z()).floor() as i32,
        );
        // is_even?
        return if (xi + yi + zi) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        };
    }
}
