use crate::utils::image_process::process_pixels;
use crate::utils::onb::Onb;
use crate::utils::vec3::Vec3;
use image::RgbImage;

// normal mapping
pub trait NormalMap: Send + Sync {
    fn modify_normal(&self, (_u, _v): (u32, u32), wtb: Onb) -> Vec3 {
        *wtb.w()
    } // 需要外界根据自己的normal(w)和x轴(t)传入坐标系
    fn convert(&self, (_u0, _v0): (f64, f64), (_w, _h): (f64, f64)) -> (u32, u32) {
        (0, 0)
    }
}

// do nothing
#[derive(Default, Copy, Clone)]
pub struct OriginMap {}

impl NormalMap for OriginMap {}

pub struct MapMap {
    width: u32,
    height: u32,
    nmap: RgbImage,
}

impl MapMap {
    pub fn new(image_path: &str) -> MapMap {
        let (width, height, nmap) = process_pixels(image_path);
        MapMap {
            width,
            height,
            nmap,
        }
    }
}

impl NormalMap for MapMap {
    fn modify_normal(&self, (u, v): (u32, u32), wtb: Onb) -> Vec3 {
        // 法线贴图的每个色块对应(-1,1)的偏移，以左上角为原点，蓝色指向我们，红色指右，绿色指下
        // 传入的onb必须以x指右，y指下
        let pixel = self.nmap.get_pixel(u, v);
        let (r, g, b) = (
            (pixel[0] as f64 / 255.999) * 2.0 - 1.0,
            (pixel[1] as f64 / 255.999) * 2.0 - 1.0,
            (pixel[2] as f64 / 255.999) * 2.0 - 1.0,
        ); //(-1,1)
        wtb.local_vec(&Vec3::new(r, g, b))
    }
    fn convert(&self, (u0, v0): (f64, f64), (w, h): (f64, f64)) -> (u32, u32) {
        // do not change the Origin Point
        (
            (u0 / w * self.width as f64) as u32,
            self.height - 1 - (v0 / h * self.height as f64) as u32, // 为什么会倒反天罡呢？
        )
    }
}
