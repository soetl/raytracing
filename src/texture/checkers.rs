use std::sync::Arc;

use crate::{
    color::{Color, Linear},
    point::Point3,
    texture::{color::SolidColor, Texture},
};

#[derive(Debug)]
pub struct CheckersTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckersTexture {
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for CheckersTexture {
    fn color(&self, u: f32, v: f32, p: &Point3) -> Color<Linear> {
        let (x, y, z) = (
            (self.inv_scale * p.x).floor() as i32,
            (self.inv_scale * p.y).floor() as i32,
            (self.inv_scale * p.z).floor() as i32,
        );

        if (x + y + z) % 2 == 0 {
            self.even.color(u, v, p)
        } else {
            self.odd.color(u, v, p)
        }
    }
}

impl From<(f32, Color<Linear>, Color<Linear>)> for CheckersTexture {
    fn from((scale, even_color, odd_color): (f32, Color<Linear>, Color<Linear>)) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(even_color)),
            Arc::new(SolidColor::new(odd_color)),
        )
    }
}
