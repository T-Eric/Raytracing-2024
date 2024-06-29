use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::ray::Ray;
use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

// impl Default for HittableList{
//     fn default() -> Self {
//         Self{
//             objects:Vec::new()
//         }
//     }
// }

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax; //最靠近光源的碰撞

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
