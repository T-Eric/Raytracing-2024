use crate::utils::aabb::Aabb;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::vec3::Point3;
use crate::utils::vec3::{cross, dot, unit_vector, Vec3};
use std::sync::Arc;

pub trait Flat {
    fn set_bounding_box(&mut self);
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool; // judge if a ray hitting flat hits object
}

pub struct Quad {
    q: Point3, // one diagonal
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64, //in Ax+By+Cz=D
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Quad {
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);
        let mut ret = Quad {
            q,
            u,
            v,
            w,
            mat,
            bbox: Aabb::default(),
            normal,
            d,
        };
        Self::set_bounding_box(&mut ret);
        ret
    }
}
impl Flat for Quad {
    fn set_bounding_box(&mut self) {
        self.bbox = Aabb::new_aabb(
            &Aabb::new_diagonal(self.q, self.q + self.u + self.v),
            &Aabb::new_diagonal(self.q + self.u, self.q + self.v),
        );
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // judge if the intersection point is in the quad
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            false
        } else {
            rec.u = a;
            rec.v = b;
            true
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = dot(&self.normal, r.direction());
        if denom.abs() < 1e-8 {
            None
        }
        //no hit on plane
        else {
            let t = (self.d - dot(&self.normal, r.origin())) / denom;
            if !ray_t.contains(t) {
                None // ray itself actually don't reach
            } else {
                let intersection = r.at(t); // hit flat, may not hit quad
                let planar_hitpt_vec = intersection - self.q; //the Q->P
                let alpha = dot(&self.w, &cross(&planar_hitpt_vec, &self.v));
                let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vec));
                let mut rec =
                    HitRecord::new(&intersection, &self.normal, self.mat.clone(), t, false);
                if !Self::is_interior(alpha, beta, &mut rec) {
                    None
                } else {
                    rec.set_face_normal(r, self.normal);
                    Some(rec)
                }
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
