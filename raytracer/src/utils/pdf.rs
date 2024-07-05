// returning a random direction weighted by the internal PDF distribution, and
// returning the corresponding PDF distribution value in that direction.

use crate::utils::hittable::Hittable;
use crate::utils::onb::Onb;
use crate::utils::utility::PI;
use crate::utils::vec3::{
    dot, random_cosine_direction, random_unit_vector, unit_vector, Point3, Vec3,
};
use rand::Rng;
use std::sync::Arc;

pub trait Pdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::default()
    }
}

// uniform density
pub struct SpherePdf {}

pub struct CosinePdf {
    uvw: Onb,
}

pub struct HittablePdf {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

// mixture of cosine and hittable
pub struct MixturePdf {
    p: [Arc<dyn Pdf>; 2],
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

impl CosinePdf {
    pub fn new(w: Vec3) -> CosinePdf {
        let mut uvw = Onb::default();
        uvw.build_from_w(&w);
        CosinePdf { uvw }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(direction), self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        // self.uvw.local_vec(&random_on_hemisphere(&Vec3::new(0.0,0.0,1.0)))
        self.uvw.local_vec(&random_cosine_direction())
    }
}

// One hittable pdf only holds one object, and cannot be HittableList
impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> HittablePdf {
        HittablePdf { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }
    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> MixturePdf {
        MixturePdf { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}