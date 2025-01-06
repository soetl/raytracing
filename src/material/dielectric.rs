use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::Random, vec::Vec3};

use super::{Linear, Material};

#[derive(Clone, Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color<Linear>)> {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > f32::random() {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, ri)
        };

        let scattered = Ray::new(hit.point, direction).with_time(ray.time());

        Some((scattered, Color::from(Vec3::ONE)))
    }
}
