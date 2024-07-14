use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::{one::One, zero::Zero};

pub(crate) trait Float
where
    Self: Zero + One + Copy + Display,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Mul<Output = Self> + MulAssign,
{
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
}

macro_rules! float_func_impl {
    ($T:ident) => {
        impl Float for $T {
            fn sqrt(self) -> Self {
                Self::sqrt(self)
            }
            fn powi(self, n: i32) -> Self {
                Self::powi(self, n)
            }
        }
    };
}

float_func_impl!(f32);
float_func_impl!(f64);
