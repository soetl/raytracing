use crate::{
    color::{Color, Linear},
    math::Point3,
    texture::Texture,
};

#[derive(Debug)]
pub struct SolidColor {
    albedo: Color<Linear>,
}

impl SolidColor {
    pub fn new(albedo: Color<Linear>) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _p: &Point3) -> Color<Linear> {
        self.albedo
    }
}

impl From<Color<Linear>> for SolidColor {
    fn from(albedo: Color<Linear>) -> Self {
        Self { albedo }
    }
}

impl From<(f32, f32, f32)> for SolidColor {
    fn from(albedo: (f32, f32, f32)) -> Self {
        Self {
            albedo: Color::new(albedo.0, albedo.1, albedo.2),
        }
    }
}
