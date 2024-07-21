use crate::{
    hittable::{HitRecord, Hittable},
    util::{aabb::Aabb, degrees_to_radians, interval::Interval, ray::Ray, vec3::*, INFINITY},
};

pub struct RotateY<H: Hittable> {
    object: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(object: H, angle: f64) -> Self {
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

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        // change the reference space
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin.e[0] = r.origin().e[0] * self.cos_theta - r.origin().e[2] * self.sin_theta;
        origin.e[2] = r.origin().e[0] * self.sin_theta + r.origin().e[2] * self.cos_theta;
        direction.e[0] = r.direction().e[0] * self.cos_theta - r.direction().e[2] * self.sin_theta;
        direction.e[2] = r.direction().e[0] * self.sin_theta + r.direction().e[2] * self.cos_theta;

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }
        let rec_data = if let Some(data) = rec {
            data
        } else {
            panic!("No hit record!");
        };
        let mut p = rec_data.p;
        p.e[0] = rec_data.p.e[0] * self.cos_theta + rec_data.p.e[2] * self.sin_theta;
        p.e[2] = rec_data.p.e[0] * (-self.sin_theta) + rec_data.p.e[2] * self.cos_theta;

        let mut normal = rec_data.normal;
        normal.e[0] = rec_data.normal.e[0] * self.cos_theta + rec_data.normal.e[2] * self.sin_theta;
        normal.e[2] =
            rec_data.normal.e[0] * (-self.sin_theta) + rec_data.normal.e[2] * self.cos_theta;

        rec_data.p = p;
        rec_data.normal = normal;
        true
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
