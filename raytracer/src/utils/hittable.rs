use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::utility::{degrees_to_radians, INFINITY};
use crate::utils::vec3::{Point3, Vec3, *};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
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

// move straight
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate {
            object: object.clone(),
            offset,
            bbox: object.bounding_box() + offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // move the ray by -offset and try to hit, return hitrecord whose point +offset
        let offset_r = Ray::new(r.origin() - &self.offset, *r.direction(), r.time());

        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let object = object.clone();
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box().clone();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = bbox.x.max * i as f64 + bbox.x.min * (1 - i) as f64;
                    let y = bbox.y.max * j as f64 + bbox.y.min * (1 - j) as f64;
                    let z = bbox.z.max * k as f64 + bbox.z.min * (1 - k) as f64;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min.e[c] = tester.e[c].min(min.e[c]);
                        max.e[c] = tester.e[c].max(max.e[c]);
                    }
                }
            }
        }
        bbox = Aabb::new_diagonal(min, max);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // change the reference space
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin.e[0] = r.origin().e[0] * self.cos_theta - r.origin().e[2] * self.sin_theta;
        origin.e[2] = r.origin().e[0] * self.sin_theta + r.origin().e[2] * self.cos_theta;
        direction.e[0] = r.direction().e[0] * self.cos_theta - r.direction().e[2] * self.sin_theta;
        direction.e[2] = r.direction().e[0] * self.sin_theta + r.direction().e[2] * self.cos_theta;

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            //change back the intersection point
            let mut p = rec.p;
            p.e[0] = rec.p.e[0] * self.cos_theta + rec.p.e[2] * self.sin_theta;
            p.e[2] = rec.p.e[0] * (-self.sin_theta) + rec.p.e[2] * self.cos_theta;

            let mut normal = rec.normal;
            normal.e[0] = rec.normal.e[0] * self.cos_theta + rec.normal.e[2] * self.sin_theta;
            normal.e[2] = rec.normal.e[0] * (-self.sin_theta) + rec.normal.e[2] * self.cos_theta;

            rec.p = p;
            rec.normal = normal;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
