use rand::Rng;
use std::ops::Range;

pub const INFINITY: f32 = f32::INFINITY;

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
        if value < self.start {
            self.start
        } else if value > self.end {
            self.end
        } else {
            value
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
