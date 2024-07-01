use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::ray::Ray;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max; //最靠近光源的碰撞

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}
