pub mod camera;
pub mod color;
mod hittable;
pub mod material;
mod point;
mod ray;
pub mod texture;
pub mod utils;
mod vec;

pub mod prelude {
    pub use crate::{
        camera::{Camera, CameraConfig},
        color::{Color, Linear, Srgb},
        logical::BvhNode,
        material::{Dielectric, Lambertian, Material, Metal},
        math::{Hittable, Point3, Vec3, VecExt},
        primitives::Sphere,
        render,
        texture::{CheckersTexture, ImageTexture, PerlinNoise, SolidColor},
    };
}

pub mod math {
    pub use crate::{
        hittable::{Hit, HitRecord, HitType, Hittable},
        point::Point3,
        ray::Ray,
        vec::{Vec3, VecExt},
    };
}

pub mod primitives {
    pub use crate::hittable::primitives::Sphere;
}

pub mod logical {
    pub use crate::hittable::logical::{Aabb, BvhNode};
}

use std::path::PathBuf;

use image::ImageResult;

use crate::{camera::CameraConfig, math::Hittable};

pub fn render(
    world: &impl Hittable,
    path: impl Into<PathBuf>,
    camera_config: &CameraConfig,
) -> ImageResult<()> {
    let camera = camera::Camera::new(camera_config);
    let image = camera.render(world);

    let path: PathBuf = path.into();
    let path = if path.is_dir() {
        path.join("image.png")
    } else {
        path.with_extension("png")
    };

    image.save(path)
}
