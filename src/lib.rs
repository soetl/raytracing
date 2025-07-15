use std::path::PathBuf;

use camera::CameraConfig;
use hittable::Hittable;
use image::ImageResult;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod point;
pub mod ray;
pub mod texture;
pub mod utils;
pub mod vec;

pub mod prelude {
    pub use crate::{
        camera::CameraConfig,
        color::Color,
        hittable::{bvh_node::BvhNode, sphere::Sphere, Hittable},
        material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
        point::Point3,
        render,
        vec::Vec3,
    };
}

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
