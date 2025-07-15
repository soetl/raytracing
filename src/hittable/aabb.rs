use crate::{point::Point3, ray::Ray, vec::Vec3};

use super::{HitRecord, HitType, Hittable};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub center: Vec3,
    pub half_extents: Vec3,
}

impl Aabb {
    pub fn new(center: Vec3, half_extents: Vec3) -> Self {
        Aabb {
            center,
            half_extents,
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb {
            center: Vec3::ZERO,
            half_extents: Vec3::ZERO,
        }
    }
}

impl From<(Point3, Point3)> for Aabb {
    fn from((min, max): (Point3, Point3)) -> Self {
        let actual_min = Point3::new(min.x.min(max.x), min.y.min(max.y), min.z.min(max.z));
        let actual_max = Point3::new(min.x.max(max.x), min.y.max(max.y), min.z.max(max.z));

        let center = (actual_min + actual_max) / 2.0;
        let half_extents = (actual_max - actual_min) / 2.0;

        Aabb::new(center, half_extents)
    }
}

impl From<(Aabb, Aabb)> for Aabb {
    fn from((a, b): (Aabb, Aabb)) -> Self {
        let a_min = a.center - a.half_extents;
        let a_max = a.center + a.half_extents;
        let b_min = b.center - b.half_extents;
        let b_max = b.center + b.half_extents;

        let min = Point3::new(
            a_min.x.min(b_min.x),
            a_min.y.min(b_min.y),
            a_min.z.min(b_min.z),
        );
        let max = Point3::new(
            a_max.x.max(b_max.x),
            a_max.y.max(b_max.y),
            a_max.z.max(b_max.z),
        );

        Aabb::from((min, max))
    }
}

impl Aabb {
    pub fn empty() -> Self {
        Aabb {
            center: Vec3::ZERO,
            half_extents: Vec3::ZERO,
        }
    }
}

impl Hittable for Aabb {
    fn hit(&self, ray: &Ray, mut ray_t: std::ops::Range<f32>) -> Option<super::HitRecord> {
        let ray_origin = ray.origin;
        let ray_direction = ray.direction;

        for i in 0..3 {
            let inv_d = 1.0 / ray_direction[i];
            let mut t0 = (self.center[i] - self.half_extents[i] - ray_origin[i]) * inv_d;
            let mut t1 = (self.center[i] + self.half_extents[i] - ray_origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.start {
                ray_t.start = t0;
            }
            if t1 < ray_t.end {
                ray_t.end = t1;
            }

            if ray_t.start > ray_t.end {
                return None;
            }
        }

        Some(HitRecord {
            hit: HitType::Logical,
            t: ray_t.start,
        })
    }

    fn aabb(&self) -> Aabb {
        *self
    }
}
