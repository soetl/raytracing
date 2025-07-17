mod cuboid;

pub use cuboid::Cuboid;

use std::sync::Arc;

use crate::math::Hittable;

pub trait Mesh {
    fn mesh(&self) -> Vec<Arc<dyn Hittable>>;
}
