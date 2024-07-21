// Bounding Volume Hierarchy, a big set of all hittables

use crate::{
    hittable::{hittable_list::HittableList, HitRecord, Hittable},
    util::{aabb::Aabb, interval::Interval, ray::Ray},
};
use rand::Rng;
use std::cmp::Ordering;

pub struct BvhNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new_vec(mut objects: Vec<Box<dyn Hittable>>) -> BvhNode {
        let mut bbox = Aabb::EMPTY;
        for object in objects.iter() {
            bbox = Aabb::new_aabb(&bbox, object.bounding_box());
        }
        // use random axis directly
        let axis = rand::thread_rng().gen_range(0..3);
        // bisect recursion
        let object_span = objects.len();

        let (left, right) = if object_span == 1 {
            (Some(objects.pop().unwrap()), None)
        } else if object_span == 2 {
            let r = objects.pop().unwrap();
            let l = objects.pop().unwrap();
            (Some(l), Some(r))
        } else {
            objects.sort_by(|a, b| Self::box_compare(&**a, &**b, axis));
            let mid = object_span / 2;
            let mut left_vec = objects;
            let right_vec = left_vec.split_off(mid);
            (
                Some(Box::new(BvhNode::new_vec(left_vec)) as Box<dyn Hittable>),
                Some(Box::new(BvhNode::new_vec(right_vec)) as Box<dyn Hittable>),
            )
        };

        BvhNode { left, right, bbox }
    }

    pub fn new_list(list: HittableList) -> BvhNode {
        Self::new_vec(list.objects)
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(Ordering::Equal)
    }
}

impl Hittable for BvhNode {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = if let Some(leftside) = &self.left {
            leftside.hit(r, ray_t, rec)
        } else {
            false
        };
        let left_t = if let Some(rec_data) = rec {
            rec_data.t
        } else {
            ray_t.max
        };
        let hit_right = if let Some(rightside) = &self.right {
            let r_interval = Interval::new(ray_t.min, if hit_left { left_t } else { ray_t.max });
            rightside.hit(r, &r_interval, rec)
        } else {
            false
        };
        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
