mod checkers;
mod color;
mod image;
mod perlin;

pub use {checkers::CheckersTexture, color::SolidColor, image::ImageTexture, perlin::PerlinNoise};

use std::fmt::Debug;

use crate::{
    color::{Color, Linear},
    point::Point3,
};

pub trait Texture: Send + Sync + Debug {
    fn color(&self, u: f32, v: f32, p: &Point3) -> Color<Linear>;
}
