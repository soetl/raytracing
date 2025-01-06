use std::{ops::Range, sync::Arc};

use crate::{material::Material, point::Point3, ray::Ray, utils::RangeExt, vec::Vec3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center: Ray::new(center, Vec3::ZERO),
            radius,
            material,
        }
    }

    pub fn with_destination(mut self, destination: Point3) -> Self {
        self.center = Ray::new(self.center.origin(), destination - self.center.origin());
        self
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time());
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (h - discriminant_sqrt) / a;
        if !ray_t.surrounds(&root) {
            root = (h + discriminant_sqrt) / a;
            if !ray_t.surrounds(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let hit_rec = HitRecord::new(
            ray.direction,
            point,
            (point - current_center) / self.radius,
            root,
            self.material.clone(),
        );

        Some(hit_rec)
    }
}
