// Paint the surface

use crate::utils::color::Color;
use crate::utils::image_process::_process_pixels;
use crate::utils::interval::Interval;
use crate::utils::perlin::Perlin;
use crate::utils::vec3::Point3;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
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

pub struct ImageTexture {
    image_width: u32,
    image_height: u32,
    image_pixels: Vec<(u8, u8, u8)>,
}

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64, // the density
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

impl CheckerTexture {
    pub fn _new(scale: f64, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new_color(c1)),
            odd: Arc::new(SolidColor::new_color(c2)),
        }
    }
}

impl Texture for CheckerTexture {
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

impl ImageTexture {
    pub fn _new(image_path: &str) -> ImageTexture {
        let (image_width, image_height, image_pixels) = _process_pixels(image_path);
        ImageTexture {
            image_width,
            image_height,
            image_pixels,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v); //Flipping v

        let i = (u * self.image_width as f64) as i32;
        let j = (v * self.image_height as f64) as i32;
        let pixel = &self.image_pixels[(j as u32 * self.image_width + i as u32) as usize];

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel.0 as f64,
            color_scale * pixel.1 as f64,
            color_scale * pixel.2 as f64,
        )
    }
}

impl NoiseTexture {
    pub fn _new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::default(),
            scale,
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
        // Color::new(1.0, 1.0, 1.0) * self.noise.turb(p, 7)
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + (p.z() * self.scale + self.noise.turb(p, 7) * 10.0).sin())
    }
}
