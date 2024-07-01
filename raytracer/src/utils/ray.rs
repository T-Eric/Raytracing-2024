use crate::utils::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Clone for Ray {
    fn clone(&self) -> Self {
        Self::copy(self)
    }
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, tm: f64) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
            tm,
        }
    }
    pub fn _new_0(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin.clone(),
            dir: direction.clone(),
            tm: 0.0,
        }
    }
    pub fn copy(other: &Ray) -> Self {
        Ray {
            orig: other.orig.clone(),
            dir: other.dir.clone(),
            tm: other.tm,
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }

    // at time t, where is it (dir is the speed)
    pub fn at(&self, t: f64) -> Point3 {
        self.orig.clone() + (self.dir.clone() * t)
    }
}
