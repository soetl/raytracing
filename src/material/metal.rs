use crate::{
    color::{Color, Linear},
    hittable::HitRecord,
    ray::Ray,
    vec::{Vec3, VecExt},
};

use super::Material;

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color<Linear>,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color<Linear>, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color<Linear>)> {
        let reflected =
            ray.direction().reflect(hit.normal).normalize() + Vec3::random_unit() * self.fuzz;
        let scattered = Ray::new(hit.point, reflected).with_time(ray.time());
        let attenuation = self.albedo;
        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
