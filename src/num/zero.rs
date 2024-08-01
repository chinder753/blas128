use std::ops::{Add, Mul};

use super::{complex::Complex, float::Float};

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
impl<T: Float> Zero for Complex<T> {
    fn zero() -> Self {
        Self {
            re: T::zero(),
            im: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.re == T::zero() && self.im == T::zero()
    }
}

pub(crate) fn zero<T: Zero>() -> T {
    Zero::zero()
}
