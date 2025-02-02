use std::ops::{Add, Mul};

use super::{complex::Complex, float::Float};

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

impl<T: Float> One for Complex<T> {
    fn one() -> Self {
        Self {
            re: T::one(),
            im: T::zero(),
        }
    }
    fn is_one(&self) -> bool {
        self.re == T::one() && self.im == T::zero()
    }
}

pub(crate) fn one<T: One>() -> T {
    One::one()
}
