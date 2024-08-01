use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::num::{complex::Complex, float::Float, one::One, zero::Zero};

pub trait VectorElement
where
    Self: Debug + Copy,
    Self: Zero + One,
    Self: PartialEq + PartialOrd,
    Self: Neg<Output = Self>,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Mul<Output = Self> + MulAssign,
{
}

impl VectorElement for f64 {}
impl VectorElement for f32 {}

impl<T: Float> VectorElement for Complex<T> {}
