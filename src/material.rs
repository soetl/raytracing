mod dielectric;
mod lambertian;
mod metal;

pub use {dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

use crate::{
    color::{Color, Linear},
    math::{Hit, Ray},
};

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color<Linear>)>;
}
