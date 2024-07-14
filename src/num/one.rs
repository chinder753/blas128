use std::ops::{Add, Mul};

use super::complex::{Complex32, Complex64};

pub(crate) trait One:
    Sized + Add<Output = Self> + Mul<Output = Self> + Mul<Self, Output = Self>
{
    fn one() -> Self;
    fn is_one(&self) -> bool;
    fn set_one(&mut self) {
        *self = One::one();
    }
}
impl One for f32 {
    fn one() -> Self {
        1.0
    }
    fn is_one(&self) -> bool {
        *self == 1.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
    fn is_one(&self) -> bool {
        *self == 1.0
    }
}

impl One for Complex32 {
    fn one() -> Self {
        Self { rel: 1.0, img: 0.0 }
    }
    fn is_one(&self) -> bool {
        self.rel == 1.0 && self.img == 0.0
    }
}

impl One for Complex64 {
    fn one() -> Self {
        Self { rel: 1.0, img: 0.0 }
    }
    fn is_one(&self) -> bool {
        self.rel == 1.0 && self.img == 0.0
    }
}

pub(crate) fn one<T: One>() -> T {
    One::one()
}
