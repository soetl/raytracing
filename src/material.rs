mod dielectric;
mod diffuse;
mod lambertian;
mod metal;

pub use {dielectric::Dielectric, diffuse::Diffuse, lambertian::Lambertian, metal::Metal};

use crate::{
    color::{Color, Linear},
    math::{Hit, Point3, Ray},
};

pub trait Material: Send + Sync + std::fmt::Debug {
    fn emit(&self, _u: f32, _v: f32, _p: &Point3) -> Color<Linear> {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<(Ray, Color<Linear>)> {
        None
    }
}
