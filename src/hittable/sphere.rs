use std::{ops::Range, sync::Arc};

use crate::{
    logical::Aabb,
    material::Material,
    math::{HitRecord, Hittable, Point3, Ray, Vec3},
    utils::RangeExt,
};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
    aabb: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        let center = Ray::new(center, Vec3::ZERO);
        let aabb = Aabb::new(center.origin(), Vec3::splat(radius));

        Sphere {
            center,
            radius,
            material,
            aabb,
        }
    }

    pub fn with_destination(mut self, destination: Point3) -> Self {
        self.center = Ray::new(self.center.origin(), destination - self.center.origin());

        self.aabb = {
            let half_extents = Vec3::new(self.radius, self.radius, self.radius);
            let boundary_box1 = Aabb::from((
                self.center.at(0.0) - half_extents,
                self.center.at(0.0) + half_extents,
            ));
            let boundary_box2 = Aabb::from((
                self.center.at(1.0) - half_extents,
                self.center.at(1.0) + half_extents,
            ));

            Aabb::from((boundary_box1, boundary_box2))
        };

        self
    }
}

impl Hittable for Sphere {
    #[inline]
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
        let outward_normal = (point - current_center) / self.radius;
        let hit_rec = HitRecord::new(
            ray.direction,
            point,
            outward_normal,
            root,
            self.uv(&outward_normal),
            self.material.clone(),
        );

        Some(hit_rec)
    }

    #[inline]
    fn aabb(&self) -> Aabb {
        self.aabb
    }

    #[inline]
    fn uv(&self, point: &Vec3) -> (f32, f32) {
        use std::f32::consts::PI;

        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}
