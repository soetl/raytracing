use crate::{point::Point3, vec::Vec3};

#[derive(Clone, Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time;
        self
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
