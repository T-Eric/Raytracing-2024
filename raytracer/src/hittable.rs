pub mod bvh;
pub mod hittable_list;
pub mod instances;
pub mod transforms;

use crate::{
    materials::Material,
    util::{aabb::Aabb, interval::Interval, ray::Ray, vec3::*},
};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
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
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool;
    fn bounding_box(&self) -> &Aabb; //caution: a ref!
    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
