use wasm_bindgen::prelude::wasm_bindgen;

mod ops;
mod ops_real;
mod ops_complex;

use crate::num::complex::{Complex32, Complex64};
use crate::num::float;
use crate::num::zero::zero;

macro_rules! vector {
    ($vec_name:ident, $t:ident) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone)]
        pub(crate) struct $vec_name {
            len: usize,
            value: Vec<$t>,
        }
    };
}

macro_rules! vector_static_impl {
    ($vec_name:ident, $t:ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            #[wasm_bindgen(constructor)]
            pub fn new(value: Vec<$t>) -> Self {
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

            pub fn fill(n: $t, len: usize) -> Self {
                Self {
                    len: len,
                    value: vec![n; len],
                }
            }
        }
    };
}

macro_rules! vector_method_impl {
    ($vec_name:ident, $t:ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            pub fn get_len(&self) -> usize {
                self.len
            }
        }
    };
}

macro_rules! vector_impl {
    ($vec_name:ident, $t:ident) => {
        vector!($vec_name, $t);
        vector_static_impl!($vec_name, $t);
        vector_method_impl!($vec_name, $t);
    };
}

vector_impl!(VectorF32, f32);
vector_impl!(VectorF64, f64);
vector_impl!(VectorC32, Complex32);
vector_impl!(VectorC64, Complex64);
