pub mod aabb;
pub mod color;
pub mod image_process;
pub mod interval;
pub mod onb;
pub mod perlin;
pub mod ray;
pub mod vec3;

// some constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
// Image
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u32 = 1920;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 625;
pub const MAX_RECURSE_DEPTH: i32 = 50;
// Threads
pub const THREAD_NUM: usize = 20;

//some utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub struct OutputParam<'a> {
    pub enable_edge_detect: bool,
    pub savedir: &'a str,
    pub savefile: &'a str,
}
