use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::num::{
    complex::Complex,
    float::{self, Float},
    one::One,
    zero::{zero, Zero},
};

pub trait VectorElement
where
    Self: Zero + One,
    Self: Debug + Copy,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Mul<Output = Self> + MulAssign,
{
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}

impl VectorElement for f64 {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
impl VectorElement for f32 {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
impl VectorElement for Complex<f64> {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
impl VectorElement for Complex<f32> {
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
}
