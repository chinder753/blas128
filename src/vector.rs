use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

mod index;
mod iter;
mod ops;
mod ops_complex;
mod ops_real;
mod simd;
pub mod vecele;

use vecele::VectorElement;

use crate::num::{
    complex::Complex,
    float::{self, Float},
    one::One,
    zero::{zero, Zero},
};

#[derive(Debug, Clone)]
pub struct Vector<T: VectorElement> {
    len: usize,
    value: Vec<T>,
}

impl<T: VectorElement> Vector<T> {
    pub fn new(value: Vec<T>) -> Self {
        Self {
            len: value.len(),
            value: value.clone(),
        }
    }

    pub fn zero(len: usize) -> Self {
        Self {
            len,
            value: vec![zero(); len],
        }
    }

    pub fn fill(n: T, len: usize) -> Self {
        Self {
            len,
            value: vec![n; len],
        }
    }

    pub fn get_len(&self) -> usize {
        self.len
    }
}
