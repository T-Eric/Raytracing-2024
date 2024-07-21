use crate::{hittable::Hittable, pdf::Pdf, util::vec3::*};

pub struct HittablePdf<'a, H: Hittable> {
    objects: &'a H,
    origin: Point3,
}

impl<'a, H: Hittable> HittablePdf<'a, H> {
    pub fn new(objects: &'a H, origin: Point3) -> Self {
        HittablePdf { objects, origin }
    }
}

impl<'a, H: Hittable> Pdf for HittablePdf<'a, H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
