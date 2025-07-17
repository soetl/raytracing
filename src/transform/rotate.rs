use std::{marker::PhantomData, ops::Range, sync::Arc};

use crate::{
    logical::Aabb,
    math::{HitRecord, HitType, Hittable, Point3, Ray, Vec3},
};

pub trait Axis: 'static + Send + Sync {}

pub struct X;
pub struct Y;
pub struct Z;

impl Axis for X {}
impl Axis for Y {}
impl Axis for Z {}

pub struct Rotate<A: Axis> {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Aabb,
    _axis: PhantomData<A>,
}

impl<A: Axis> Rotate<A> {
    #[inline(always)]
    pub fn aabb(&self) -> Aabb {
        self.aabb
    }

    #[inline]
    fn compute_rotated_aabb_generic<F>(input_aabb: Aabb, transform_fn: F) -> (Point3, Point3)
    where
        F: Fn(Vec3) -> Vec3,
    {
        let min_corner = input_aabb.center - input_aabb.half_extents;
        let max_corner = input_aabb.center + input_aabb.half_extents;

        let mut transformed_points = [Vec3::ZERO; 8];

        for (i, point) in transformed_points.iter_mut().enumerate() {
            let corner = Vec3::new(
                if i & 1 == 0 {
                    min_corner.x
                } else {
                    max_corner.x
                },
                if i & 2 == 0 {
                    min_corner.y
                } else {
                    max_corner.y
                },
                if i & 4 == 0 {
                    min_corner.z
                } else {
                    max_corner.z
                },
            );
            *point = transform_fn(corner);
        }

        let mut min = transformed_points[0];
        let mut max = transformed_points[0];

        for &point in &transformed_points[1..] {
            min = min.min(point);
            max = max.max(point);
        }

        (min, max)
    }

    #[inline]
    fn hit_generic<F, G>(
        &self,
        r: &Ray,
        ray_t: Range<f32>,
        forward_fn: F,
        inverse_fn: G,
    ) -> Option<HitRecord>
    where
        F: Fn(Vec3) -> Vec3,
        G: Fn(Vec3) -> Vec3,
    {
        let rotated_r = Ray::new(forward_fn(r.origin), forward_fn(r.direction));

        self.object.hit(&rotated_r, ray_t).map(|mut rec| {
            if let HitType::Physical { mut hit } = rec.hit {
                hit.point = inverse_fn(hit.point);
                hit.normal = inverse_fn(hit.normal);
                rec.hit = HitType::Physical { hit };
            }
            rec
        })
    }
}

impl Rotate<X> {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let (sin_theta, cos_theta) = angle.to_radians().sin_cos();
        let input_aabb = object.aabb();

        let (min, max) = Self::compute_rotated_aabb(input_aabb, sin_theta, cos_theta);
        let aabb = Aabb::from((min, max));

        Self {
            object,
            sin_theta,
            cos_theta,
            aabb,
            _axis: PhantomData,
        }
    }

    #[inline]
    fn compute_rotated_aabb(input_aabb: Aabb, sin_theta: f32, cos_theta: f32) -> (Point3, Point3) {
        Self::compute_rotated_aabb_generic(input_aabb, |v| {
            Vec3::new(
                v.x,
                cos_theta * v.y - sin_theta * v.z,
                sin_theta * v.y + cos_theta * v.z,
            )
        })
    }

    #[inline(always)]
    fn transform_forward(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            v.x,
            self.cos_theta * v.y + self.sin_theta * v.z,
            -self.sin_theta * v.y + self.cos_theta * v.z,
        )
    }

    #[inline(always)]
    fn transform_inverse(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            v.x,
            self.cos_theta * v.y - self.sin_theta * v.z,
            self.sin_theta * v.y + self.cos_theta * v.z,
        )
    }
}

impl Hittable for Rotate<X> {
    #[inline]
    fn hit(&self, r: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        self.hit_generic(
            r,
            ray_t,
            |v| self.transform_forward(v),
            |v| self.transform_inverse(v),
        )
    }

    #[inline(always)]
    fn aabb(&self) -> Aabb {
        self.aabb
    }

    fn uv(&self, point: &Point3) -> (f32, f32) {
        let object_point = self.transform_forward(*point);
        self.object.uv(&object_point)
    }
}

impl Rotate<Y> {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let (sin_theta, cos_theta) = angle.to_radians().sin_cos();
        let input_aabb = object.aabb();

        let (min, max) = Self::compute_rotated_aabb(input_aabb, sin_theta, cos_theta);
        let aabb = Aabb::from((min, max));

        Self {
            object,
            sin_theta,
            cos_theta,
            aabb,
            _axis: PhantomData,
        }
    }

    #[inline]
    fn compute_rotated_aabb(input_aabb: Aabb, sin_theta: f32, cos_theta: f32) -> (Point3, Point3) {
        Self::compute_rotated_aabb_generic(input_aabb, |v| {
            Vec3::new(
                cos_theta * v.x + sin_theta * v.z,
                v.y,
                -sin_theta * v.x + cos_theta * v.z,
            )
        })
    }

    #[inline(always)]
    fn transform_forward(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * v.x - self.sin_theta * v.z,
            v.y,
            self.sin_theta * v.x + self.cos_theta * v.z,
        )
    }

    #[inline(always)]
    fn transform_inverse(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * v.x + self.sin_theta * v.z,
            v.y,
            -self.sin_theta * v.x + self.cos_theta * v.z,
        )
    }
}

impl Hittable for Rotate<Y> {
    #[inline]
    fn hit(&self, r: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        self.hit_generic(
            r,
            ray_t,
            |v| self.transform_forward(v),
            |v| self.transform_inverse(v),
        )
    }

    #[inline(always)]
    fn aabb(&self) -> Aabb {
        self.aabb
    }

    fn uv(&self, point: &Point3) -> (f32, f32) {
        let object_point = self.transform_forward(*point);
        self.object.uv(&object_point)
    }
}

impl Rotate<Z> {
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> Self {
        let (sin_theta, cos_theta) = angle.to_radians().sin_cos();
        let input_aabb = object.aabb();

        let (min, max) = Self::compute_rotated_aabb(input_aabb, sin_theta, cos_theta);
        let aabb = Aabb::from((min, max));

        Self {
            object,
            sin_theta,
            cos_theta,
            aabb,
            _axis: PhantomData,
        }
    }

    #[inline]
    fn compute_rotated_aabb(input_aabb: Aabb, sin_theta: f32, cos_theta: f32) -> (Point3, Point3) {
        Self::compute_rotated_aabb_generic(input_aabb, |v| {
            Vec3::new(
                cos_theta * v.x - sin_theta * v.y,
                sin_theta * v.x + cos_theta * v.y,
                v.z,
            )
        })
    }

    #[inline(always)]
    fn transform_forward(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * v.x + self.sin_theta * v.y,
            -self.sin_theta * v.x + self.cos_theta * v.y,
            v.z,
        )
    }

    #[inline(always)]
    fn transform_inverse(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.cos_theta * v.x - self.sin_theta * v.y,
            self.sin_theta * v.x + self.cos_theta * v.y,
            v.z,
        )
    }
}

impl Hittable for Rotate<Z> {
    #[inline]
    fn hit(&self, r: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        self.hit_generic(
            r,
            ray_t,
            |v| self.transform_forward(v),
            |v| self.transform_inverse(v),
        )
    }

    #[inline(always)]
    fn aabb(&self) -> Aabb {
        self.aabb
    }

    fn uv(&self, point: &Point3) -> (f32, f32) {
        let object_point = self.transform_forward(*point);
        self.object.uv(&object_point)
    }
}

pub type RotateX = Rotate<X>;
pub type RotateY = Rotate<Y>;
pub type RotateZ = Rotate<Z>;
