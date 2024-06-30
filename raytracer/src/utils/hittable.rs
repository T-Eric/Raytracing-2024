use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3, *};
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: None,
            t: -1.0,
            front_face: false,
        }
    }
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p.clone(),
            normal: self.normal.clone(),
            mat: self.mat.clone(),
            t: self.t,
            front_face: self.front_face,
        }
    }
}

impl HitRecord {
    pub fn _new(
        p: &Point3,
        normal: &Vec3,
        mat: Option<Arc<dyn Material>>,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            p: p.clone(),
            normal: normal.clone(),
            mat,
            t,
            front_face,
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
