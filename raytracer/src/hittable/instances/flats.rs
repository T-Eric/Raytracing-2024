use crate::features::normal_map::NormalMap;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{HitRecord, Hittable};
use crate::util::aabb::Aabb;
use crate::util::interval::Interval;
use crate::util::onb::Onb;
use crate::util::ray::Ray;
use crate::util::{degrees_to_radians, INFINITY};
use crate::{materials::Material, util::vec3::*};
use rand::Rng;

pub trait Flat {
    fn set_bounding_box(&mut self);
    fn is_interior(a: f64, b: f64, ins: &Point3) -> bool; // judge if a ray hitting flat hits object
}

pub struct Quad<M: Material, N: NormalMap> {
    q: Point3, // one diagonal
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: M,
    nmap: N,
    bbox: Aabb,
    normal: Vec3,
    d: f64, //in Ax+By+Cz=D
    area: f64,
}

impl<M: Material, N: NormalMap> Quad<M, N> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: M, nmap: N) -> Self {
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
impl<M: Material, N: NormalMap> Flat for Quad<M, N> {
    fn set_bounding_box(&mut self) {
        self.bbox = Aabb::new_aabb(
            &Aabb::new_diagonal(self.q, self.q + self.u + self.v),
            &Aabb::new_diagonal(self.q + self.u, self.q + self.v),
        );
    }

    fn is_interior(a: f64, b: f64, _ins: &Point3) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // judge if the intersection point is in the quad
        unit_interval.contains(a) && unit_interval.contains(b)
    }
}

impl<M: Material, N: NormalMap> Hittable for Quad<M, N> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        let denom = dot(&self.normal, r.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        //no hit on plane
        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false; // ray itself actually don't reach
        }
        let intersection = r.at(t); // hit flat, may not hit quad
        let planar_hitpt_vec = intersection - self.q; //the Q->P
        let alpha = dot(&self.w, &cross(&planar_hitpt_vec, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vec));
        if !Self::is_interior(alpha, beta, &intersection) {
            return false;
        }
        let mut rec_data = HitRecord {
            p: intersection,
            normal: self.normal,
            mat: &self.mat,
            t,
            u: alpha,
            v: beta,
            front_face: false,
        };
        rec_data.set_face_normal(r, self.normal);
        // 在此处修改normal，但愿有效
        let (u, v) = self.nmap.convert((alpha, beta), (1.0, 1.0));
        // 需要让wtb的u与横边平行，v与竖边平行，不一定要是正交
        let mut wtb = Onb::default();
        wtb.axis[0] = unit_vector(&self.u);
        wtb.axis[1] = unit_vector(&self.v);
        wtb.axis[2] = unit_vector(&rec_data.normal);
        rec_data.normal = self.nmap.modify_normal((u, v), wtb);
        *rec = Some(rec_data);
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec: Option<HitRecord> = None;
        if self.hit(
            &Ray::new(*origin, *direction, 0.0),
            &Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            let rec = rec.unwrap();
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
pub fn cube<M: Material + Clone + 'static, N: NormalMap + Clone + 'static>(
    a: Point3,
    b: Point3,
    mat: M,
    nmap: N,
) -> HittableList {
    let mut sides = HittableList::default();
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Box::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Box::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
        nmap.clone(),
    )));
    sides.add(Box::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat,
        nmap,
    )));

    sides
}

// only for obj files
pub struct Tria<M: Material> {
    pub p: [Point3; 3],
    pub tex: [(f64, f64); 3],
    pub norm: [Vec3; 3],
    pub mat: M,
    pub bbox: Aabb,
}

impl<M: Material> Tria<M> {
    pub fn new(
        points: &[Vec3],
        tex: &[(f64, f64)],
        normals: &[Vec3],
        indices: [usize; 3],
        mat: M,
    ) -> Self {
        // directly find using indices
        assert_ne!(normals.len(), 0);
        let bbox = Aabb::default();
        let mut ret = if tex.is_empty() {
            Self {
                p: [points[indices[0]], points[indices[1]], points[indices[2]]],
                tex: [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
                norm: [
                    normals[indices[0]],
                    normals[indices[1]],
                    normals[indices[2]],
                ],
                mat,
                bbox,
            }
        } else {
            Self {
                p: [points[indices[0]], points[indices[1]], points[indices[2]]],
                tex: [tex[indices[0]], tex[indices[1]], tex[indices[2]]],
                norm: [
                    normals[indices[0]],
                    normals[indices[1]],
                    normals[indices[2]],
                ],
                mat,
                bbox,
            }
        };
        ret.reset_bbox();
        ret
    }
    pub fn zoom(&mut self, scale: f64) {
        for p in self.p.iter_mut() {
            *p *= scale;
        }
        self.reset_bbox();
    }
    pub fn trans(&mut self, direct: Vec3) {
        for p in self.p.iter_mut() {
            *p += direct;
        }
        self.reset_bbox();
    }
    pub fn rotate(&mut self, center: Vec3, r_x: f64, r_y: f64, r_z: f64) {
        let mut op: [Vec3; 3] = Default::default();
        let mut on: [Vec3; 3] = Default::default();

        for (i, a) in self.p.iter_mut().enumerate() {
            on[i] = self.norm[i];
            *a -= center;
            op[i] = *a;
        }

        let cos_x = degrees_to_radians(r_x).cos();
        let sin_x = degrees_to_radians(r_x).sin();
        for i in 0..3 {
            self.p[i].e[1] = cos_x * op[i].e[1] - sin_x * op[i].e[2];
            self.p[i].e[2] = sin_x * op[i].e[1] + cos_x * op[i].e[2];
            op[i] = self.p[i];

            self.norm[i].e[1] = cos_x * on[i].e[1] - sin_x * on[i].e[2];
            self.norm[i].e[2] = sin_x * on[i].e[1] + cos_x * on[i].e[2];
            on[i] = self.norm[i];
        }

        let cos_y = degrees_to_radians(r_y).cos();
        let sin_y = degrees_to_radians(r_y).sin();
        for i in 0..3 {
            self.p[i].e[0] = cos_y * op[i].e[0] - sin_y * op[i].e[2];
            self.p[i].e[2] = sin_y * op[i].e[0] + cos_y * op[i].e[2];
            op[i] = self.p[i];

            self.norm[i].e[0] = cos_y * on[i].e[0] - sin_y * on[i].e[2];
            self.norm[i].e[2] = sin_y * on[i].e[0] + cos_y * on[i].e[2];
            on[i] = self.norm[i];
        }

        let cos_z = degrees_to_radians(r_z).cos();
        let sin_z = degrees_to_radians(r_z).sin();
        for i in 0..3 {
            self.p[i].e[0] = cos_z * op[i].e[0] - sin_z * op[i].e[1];
            self.p[i].e[1] = sin_z * op[i].e[0] + cos_z * op[i].e[1];

            self.norm[i].e[0] = cos_z * on[i].e[0] - sin_z * on[i].e[1];
            self.norm[i].e[1] = sin_z * on[i].e[0] + cos_z * on[i].e[1];
        }

        for i in 0..3 {
            self.p[i] += center;
        }
        self.reset_bbox();
    }
    pub fn set_position(&mut self, center_old: Vec3, center_new: Vec3) {
        for p in self.p.iter_mut() {
            *p += center_new - center_old;
        }
        self.reset_bbox();
    }
    fn get_hit_pt(&self, r: &Ray) -> (Vec3, f64) {
        let e1 = self.p[1] - self.p[0];
        let e2 = self.p[2] - self.p[0];
        let s = r.origin() - &self.p[0];
        let s1 = cross(r.direction(), &e2);
        let s2 = cross(&s, &e1);

        let div = 1.0 / dot(&s1, &e1);

        let t = dot(&s2, &e2) * div;
        let mut b: [f64; 3] = [0.0; 3];
        b[1] = dot(&s1, &s) * div;
        b[2] = dot(&s2, r.direction()) * div;
        b[0] = 1.0 - (b[1] + b[2]);

        (self.p[0] * b[0] + self.p[1] * b[1] + self.p[2] * b[2], t)
    }
    fn get_bary_coords(&self, ins: &Vec3) -> [f64; 3] {
        let mut n: [Vec3; 3] = [Vec3::default(); 3];
        let area_vec = cross(&(self.p[1] - self.p[0]), &(self.p[2] - self.p[0]));
        n[0] = cross(&(self.p[2] - self.p[1]), &(ins - &self.p[1]));
        n[1] = cross(&(self.p[0] - self.p[2]), &(ins - &self.p[2]));
        n[2] = cross(&(self.p[1] - self.p[0]), &(ins - &self.p[0]));

        [
            dot(&n[0], &area_vec) / area_vec.length().powi(2),
            dot(&n[1], &area_vec) / area_vec.length().powi(2),
            dot(&n[2], &area_vec) / area_vec.length().powi(2),
        ]
    }
    fn reset_bbox(&mut self) {
        self.bbox = Aabb::new_aabb(
            &Aabb::new_diagonal(self.p[0], self.p[1]),
            &Aabb::new_diagonal(self.p[0], self.p[2]),
        );
    }
}

impl<M: Material> Hittable for Tria<M> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        let (hit_pt, t) = self.get_hit_pt(r);
        if !ray_t.contains(t) {
            return false;
        }
        let c = self.get_bary_coords(&hit_pt);
        if c[1] < 0.0 || c[2] < 0.0 || c[1] + c[2] > 1.0 {
            return false;
        }

        // actual normal can calculate from bary coords
        let normal =
            unit_vector(&(self.norm[0] * c[0] + self.norm[1] * c[1] + self.norm[2] * c[2]));

        let mut u = 0.0;
        let mut v = 0.0;
        for (i, tex) in self.tex.iter().enumerate() {
            u += tex.0 * c[i];
            v += tex.1 * c[i];
        }

        let mut rec_data = HitRecord {
            p: hit_pt,
            normal,
            mat: &self.mat,
            t,
            u,
            v,
            front_face: false,
        };
        rec_data.set_face_normal(r, normal);
        *rec = Some(rec_data);
        true
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
