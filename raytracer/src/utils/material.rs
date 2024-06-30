use crate::utils::color::Color;
use crate::utils::hittable::HitRecord;
use crate::utils::ray::Ray;
use crate::utils::vec3::refract;
use crate::utils::vec3::{dot, random_unit_vector, reflect, unit_vector, Vec3};

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

pub struct Dielectric {
    refraction_index: f64,
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
    pub fn _new(albedo: &Color, fuzz: f64) -> Self {
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

impl Dielectric {
    pub fn _new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for Fresnel reflection
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(&-&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        //consider full reflection
        let cannot_reflect = ri * sin_theta > 1.0;
        let direction: Vec3 =
            if cannot_reflect || self.reflectance(cos_theta, ri) > rand::random::<f64>() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        *scattered = Ray::new(&rec.p, &direction);
        true
    }
}
