use std::fmt::Debug;

use crate::num::zero::zero;

use vec_ele::VectorElement;

mod index;
mod iter;
mod ops;

pub mod vec_ele;

#[derive(Debug, Clone)]
pub struct Vector<T: VectorElement> {
    len: usize,
    value: Vec<T>,
}

pub trait VectorBasic<T: VectorElement> {
    fn new(value: Vec<T>) -> Self;
    fn zero(len: usize) -> Self;
    fn fill(n: T, len: usize) -> Self;
    fn get_len(&self) -> usize;
}

impl<T: VectorElement> VectorBasic<T> for Vector<T> {
    fn new(value: Vec<T>) -> Self {
        Self {
            len: value.len(),
            value: value.clone(),
        }
    }

    fn zero(len: usize) -> Self {
        Self {
            len,
            value: vec![zero(); len],
        }
    }

    fn fill(n: T, len: usize) -> Self {
        Self {
            len,
            value: vec![n; len],
        }
    }

    fn get_len(&self) -> usize {
        self.len
    }
}
