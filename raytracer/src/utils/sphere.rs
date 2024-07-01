use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::vec3::*;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: center.clone(),
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = &self.center - r.origin();
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
        let outward_normal = (&p - &self.center) / self.radius;
        let mut rec = HitRecord::new(&p, &outward_normal, self.mat.clone(), t, false);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
