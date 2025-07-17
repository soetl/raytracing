use std::{ops::Range, sync::Arc};

use crate::{
    math::{HitRecord, HitType, Hittable, Point3, Ray, Vec3},
    prelude::Aabb,
};

pub struct Translate {
    offset: Vec3,
    object: Arc<dyn Hittable>,
    aabb: Aabb,
}

impl Translate {
    pub fn new(offset: Vec3, object: Arc<dyn Hittable>) -> Self {
        let aabb = object.aabb() + offset;
        Self {
            offset,
            object,
            aabb,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction());

        self.object.hit(&offset_ray, ray_t).and_then(|mut rec| {
            if let HitType::Physical { mut hit } = rec.hit {
                hit.point += self.offset;
                rec.hit = HitType::Physical { hit };
                Some(rec)
            } else {
                None
            }
        })
    }

    fn aabb(&self) -> Aabb {
        self.aabb
    }

    fn uv(&self, point: &Point3) -> (f32, f32) {
        let object_point = *point - self.offset;
        self.object.uv(&object_point)
    }
}
