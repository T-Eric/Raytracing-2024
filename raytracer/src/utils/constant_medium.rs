// fog, smoke, etc., partially transparent

use crate::utils::aabb::Aabb;
use crate::utils::color::Color;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::material::Isotropic;
use crate::utils::material::Material;
use crate::utils::ray::Ray;
use crate::utils::texture::Texture;
use crate::utils::utility::INFINITY;
use crate::utils::vec3::{Point3, Vec3};
use rand::Rng;
use std::sync::Arc;

pub struct ConstMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstMedium {
    pub fn _new_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        tex: Arc<dyn Texture>,
    ) -> ConstMedium {
        ConstMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::_new_texture(tex)),
        }
    }
    pub fn new_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> ConstMedium {
        ConstMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::_new_color(albedo)),
        }
    }
}

impl Hittable for ConstMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        const ENABLEDEBUG: bool = false;
        let debugging = ENABLEDEBUG && rng.gen_range(0.0..1.0) < 0.00001;

        if let Some(mut rec1) = self.boundary.hit(r, &Interval::UNIVERSE) {
            if let Some(mut rec2) = self
                .boundary
                .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY))
            {
                if debugging {
                    eprintln!("t_min={0}, t_max={1}", rec1.t, rec2.t);
                }
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }
                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.direction().length();
                let dist_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let rd: f64 = rng.gen_range(0.0..1.0);
                let hit_dist = self.neg_inv_density * rd.ln();

                if hit_dist > dist_inside_boundary {
                    return None;
                }

                let mut rec = HitRecord::new(
                    &Point3::default(),
                    &Vec3::new(1.0, 0.0, 0.0),
                    self.phase_function.clone(),
                    rec1.t + hit_dist / ray_length,
                    true,
                );
                rec.p = r.at(rec1.t);

                if debugging {
                    eprintln!(
                        "hit_dist={0}, rec.t={1}, rec.p={2:?}",
                        hit_dist, rec.t, rec.p
                    );
                }

                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
