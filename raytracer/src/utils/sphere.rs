use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::vec3::*;
use std::sync::Arc;

pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
}

impl Sphere {
    pub fn new_static(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center1: center.clone(),
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::default(),
        }
    }
    pub fn new_motive(
        center1: &Point3,
        center2: &Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Sphere {
        Sphere {
            center1: center1.clone(),
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center1,
        }
    }
    pub fn sphere_center(&self, time: f64) -> Point3 {
        &self.center1 + &(&self.center_vec * time)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            Self::sphere_center(self, r.time())
        } else {
            self.center1.clone()
        };
        let oc = &center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = h * h - a * c;
        if delta < 0.0 {
            return None;
        }
        let sqrtd = delta.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (&p - &center) / self.radius;
        let mut rec = HitRecord::new(&p, &outward_normal, self.mat.clone(), t, false);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
