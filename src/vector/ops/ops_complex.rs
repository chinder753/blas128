use std::{
    mem::size_of,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use crate::num::complex::Complex;

use super::Vector;

macro_rules! vector_ops_complex_reload_impl {
    ($t:ident, $method: ident, $assign_method: ident, $assign_bound:ident, $bound_assign_method:ident, $bound:ident) => {
        impl Vector<Complex<$t>> {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.len, rhs.len);
                const LANE: usize = 16 / size_of::<$t>();
                let mut i = 0;
                for i in 0..self.len {
                    $assign_bound::$bound_assign_method(&mut self[i], rhs[i]);
                }
            }

            fn $method(self, rhs: Self) -> Self {
                assert_eq!(self.len, rhs.len);
                let mut r = self.clone();
                $assign_bound::$bound_assign_method(&mut r, rhs);
                r
            }
        }
    };
}

macro_rules! vector_ops_impl {
    ($t:ident) => {
        vector_ops_complex_reload_impl!($t, add_simd, add_assign_simd, AddAssign, add_assign, Add);
        vector_ops_complex_reload_impl!($t, sub_simd, sub_assign_simd, SubAssign, sub_assign, Sub);
        vector_ops_complex_reload_impl!($t, div_simd, div_assign_simd, DivAssign, div_assign, Div);
        vector_ops_complex_reload_impl!($t, mul_simd, mul_assign_simd, MulAssign, mul_assign, Mul);
    };
}

vector_ops_impl!(f32);
vector_ops_impl!(f64);
