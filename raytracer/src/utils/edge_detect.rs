use crate::utils::utility::PI;
use image::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn gauss_fuzzing(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    sigma: f64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    // gauss matrix
    let mut matrix = [[0.0; 3]; 3];
    let mut sum = 0.0;
    for (i, layer) in matrix.iter_mut().enumerate() {
        for (j, cell) in layer.iter_mut().enumerate() {
            let exponent =
                ((i as f64 - 1.0).powi(2) + (j as f64 - 1.0).powi(2)) / (2.0 * sigma.powi(2));
            *cell = (-1.0 * exponent).exp() / (2.0 * PI * sigma.powi(2));
            sum += *cell;
        }
    }
    // Normalize the kernel
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            *cell /= sum;
        }
    }

    // Create a new image buffer for the result
    let blurred_sketch = Arc::new(Mutex::new(ImageBuffer::new(width, height)));

    let mut handles = vec![];
    for y in 0..height {
        let img_clone = img.clone();
        let blurred_sketch_clone = Arc::clone(&blurred_sketch);
        let handle = thread::spawn(move || {
            for x in 0..width {
                let mut sum_rgb = [0.0; 3];
                let mut weight_sum = 0.0;

                for i in 0..3 {
                    for j in 0..3 {
                        let nx = x as i32 - 1 + i;
                        let ny = y as i32 - 1 + j;

                        // 应用高斯边界条件
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let kernel_weight = matrix[i as usize][j as usize];
                            let kernel_pixel = img_clone[(nx as u32, ny as u32)];
                            sum_rgb[0] += kernel_pixel[0] as f64 * kernel_weight;
                            sum_rgb[1] += kernel_pixel[1] as f64 * kernel_weight;
                            sum_rgb[2] += kernel_pixel[2] as f64 * kernel_weight;
                            weight_sum += kernel_weight;
                        }
                    }
                }

                let mut blurred_sketch_mut = blurred_sketch_clone.lock().unwrap();
                let pixel: &mut Rgb<u8> = blurred_sketch_mut.get_pixel_mut(x, y);
                pixel[0] = (sum_rgb[0] / weight_sum) as u8;
                pixel[1] = (sum_rgb[1] / weight_sum) as u8;
                pixel[2] = (sum_rgb[2] / weight_sum) as u8;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(blurred_sketch)
        .ok()
        .unwrap()
        .into_inner()
        .unwrap()
}

pub fn edge_detecting(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let mut gray_img = ImageBuffer::new(width + 2, height + 2);
    for (x, y, pixel) in img.enumerate_pixels() {
        let gray = Luma([(0.299 * pixel[0] as f64
            + 0.587 * pixel[1] as f64
            + 0.114 * pixel[2] as f64) as u8]);
        gray_img.put_pixel(x + 1, y + 1, gray);
    }
    let gray_img = Arc::new(gray_img);

    let sobel_kernel = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    let sketch_img = Arc::new(Mutex::new(ImageBuffer::new(width, height)));

    let handles: Vec<_> = (1..height + 1)
        .map(|y| {
            let gray_img = gray_img.clone();
            let sketch_img = sketch_img.clone();
            thread::spawn(move || {
                for x in 1..width + 1 {
                    let mut gx = 0;
                    let mut gy = 0;
                    for i in 0..sobel_kernel.len() {
                        for j in 0..sobel_kernel[i].len() {
                            let pixel = gray_img.get_pixel(x + j as u32 - 1, y + i as u32 - 1).0[0];
                            gx += pixel as i32 * sobel_kernel[j][i];
                            gy += pixel as i32 * sobel_kernel[i][j];
                        }
                    }
                    let mut mag = ((gx * gx + gy * gy) as f64).sqrt().round() as u8;
                    if mag < 60 {
                        mag = 0;
                    }
                    mag = if mag as f64 * 1.2 > 255.999 {
                        255
                    } else {
                        (mag as f64).floor() as u8
                    };
                    let mut sketch_img_mut = sketch_img.lock().unwrap();
                    sketch_img_mut.put_pixel(x - 1, y - 1, Luma([mag])); // a bit of advance
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Arc::try_unwrap(sketch_img)
        .expect("Mutex was cloned more than once")
        .into_inner()
        .expect("ImageBuffer was dropped elsewhere")
}

pub fn combination(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    edge: ImageBuffer<Luma<u8>, Vec<u8>>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut combined_img = ImageBuffer::new(width, height);

    for (x, y, pixel) in edge.enumerate_pixels() {
        let alpha = (255 - pixel.0[0]) as f64;
        let bg_pixel = img.get_pixel(x, y);
        // Alpha blending
        let combined_pixel = Rgb([
            (bg_pixel[0] as f64 * alpha / 255.000) as u8,
            (bg_pixel[1] as f64 * alpha / 255.000) as u8,
            (bg_pixel[2] as f64 * alpha / 255.000) as u8,
        ]);
        combined_img.put_pixel(x, y, combined_pixel);
    }

    combined_img
}
