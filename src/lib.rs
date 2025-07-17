pub mod camera;
pub mod color;
mod hittable;
pub mod material;
pub mod mesh;
mod point;
mod ray;
pub mod texture;
pub mod transform;
pub mod utils;
mod vec;

pub mod prelude {
    pub use crate::{
        camera::*,
        color::*,
        logical::*,
        material::*,
        math::{Hittable, Point3, Vec3, VecExt},
        mesh::*,
        primitives::*,
        render,
        texture::*,
        transform::*,
    };
}

pub mod math {
    pub use crate::{hittable::*, point::*, ray::*, vec::*};
}

pub mod primitives {
    pub use crate::hittable::primitive::*;
}

pub mod logical {
    pub use crate::hittable::logical::*;
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
