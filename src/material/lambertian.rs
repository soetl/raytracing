use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Vec3, VecExt},
};

use super::{Color, Linear, Material};

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Color<Linear>,
}

impl Lambertian {
    pub fn new(albedo: Color<Linear>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color<Linear>)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction).with_time(ray.time());
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
