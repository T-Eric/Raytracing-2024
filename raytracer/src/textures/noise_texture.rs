use crate::{
    textures::Texture,
    util::{color::Color, perlin::Perlin, vec3::*},
};

#[derive(Default, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64, // the density
    with_turbulence: bool,
    turb_depth: i32,
}

impl NoiseTexture {
    pub fn _new(scale: f64, with_turbulence: bool, turb_depth: i32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::default(),
            scale,
            with_turbulence,
            turb_depth,
        }
    }
}

impl Texture for NoiseTexture {
    // normal perlin fuzzing
    // fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
    //     Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(p * self.scale)))
    // }

    // with turbulence
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        if self.with_turbulence {
            Color::new(0.5, 0.5, 0.5)
                * (1.0 + (p.z() * self.scale + self.noise.turb(p, self.turb_depth) * 10.0).sin())
        } else {
            Color::new(0.5, 0.5, 0.5) * (1.0 + self.noise.noise(&(p * self.scale)))
        }
    }
}
