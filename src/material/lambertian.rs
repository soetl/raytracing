use std::sync::Arc;

use crate::{
    hittable::Hit,
    ray::Ray,
    texture::{color::SolidColor, Texture},
    vec::{Vec3, VecExt},
};

use super::{Color, Linear, Material};

#[derive(Clone, Debug)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color<Linear>)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction).with_time(ray.time());
        let attenuation = self.texture.color(hit.u, hit.v, &hit.point);
        Some((scattered, attenuation))
    }
}

impl From<Color<Linear>> for Lambertian {
    fn from(color: Color<Linear>) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(color)),
        }
    }
}
