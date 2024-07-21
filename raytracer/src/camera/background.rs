use crate::textures::Texture;
use crate::util::{
    vec3::{Point3, Vec3},
    IMAGE_HEIGHT, IMAGE_WIDTH,
};
use std::sync::Arc;
pub struct BackGround {
    pub tex: Arc<dyn Texture>,
}

impl BackGround {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        BackGround { tex }
    }
    pub fn value(&self, u: u32, v: u32) -> Vec3 {
        self.tex.value(
            u as f64 / IMAGE_WIDTH as f64,
            v as f64 / IMAGE_HEIGHT as f64,
            &Point3::default(),
        )*0.3
    }
}
