use std::{
    simd::Simd,
    slice::{Chunks, Iter, IterMut},
};

use crate::{
    num::complex::Complex,
    simd::{F32_LANE, F64_LANE, V128},
};

use super::{Vector, VectorElement};

impl<T: VectorElement> Vector<T> {
    pub fn chunks(&self, chunk_size: usize) -> Chunks<'_, T> {
        self.value.chunks(chunk_size)
    }
    pub fn iter(&self) -> Iter<'_, T> {
        self.value.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.value.iter_mut()
    }
}

impl Vector<f32> {
    pub fn iter_simd(&self) -> (&[f32], &[V128<f32>], &[f32]) {
        self.value.as_simd()
    }
    pub fn iter_simd_mut(&mut self) -> (&mut [f32], &mut [V128<f32>], &mut [f32]) {
        self.value.as_simd_mut()
    }
}

impl Vector<f64> {
    pub fn iter_simd(&self) -> (&[f64], &[V128<f64>], &[f64]) {
        self.value.as_simd()
    }
    pub fn iter_simd_mut(&mut self) -> (&mut [f64], &mut [V128<f64>], &mut [f64]) {
        self.value.as_simd_mut()
    }
}

impl Vector<Complex<f32>> {
    pub fn iter_simd(&self) -> (&[Complex<f32>], &[V128<f32>], &[Complex<f32>]) {
        unsafe { self.value.align_to() }
    }
    pub fn iter_simd_mut(
        &mut self,
    ) -> (&mut [Complex<f32>], &mut [V128<f32>], &mut [Complex<f32>]) {
        unsafe { self.value.align_to_mut() }
    }
}

impl Vector<Complex<f64>> {
    pub fn iter_simd(&self) -> (&[Complex<f64>], &[V128<f64>], &[Complex<f64>]) {
        unsafe { self.value.align_to() }
    }
    pub fn iter_simd_mut(
        &mut self,
    ) -> (&mut [Complex<f64>], &mut [V128<f64>], &mut [Complex<f64>]) {
        unsafe { self.value.align_to_mut() }
    }
}
