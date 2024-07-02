use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3, *};
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Clone for HitRecord {
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p,
            normal: self.normal,
            mat: self.mat.clone(),
            t: self.t,
            u: self.u,
            v: self.v,
            front_face: self.front_face,
        }
    }
}

impl HitRecord {
    pub fn new(
        p: &Point3,
        normal: &Vec3,
        mat: Arc<dyn Material>,
        t: f64,
        // u:f64,
        // v:f64, // hit position
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            p: *p,
            normal: *normal,
            mat,
            t,
            u: 0.0,
            v: 0.0,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb; //caution: a ref!
}
