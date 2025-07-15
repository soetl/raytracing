use std::path::Path;

use image::RgbImage;

use crate::{
    color::{Color, Linear},
    point::Point3,
    texture::Texture,
    utils::DEBUG_COLOR,
};

#[derive(Debug)]
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Self {
        Self { image }
    }

    pub fn load(path: &Path) -> Result<Self, image::ImageError> {
        let image = image::open(path)?.into_rgb8();
        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f32, v: f32, _p: &Point3) -> Color<Linear> {
        if self.image.height() == 0 || self.image.width() == 0 {
            DEBUG_COLOR
        } else {
            let u = u.clamp(0.0, 1.0);
            let v = 1.0 - v.clamp(0.0, 1.0);

            let x = (u * self.image.width() as f32) as u32;
            let y = (v * self.image.height() as f32) as u32;
            let pixel = self.image.get_pixel(x, y);

            Color::new(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
            )
        }
    }
}
