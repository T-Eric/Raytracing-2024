use image::GenericImageView;
use rayon::prelude::*;
use std::path::Path;

pub fn _process_pixels(image_path: &str) -> (u32, u32, Vec<(u8, u8, u8)>) {
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
    for path in search_paths.iter() {
        let fullpath = Path::new(path).join(&image_path);
        if let Ok(img) = image::open(&fullpath) {
            let (width, height) = img.dimensions();
            // use par_chunk to ensure each thread process 3 u8 value (1 full pixel)
            let pixels: Vec<(u8, u8, u8)> = img
                .to_rgb8()
                .par_chunks(3)
                .map(|chunk| (chunk[0], chunk[1], chunk[2]))
                .collect();
            return (width, height, pixels);
        }
    }
    (0, 0, Vec::new())
}
