use std::ops::Range;

use rand::Rng;

use crate::color::{Color, Linear};

pub const DEBUG_COLOR: Color<Linear> = Color::new(1.0, 0.0, 1.0);

pub trait RangeExt<T> {
    fn surrounds(&self, value: &T) -> bool;
    fn clamp(&self, value: T) -> T;
}

impl<T> RangeExt<T> for Range<T>
where
    T: PartialOrd + Copy,
{
    #[inline]
    fn surrounds(&self, value: &T) -> bool {
        self.start < *value && *value < self.end
    }

    #[inline]
    fn clamp(&self, value: T) -> T {
        match value {
            x if x < self.start => self.start,
            x if x > self.end => self.end,
            x => x,
        }
    }
}

pub trait Random<T>: Sized {
    fn random() -> Self;
    fn random_range(range: &Range<T>) -> Self;
}

impl Random<f32> for f32 {
    #[inline]
    fn random() -> f32 {
        rand::random::<f32>()
    }

    #[inline]
    fn random_range(range: &Range<f32>) -> f32 {
        rand::rng().random_range(range.start..range.end)
    }
}
