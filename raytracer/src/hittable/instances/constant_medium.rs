use crate::hittable::HitRecord;
use crate::materials::isotropic::Isotropic;
use crate::util::aabb::Aabb;
use crate::util::color::Color;
use crate::util::interval::Interval;
use crate::util::ray::Ray;
use crate::util::INFINITY;
use crate::{hittable::Hittable, util::vec3::*};
use rand::Rng;

pub struct ConstMedium<H: Hittable> {
    boundary: H,
    neg_inv_density: f64,
    phase_function: Isotropic,
}

impl<H: Hittable> ConstMedium<H> {
    pub fn new_color(boundary: H, density: f64, albedo: Color) -> ConstMedium<H> {
        ConstMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new_color(albedo),
        }
    }
}

impl<H: Hittable> Hittable for ConstMedium<H> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        let mut rng = rand::thread_rng();
        let enable_debug = false;
        let debugging = enable_debug && rng.gen_range(0.0..1.0) < 0.00001;

        let mut rec1 = None;
        let mut rec2 = None;

        if !self.boundary.hit(r, &Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        let mut rec1_data = if let Some(data) = rec1 {
            data
        } else {
            panic!("No hit record at front!");
        };

        if !self
            .boundary
            .hit(r, &Interval::new(rec1_data.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        let mut rec2_data = if let Some(data) = rec2 {
            data
        } else {
            panic!("No hit record at back!");
        };

        if debugging {
            eprintln!("t_min={0}, t_max={1}", rec1_data.t, rec2_data.t);
        }

        rec1_data.t = rec1_data.t.max(ray_t.min);
        rec2_data.t = rec2_data.t.min(ray_t.max);
        if rec1_data.t >= rec2_data.t {
            return false;
        }

        rec1_data.t = rec1_data.t.max(0.0);

        let ray_length = r.direction().length();
        let dist_inside = (rec2_data.t - rec1_data.t) * ray_length;
        let rd: f64 = rng.gen_range(0.0..1.0);
        let hit_dist = self.neg_inv_density * rd.ln();

        if hit_dist > dist_inside {
            return false;
        }

        let temp_t = rec1_data.t + hit_dist / ray_length;

        *rec = Some(HitRecord {
            p: r.at(temp_t),
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat: &self.phase_function,
            t: temp_t,
            u: 0.0,
            v: 0.0,
            front_face: true,
        });

        if debugging {
            if let Some(rec_data) = rec {
                eprintln!("hit_distance={}", hit_dist);
                eprintln!("rec.t={}", rec_data.t);
                eprintln!("rec.p={:?}", rec_data.p);
            }
        }
        true
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
