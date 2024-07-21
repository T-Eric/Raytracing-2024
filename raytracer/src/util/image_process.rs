use image::{GenericImageView, RgbImage};
use std::path::Path;
pub fn process_pixels(image_path: &str) -> (u32, u32, RgbImage) {
    let image_path = image_path.to_string();
    let search_paths = [
        ".",
        "source",
        "../source",
        "../..",
        "../../source",
        "../../../..",
        "../../../source",
    ];
    let mut found = false;
    let mut width: u32 = 100;
    let mut height: u32 = 100;
    let mut pixels = image::ImageBuffer::new(width, height);
    for path in search_paths.iter() {
        let fullpath = Path::new(path).join(&image_path);
        if let Ok(img) = image::open(&fullpath) {
            (width, height) = img.dimensions();
            // use par_chunk to ensure each thread process 3 u8 value (1 full pixel)
            pixels = img.to_rgb8();
            found = true;
            break;
        }
    }
    if !found {
        eprintln!("Cannot open file: '{}'...", image_path);
    }
    (width, height, pixels)
}
