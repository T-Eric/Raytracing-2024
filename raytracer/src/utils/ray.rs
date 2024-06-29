use crate::utils::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Clone for Ray {
    fn clone(&self) -> Self {
        Self::_copy(self)
    }
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }
    pub fn _copy(other: &Ray) -> Self {
        Ray {
            orig: other.orig.clone(),
            dir: other.dir.clone(),
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    // at time t, where is it (dir is the speed)
    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + (self.dir.clone() * t)
    }
}
