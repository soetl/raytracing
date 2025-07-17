use std::{path::Path, sync::Arc};

use image::{ImageError, RgbImage};

use crate::{
    color::{Color, Linear},
    material::Material,
    math::{Hit, Ray, Vec3, VecExt},
    texture::{CheckersTexture, ImageTexture, PerlinNoise, SolidColor, Texture},
};

#[derive(Clone, Debug)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color<Linear>)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction).with_time(ray.time());
        let attenuation = self.texture.color(hit.u, hit.v, &hit.point);
        Some((scattered, attenuation))
    }
}

impl From<SolidColor> for Lambertian {
    fn from(color: SolidColor) -> Self {
        Self {
            texture: Arc::new(color),
        }
    }
}

impl From<Color<Linear>> for Lambertian {
    fn from(color: Color<Linear>) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(color)),
        }
    }
}

impl From<CheckersTexture> for Lambertian {
    fn from(texture: CheckersTexture) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}

impl From<ImageTexture> for Lambertian {
    fn from(texture: ImageTexture) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}

impl From<RgbImage> for Lambertian {
    fn from(image: RgbImage) -> Self {
        Self {
            texture: Arc::new(ImageTexture::new(image)),
        }
    }
}

impl TryFrom<&Path> for Lambertian {
    type Error = ImageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(Self {
            texture: Arc::new(ImageTexture::load(path)?),
        })
    }
}

impl From<PerlinNoise> for Lambertian {
    fn from(texture: PerlinNoise) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}
