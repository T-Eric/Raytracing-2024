use crate::utils::aabb::Aabb;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::hittable_list::HittableList;
use crate::utils::interval::Interval;
use crate::utils::material::Material;
use crate::utils::normal_map::NormalMap;
use crate::utils::onb::Onb;
use crate::utils::ray::Ray;
use crate::utils::utility::INFINITY;
use crate::utils::vec3::Point3;
use crate::utils::vec3::{cross, dot, unit_vector, Vec3};
use rand::Rng;
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
    nmap: Arc<dyn NormalMap>,
    bbox: Aabb,
    normal: Vec3,
    d: f64, //in Ax+By+Cz=D
    area: f64,
}

impl Quad {
    pub fn new(
        q: Point3,
        u: Vec3,
        v: Vec3,
        mat: Arc<dyn Material>,
        nmap: Arc<dyn NormalMap>,
    ) -> Quad {
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
            nmap,
            normal,
            d,
            area: n.length(),
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
                    rec.u = alpha;
                    rec.v = beta;
                    // 在此处修改normal，但愿有效
                    let (u, v) = self.nmap.convert((alpha, beta), (1.0, 1.0));
                    // 需要让wtb的u与横边平行，v与竖边平行，不一定要是正交
                    let mut wtb = Onb::default();
                    wtb.axis[0] = unit_vector(&self.u);
                    wtb.axis[1] = unit_vector(&self.v);
                    wtb.axis[2] = unit_vector(&rec.normal);
                    rec.normal = self.nmap.modify_normal((u, v), wtb);
                    Some(rec)
                }
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if let Some(rec) = self.hit(
            &Ray::new(*origin, *direction, 0.0),
            &Interval::new(0.001, INFINITY),
        ) {
            let dist_squared = rec.t * rec.t * direction.length_squared();
            let cosine = dot(direction, &rec.normal).abs() / direction.length();
            dist_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let p = self.q + (self.u * rng.gen_range(0.0..1.0)) + (self.v * rng.gen_range(0.0..1.0));
        &p - origin
    }
}

// a 3d box,formed by six sides
pub fn cube(
    a: Point3,
    b: Point3,
    mat: Arc<dyn Material>,
    nmap: Arc<dyn NormalMap>,
) -> Arc<HittableList> {
    let mut sides = HittableList::default();
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat,
        nmap,
    )));

    Arc::new(sides)
}
