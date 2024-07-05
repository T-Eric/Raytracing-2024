use crate::utils::aabb::Aabb;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3};
use rand::Rng;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = Aabb::new_aabb(&self.bbox, object.bounding_box());
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
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let int_size = self.objects.len();
        let pos = rng.gen_range(0..=int_size - 1);
        self.objects[pos].random(origin)
    }
}
