use std::ops::{Add, Mul};

use super::complex::{Complex32, Complex64};

pub(crate) trait Zero:
    Sized + Add<Output = Self> + Mul<Output = Self> + Mul<Self, Output = Self>
{
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}
impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}
impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}
impl Zero for Complex32 {
    fn zero() -> Self {
        Self { rel: 0.0, img: 0.0 }
    }
    fn is_zero(&self) -> bool {
        self.rel == 0.0 && self.img == 0.0
    }
}
impl Zero for Complex64 {
    fn zero() -> Self {
        Self { rel: 0.0, img: 0.0 }
    }
    fn is_zero(&self) -> bool {
        self.rel == 0.0 && self.img == 0.0
    }
}

pub(crate) fn zero<T: Zero>() -> T {
    Zero::zero()
}
