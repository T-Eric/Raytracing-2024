// Bounding Volume Hierarchy, a big set of all hittables

use crate::utils::aabb::Aabb;
use crate::utils::hittable::{HitRecord, Hittable};
use crate::utils::hittable_list::HittableList;
use crate::utils::interval::Interval;
use crate::utils::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new_vec(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        // Build the box with the largest axis
        let mut bbox = Aabb::EMPTY;
        for object in objects.iter() {
            bbox = Aabb::new_aabb(&bbox, object.bounding_box())
        }
        let axis = bbox.longest_axis();

        // bisect recursion
        let object_span = end - start;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            left = objects[start].clone();
            right = left.clone(); // 如果只有一个对象，左右子树相同
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));

            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::new_vec(objects, start, mid));
            right = Arc::new(BvhNode::new_vec(objects, mid, end));
        }

        BvhNode {
            left: left.clone(),
            right: right.clone(),
            bbox,
        }
    }

    pub fn new_list(list: &mut HittableList) -> BvhNode {
        let len = list.objects.len();
        Self::new_vec(&mut list.objects, 0, len)
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }
        match self.left.hit(r, ray_t) {
            Some(left_rec) => {
                let right_ray_t = Interval::new(ray_t.min, left_rec.t);
                match self.right.hit(r, &right_ray_t) {
                    Some(right_rec) => {
                        if right_rec.t < left_rec.t {
                            Some(right_rec)
                        } else {
                            Some(left_rec)
                        }
                    }
                    None => Some(left_rec),
                }
            }
            None => self.right.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
