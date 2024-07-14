use std::mem::size_of;
use std::simd::Simd;

use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use crate::num::complex::{Complex32, Complex64};

use super::{VectorC32, VectorC64};


macro_rules! vector_ops_complex_reload_impl {
    ($vec_name:ident, $st:ident, $t:ident, $assign_bound:ident, $assign_method:ident, $bound:ident, $method:ident) => {
        impl $assign_bound for $vec_name {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.len, rhs.len);
                const LANE: usize = 16 / size_of::<$t>();
                let mut i = 0;
                for i in 0..self.len{
                    $assign_bound::$assign_method(&mut self[i], rhs[i]);
                }
            }
        }

        impl $bound for $vec_name {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self::Output {
                assert_eq!(self.len, rhs.len);
                let mut r = self.clone();
                $assign_bound::$assign_method(&mut r, rhs);
                r
            }
        }
    };
}

macro_rules! vector_ops_impl {
    ($vec_name:ident, $st:ident, $t:ident) => {
        vector_ops_complex_reload_impl!($vec_name,  $st, $t, AddAssign, add_assign, Add, add);
        vector_ops_complex_reload_impl!($vec_name,  $st, $t, SubAssign, sub_assign, Sub, sub);
        vector_ops_complex_reload_impl!($vec_name,  $st, $t, DivAssign, div_assign, Div, div);
        vector_ops_complex_reload_impl!($vec_name,  $st, $t, MulAssign, mul_assign, Mul, mul);
    };
}

vector_ops_impl!(VectorC32, f32, Complex32);
vector_ops_impl!(VectorC64, f64, Complex64);