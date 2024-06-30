use crate::utils::color::Color;
use crate::utils::hittable::HitRecord;
use crate::utils::ray::Ray;
use crate::utils::vec3::{dot, random_unit_vector, reflect, unit_vector};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

// Lambertian material which always do fraction reflection
pub struct Lambertian {
    albedo: Color,
}

// Metal, full reflection
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Lambertian {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &rec.normal + &random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *_scattered = Ray::new(&rec.p, &scatter_direction);
        *_attenuation = self.albedo.clone();
        true
    }
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Metal {
            albedo: albedo.clone(),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        // fuzz operation
        reflected = unit_vector(&reflected) + (random_unit_vector() * self.fuzz);
        *scattered = Ray::new(&rec.p, &reflected);
        *_attenuation = self.albedo.clone();

        dot(scattered.direction(), &rec.normal) > 0.0
    }
}
