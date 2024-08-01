use std::simd::{LaneCount, Simd, SupportedLaneCount};

use crate::{num::float::Float, simd::V128};

use super::Complex;

pub(crate) trait ComplexToV128<T: Float>
where
    LaneCount<{ 16 / size_of::<T>() }>: SupportedLaneCount,
{
    fn to_v128(&self) -> V128<T>;
}

impl ComplexToV128<f64> for Complex<f64> {
    fn to_v128(&self) -> V128<f64> {
        Simd::from_array([self.re, self.im])
    }
}

impl ComplexToV128<f32> for Complex<f32> {
    fn to_v128(&self) -> V128<f32> {
        Simd::from_array([self.re, self.im, 0.0, 0.0])
    }
}
