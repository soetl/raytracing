use std::{path::Path, sync::Arc};

use image::{ImageError, RgbImage};

use crate::{
    color::{Color, Linear},
    prelude::Material,
    texture::{CheckersTexture, ImageTexture, PerlinNoise, SolidColor, Texture},
};

#[derive(Debug)]
pub struct Diffuse {
    texture: Arc<dyn Texture>,
}

impl Diffuse {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for Diffuse {
    fn emit(&self, u: f32, v: f32, p: &crate::prelude::Point3) -> Color<Linear> {
        self.texture.color(u, v, p)
    }
}

impl From<SolidColor> for Diffuse {
    fn from(color: SolidColor) -> Self {
        Self {
            texture: Arc::new(color),
        }
    }
}

impl From<Color<Linear>> for Diffuse {
    fn from(color: Color<Linear>) -> Self {
        Self {
            texture: Arc::new(SolidColor::new(color)),
        }
    }
}

impl From<CheckersTexture> for Diffuse {
    fn from(texture: CheckersTexture) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}

impl From<ImageTexture> for Diffuse {
    fn from(texture: ImageTexture) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}

impl From<RgbImage> for Diffuse {
    fn from(image: RgbImage) -> Self {
        Self {
            texture: Arc::new(ImageTexture::new(image)),
        }
    }
}

impl TryFrom<&Path> for Diffuse {
    type Error = ImageError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(Self {
            texture: Arc::new(ImageTexture::load(path)?),
        })
    }
}

impl From<PerlinNoise> for Diffuse {
    fn from(texture: PerlinNoise) -> Self {
        Self {
            texture: Arc::new(texture),
        }
    }
}
