pub mod sphere;

use std::{ops::Range, sync::Arc};

use crate::{material::Material, point::Point3, ray::Ray, vec::Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
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
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.end;
        let mut hit_record = None;

        for object in self {
            if let Some(record) = object.hit(ray, ray_t.start..closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}
