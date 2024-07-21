use crate::{
    hittable::{HitRecord, Hittable},
    util::{aabb::Aabb, interval::Interval, ray::Ray, vec3::*},
};
use rand::Rng;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = Aabb::new_aabb(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
    pub fn _clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        let mut temp_rec = None;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                if let Some(rec_data) = &temp_rec {
                    closest_so_far = rec_data.t;
                    *rec = temp_rec.clone();
                }
            }
        }

        hit_anything
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        if self.objects.is_empty() {
            return 0.1;
        }
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        if self.objects.is_empty() {
            return Vec3::random();
        }
        let mut rng = rand::thread_rng();
        let int_size = self.objects.len();
        let pos = rng.gen_range(0..int_size);
        self.objects[pos].random(origin)
    }
}
