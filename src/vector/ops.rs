use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use std::any::type_name;
use std::mem::size_of;

use crate::wasm_bindgen;

use super::{VectorC32, VectorC64};
use super::{VectorF32, VectorF64};
use crate::num::complex::{Complex32, Complex64};
use crate::num::float;

macro_rules! vector_index_reload_impl {
    ($vec_name:ident, $t:ident) => {
        impl Index<usize> for $vec_name {
            type Output = $t;

            fn index(&self, index: usize) -> &Self::Output {
                &self.value[index]
            }
        }

        impl IndexMut<usize> for $vec_name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.value[index]
            }
        }
    };
}

macro_rules! vector_ops_func_impl {
    ($vec_name:ident, $t:ident, $assign_method:ident, $method:ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            pub fn $assign_method(mut self, rhs: Self) {
                assert_eq!(self.len, rhs.len);
                $vec_name::$assign_method(self, rhs);
            }
            pub fn $method(self, rhs: Self) -> $vec_name {
                assert_eq!(self.len, rhs.len);
                let mut r = self.clone();
                $vec_name::$method(self, rhs);
                r
            }
        }
    };
}

macro_rules! vector_ops_impl {
    ($vec_name:ident, $t:ident) => {
        vector_index_reload_impl!($vec_name, $t);
        vector_ops_func_impl!($vec_name, $t, add_assign, add);
        vector_ops_func_impl!($vec_name, $t, sub_assign, sub);
        vector_ops_func_impl!($vec_name, $t, div_assign, div);
        vector_ops_func_impl!($vec_name, $t, mul_assign, mul);
    };
}

vector_ops_impl!(VectorF32, f32);
vector_ops_impl!(VectorF64, f64);
vector_ops_impl!(VectorC32, Complex32);
vector_ops_impl!(VectorC64, Complex64);
