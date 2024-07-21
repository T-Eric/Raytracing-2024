use crate::{
    textures::Texture,
    util::{color::Color, image_process::process_pixels, interval::Interval, vec3::*},
};
use image::RgbImage;

#[derive(Clone)]
pub struct ImageTexture {
    image_width: u32,
    image_height: u32,
    image_pixels: RgbImage,
}

impl ImageTexture {
    pub fn new_path(image_path: &str) -> ImageTexture {
        let (image_width, image_height, image_pixels) = process_pixels(image_path);
        ImageTexture {
            image_width,
            image_height,
            image_pixels,
        }
    }
    pub fn _new_image(image_pixels: RgbImage) -> ImageTexture {
        let (image_width, image_height) = image_pixels.dimensions();
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

        let i = ((u * self.image_width as f64) as u32).clamp(0, self.image_width - 1);
        let j = ((v * self.image_height as f64) as u32).clamp(0, self.image_height - 1);
        let pixel = self.image_pixels.get_pixel(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
