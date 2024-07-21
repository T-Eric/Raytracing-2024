use crate::{
    hittable::{HitRecord, Hittable},
    util::{aabb::Aabb, interval::Interval, ray::Ray, vec3::Vec3},
};

pub struct Translate<H: Hittable> {
    object: H,
    offset: Vec3,
    bbox: Aabb,
}

impl<H: Hittable> Translate<H> {
    pub fn new(object: H, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit<'a>(&'a self, r: &Ray, ray_t: &Interval, rec: &mut Option<HitRecord<'a>>) -> bool {
        // move the ray by -offset and try to hit, return hitrecord whose point +offset
        let offset_r = Ray::new(r.origin() - &self.offset, *r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }
        let rec_data = if let Some(data) = rec {
            data
        } else {
            panic!("No hit record!");
        };
        rec_data.p += self.offset;
        rec_data.set_face_normal(&offset_r, rec_data.normal);
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
