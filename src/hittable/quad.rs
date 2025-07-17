use std::{ops::Range, sync::Arc};

use crate::{
    logical::Aabb,
    material::Material,
    math::{HitRecord, Hittable, Point3, Ray, Vec3},
    utils::RangeExt,
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    d: f32,

    material: Arc<dyn Material>,
    aabb: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let corners = [q, q + u, q + v, q + u + v];
        let min = corners[0].min(corners[1]).min(corners[2]).min(corners[3]);
        let max = corners[0].max(corners[1]).max(corners[2]).max(corners[3]);
        let aabb = Aabb::from((min, max));

        let n = Vec3::cross(u, v);
        let normal = n.normalize();
        let d = Vec3::dot(q, normal);
        let w = n / n.dot(n);

        Self {
            q,
            u,
            v,
            w,
            normal,
            d,
            material,
            aabb,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction());

        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin())) / denom;

        if !ray_t.surrounds(&t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;

        let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !((0.0..1.0).contains(&alpha) && (0.0..1.0).contains(&beta)) {
            return None;
        }

        let hit_rec = HitRecord::new(
            ray.direction,
            intersection,
            self.normal,
            t,
            (alpha, beta),
            self.material.clone(),
        );

        Some(hit_rec)
    }

    fn aabb(&self) -> Aabb {
        self.aabb
    }
}
