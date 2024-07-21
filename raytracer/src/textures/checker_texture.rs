use crate::{
    textures::{SolidColor, Texture},
    util::{color::Color, vec3::*},
};

pub struct CheckerTexture<TO: Texture, TE: Texture> {
    inv_scale: f64,
    even: TE,
    odd: TO,
}

impl<TO: Texture, TE: Texture> CheckerTexture<TO, TE> {
    pub fn _new_color(scale: f64, c1: Color, c2: Color) -> CheckerTexture<SolidColor, SolidColor> {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: SolidColor::new_color(c1),
            odd: SolidColor::new_color(c2),
        }
    }
    pub fn _new_tex(scale: f64, te: TE, to: TO) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: te,
            odd: to,
        }
    }
}

impl<TO: Texture, TE: Texture> Texture for CheckerTexture<TO, TE> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let (xi, yi, zi) = (
            (self.inv_scale * p.x()).floor() as i32,
            (self.inv_scale * p.y()).floor() as i32,
            (self.inv_scale * p.z()).floor() as i32,
        );
        // is_even?
        if (xi + yi + zi) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
