use image::GenericImageView;
use rayon::prelude::*;

pub fn process_pixels(image_path: &str) -> (u32, u32, Vec<(u8, u8, u8)>) {
    let img = image::open(image_path).expect("Open image failed!");
    let (width, height) = img.dimensions();

    // use par_chunk to ensure each thread process 3 u8 value (1 full pixel)
    let pixels: Vec<(u8, u8, u8)> = img
        .to_rgb8()
        .par_chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect();

    (width, height, pixels)
}
