use std::mem::size_of;
use std::simd::Simd;

use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use super::Vector;

macro_rules! vector_ops_real_reload_impl {
    ($t:ident, $assign_method:ident, $method:ident, $assign_bound:ident, $assign_bound_method:ident, $bound:ident, $bound_method:ident) => {
        impl Vector<$t> {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.len, rhs.len);
                let (self_pre, self_mid, self_end) = self.iter_simd_mut();
                let (rhs_pre, rhs_mid, rhs_end) = rhs.iter_simd();
                self_pre
                    .iter_mut()
                    .zip(rhs_pre.iter())
                    .for_each(|(v1, v2)| *v1 = $bound::$bound_method(*v1, *v2));
                self_mid
                    .iter_mut()
                    .zip(rhs_mid.iter())
                    .for_each(|(v1, v2)| *v1 = $bound::$bound_method(*v1, *v2));
                self_end
                    .iter_mut()
                    .zip(rhs_end.iter())
                    .for_each(|(v1, v2)| *v1 = $bound::$bound_method(*v1, *v2));
            }

            fn $method(self, rhs: Self) -> Self {
                assert_eq!(self.len, rhs.len);
                let mut r = self.clone();
                $assign_bound::$assign_bound_method(&mut r, rhs);
                r
            }
        }
    };
}

macro_rules! vector_ops_impl {
    ($t:ident) => {
        vector_ops_real_reload_impl!(
            $t,
            add_assign_simd,
            add_simd,
            AddAssign,
            add_assign,
            Add,
            add
        );
        vector_ops_real_reload_impl!(
            $t,
            sub_assign_simd,
            sub_simd,
            SubAssign,
            sub_assign,
            Sub,
            sub
        );
        vector_ops_real_reload_impl!(
            $t,
            div_assign_simd,
            div_simd,
            DivAssign,
            div_assign,
            Div,
            div
        );
        vector_ops_real_reload_impl!(
            $t,
            mul_assign_simd,
            mul_simd,
            MulAssign,
            mul_assign,
            Mul,
            mul
        );
    };
}

vector_ops_impl!(f32);
vector_ops_impl!(f64);
