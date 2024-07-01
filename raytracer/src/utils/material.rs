use crate::utils::color::Color;
use crate::utils::hittable::HitRecord;
use crate::utils::ray::Ray;
use crate::utils::vec3::refract;
use crate::utils::vec3::{dot, random_unit_vector, reflect, unit_vector, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>; // attenuation and scattered
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = &rec.normal + &random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        let attenuation = self.albedo.clone();
        let scattered = Ray::new(&rec.p, &scatter_direction);
        Some((attenuation, scattered))
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        // fuzz operation
        reflected = unit_vector(&reflected) + (random_unit_vector() * self.fuzz);
        let scattered = Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo.clone();

        if dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
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

        let scattered = Ray::new(&rec.p, &direction);

        Some((attenuation, scattered))
    }
}
