use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{float::Float, one::One, zero::Zero};

mod ops;
mod ops_simd;
mod simd;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

pub trait ComplexBasis<T: Float>
where
    Self: Sized,
    Self: Debug,
    Self: Clone + Copy,
    Self: Zero + One,
{
    fn new(re: T, im: T) -> Self;
    fn re(&self) -> T;
    fn set_re(&mut self, re: T) -> &mut Self;
    fn im(&self) -> T;
    fn set_im(&mut self, im: T) -> &mut Self;
}

pub trait ComplexMath<T: Float>
where
    Self: ComplexBasis<T>,
    Self: Neg<Output = Self>,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Mul<Output = Self> + MulAssign,
{
    fn conj(self) -> Self {
        Self::new(self.re(), -self.im())
    }
    fn inv(self) -> Self {
        let nrm2 = self.nrm2();
        Self::new(self.re() / nrm2, -self.im() / nrm2)
    }
    fn nrm2(&self) -> T {
        self.re().powi(2) + self.im().powi(2)
    }
    fn max(self, other: Self) -> Self {
        let a_sum = self.re() + self.im();
        let b_sum = other.re() + other.im();
        if a_sum > b_sum {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self {
        let a_sum = self.re() + self.im();
        let b_sum = other.re() + other.im();
        if a_sum < b_sum {
            self
        } else {
            other
        }
    }
}

impl<T: Float> ComplexBasis<T> for Complex<T> {
    fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    fn re(&self) -> T {
        self.re
    }

    fn set_re(&mut self, re: T) -> &mut Self {
        self.re = re;
        self
    }

    fn im(&self) -> T {
        self.im
    }

    fn set_im(&mut self, im: T) -> &mut Self {
        self.im = im;
        self
    }
}

impl<T: Float> ComplexMath<T> for Complex<T> {}
