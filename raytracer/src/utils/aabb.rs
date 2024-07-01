// aabb means Axis-Aligned Bounding Boxes (a box and its 3 perpendicular
// pairwise (parallel to an axis) forms a box)

use crate::utils::interval::Interval;
use crate::utils::ray::Ray;
use crate::utils::vec3::Point3;
use std::mem::swap;

#[derive(Default, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn _new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb {
            x,
            y,
            z
        }
    }
    pub fn new_diagonal(a: Point3, b: Point3) -> Aabb {
        Aabb {
            x: if a.x() < b.x() {
                Interval::new(a.x(), b.x())
            } else {
                Interval::new(b.x(), a.x())
            },
            y: if a.y() < b.y() {
                Interval::new(a.y(), b.y())
            } else {
                Interval::new(b.y(), a.y())
            },
            z: if a.z() < b.z() {
                Interval::new(a.z(), b.z())
            } else {
                Interval::new(b.z(), a.z())
            },
        }
    }
    pub fn new_aabb(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            x: Interval::new_interval(&a.x, &b.x),
            y: Interval::new_interval(&a.y, &b.y),
            z: Interval::new_interval(&a.z, &b.z),
        }
    }
    pub fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = r.origin().clone();
        let ray_dir = r.direction().clone();
        let mut ray_t = ray_t.clone();

        for axis in 0..3 {
            let axis = axis as usize;
            let ax = self.axis_interval(axis as i32);
            let adinv = 1.0 / ray_dir.get(axis).unwrap(); // standardize

            let mut t0 = (ax.min - ray_orig.get(axis).unwrap()) * adinv;
            let mut t1 = (ax.max - ray_orig.get(axis).unwrap()) * adinv;
            if t0 >= t1 {
                swap(&mut t0, &mut t1);
            }
            // if t0>ray_t.min{ray_t.min=t0;}
            ray_t.min = ray_t.min.max(t0);
            // if t1<ray_t.max{ray_t.max=t1;}
            ray_t.max = ray_t.max.min(t1);
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
    // The longest axis of the bounding box
    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
}

// consts
impl Aabb {
    pub const EMPTY: Aabb = Aabb {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };
    pub const _UNIVERSE: Aabb = Aabb {
        x: Interval::_UNIVERSE,
        y: Interval::_UNIVERSE,
        z: Interval::_UNIVERSE,
    };
}