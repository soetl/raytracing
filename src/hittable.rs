pub mod aabb;
pub mod sphere;

use std::{ops::Range, sync::Arc};

use crate::{material::Material, point::Point3, ray::Ray, vec::Vec3};

#[derive(Debug, Clone)]
pub enum HitType {
    Physical { hit: Hit },
    Logical,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

#[derive(Debug)]
pub struct HitRecord {
    pub hit: HitType,
    pub t: f32,
}

impl HitRecord {
    pub fn new(
        direction: Vec3,
        point: Point3,
        normal: Vec3,
        t: f32,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let front_face = direction.dot(normal) < 0.0;
        let normal = if front_face {
            normal.normalize()
        } else {
            -normal.normalize()
        };

        HitRecord {
            hit: HitType::Physical {
                hit: Hit {
            point,
            normal,
                    front_face,
                    material,
                },
            },
            t,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord>;
    fn aabb(&self) -> Aabb;
}
