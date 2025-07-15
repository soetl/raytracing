pub mod checkers;
pub mod color;
pub mod image;

use std::fmt::Debug;

use crate::{
    color::{Color, Linear},
    point::Point3,
};

pub trait Texture: Send + Sync + Debug {
    fn color(&self, u: f32, v: f32, p: &Point3) -> Color<Linear>;
}
