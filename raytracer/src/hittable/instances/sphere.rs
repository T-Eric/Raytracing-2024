use crate::{
    hittable::{HitRecord, Hittable},
    materials::Material,
    util::{aabb::Aabb, interval::Interval, onb::Onb, ray::Ray, vec3::*, INFINITY, PI},
};
use rand::Rng;

pub struct Sphere<M: Material> {
    pub center1: Point3,
    pub radius: f64,
    mat: M,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new_static(center: Point3, radius: f64, mat: M) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center1: center,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::default(),
            bbox: Aabb::new_diagonal(center - rvec, center + rvec),
        }
    }
    pub fn new_motive(center1: Point3, center2: Point3, radius: f64, mat: M) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_diagonal(center1 - rvec, center1 + rvec);
        let box2 = Aabb::new_diagonal(center2 - rvec, center2 + rvec);
        Sphere {
            center1,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: Aabb::new_aabb(&box1, &box2),
        }
    }
    pub fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + (self.center_vec * time)
    }

    // ball axis to standard axis
    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        let center = if self.is_moving {
            Self::sphere_center(self, r.time())
        } else {
            self.center1
        };
        let oc = &center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = h * h - a * c;
        if delta < 0.0 {
            return false;
        }
        let sqrtd = delta.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - center) / self.radius;
        let mut rec_data = HitRecord {
            p: r.at(root),
            normal: Vec3::default(),
            mat: &self.mat,
            t: root,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };
        (rec_data.u, rec_data.v) = Self::get_sphere_uv(&outward_normal);
        rec_data.set_face_normal(r, outward_normal);

        *rec = Some(rec_data);
        true
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // only for static spheres
        let mut rec = None;
        if !self.hit(
            &Ray::new(*origin, *direction, 0.0),
            &Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }
        let cos_theta_max =
            (1.0 - self.radius.powi(2) / (&self.center1 - origin).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = &self.center1 - origin;
        let dist_squared = direction.length_squared();
        let mut uvw = Onb::default();
        uvw.build_from_w(&direction);
        uvw.local_vec(&random_to_sphere(self.radius, dist_squared))
    }
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
