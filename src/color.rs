#![allow(dead_code)]
use image::Rgb;

use crate::{utils::RangeExt, vec::Vec3};

pub trait ColorSpace: Copy + Clone + std::fmt::Debug {}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Color<T: ColorSpace> {
    pub v: Vec3,
    _color_space: std::marker::PhantomData<T>,
}

impl<T: ColorSpace> Color<T> {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color {
            v: Vec3::new(r, g, b),
            _color_space: std::marker::PhantomData,
        }
    }

    pub fn r(&self) -> f32 {
        self.v.x
    }

    pub fn mut_r(&mut self) -> &mut f32 {
        &mut self.v.x
    }

    pub fn g(&self) -> f32 {
        self.v.y
    }

    pub fn mut_g(&mut self) -> &mut f32 {
        &mut self.v.y
    }

    pub fn b(&self) -> f32 {
        self.v.z
    }

    pub fn mut_b(&mut self) -> &mut f32 {
        &mut self.v.z
    }
}

impl<T: ColorSpace> From<Vec3> for Color<T> {
    fn from(vec: Vec3) -> Self {
        Color {
            v: vec,
            _color_space: std::marker::PhantomData,
        }
    }
}

impl<T: ColorSpace> From<Color<T>> for Vec3 {
    fn from(color: Color<T>) -> Self {
        color.v
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Linear;
impl ColorSpace for Linear {}

impl Color<Linear> {
    pub fn to_srgb(self) -> Color<Srgb> {
        Color::from(self)
    }
}

impl From<Color<Srgb>> for Color<Linear> {
    fn from(color: Color<Srgb>) -> Self {
        Self::new(
            Srgb::gamma_function(color.r()),
            Srgb::gamma_function(color.g()),
            Srgb::gamma_function(color.b()),
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Srgb;
impl ColorSpace for Srgb {}

impl Srgb {
    pub fn gamma_function(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }
        if value <= 0.04045 {
            value / 12.92 // linear falloff in dark values
        } else {
            ((value + 0.055) / 1.055).powf(2.4) // gamma curve in other area
        }
    }

    pub fn gamma_function_inverse(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }

        if value <= 0.0031308 {
            value * 12.92 // linear falloff in dark values
        } else {
            (1.055 * value.powf(1.0 / 2.4)) - 0.055 // gamma curve in other area
        }
    }
}

impl Color<Srgb> {
    pub fn to_linear(self) -> Color<Linear> {
        Color::from(self)
    }
}

impl From<Color<Linear>> for Color<Srgb> {
    fn from(color: Color<Linear>) -> Color<Srgb> {
        Color::new(
            Srgb::gamma_function_inverse(color.r()),
            Srgb::gamma_function_inverse(color.g()),
            Srgb::gamma_function_inverse(color.b()),
        )
    }
}

impl From<Color<Srgb>> for Rgb<u8> {
    fn from(color: Color<Srgb>) -> Self {
        let range = 0.001..0.999;
        Rgb([
            (256.0 * range.clamp(color.r())) as u8,
            (256.0 * range.clamp(color.g())) as u8,
            (256.0 * range.clamp(color.b())) as u8,
        ])
    }
}
