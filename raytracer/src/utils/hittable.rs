use crate::utils::interval::Interval;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3, *};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: -1.0,
            front_face: false,
        }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        Self::copy(self)
    }
}

impl HitRecord {
    pub fn _new(p: &Point3, normal: &Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            p: p.clone(),
            normal: normal.clone(),
            t,
            front_face,
        }
    }

    pub fn copy(other: &HitRecord) -> HitRecord {
        HitRecord {
            p: Point3::copy(&other.p),
            normal: Point3::copy(&other.normal),
            t: other.t,
            front_face: other.front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
