#[allow(dead_code)]
use crate::utils::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }
    pub fn _zero() -> Self {
        Ray {
            orig: Point3::zero(),
            dir: Vec3::zero(),
        }
    }
    pub fn _copy(other: &Ray) -> Self {
        Ray {
            orig: other.orig.clone(),
            dir: other.dir.clone(),
        }
    }
    pub fn _origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    // at time t, where is it (dir is the speed)
    pub fn _at(&self, t: f64) -> Point3 {
        self.orig.clone() + (self.dir.clone() * t)
    }
}
