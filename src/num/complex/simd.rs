use std::simd::Simd;

use crate::simd::V128;

use super::Complex;

impl Complex<f64> {
    pub(crate) fn to_v128(&self) -> V128<f64> {
        Simd::from_array([self.rel, self.img])
    }
}

impl Complex<f32> {
    pub(crate) fn to_v128(&self) -> V128<f32> {
        Simd::from_array([self.rel, self.img, 0.0, 0.0])
    }
}
