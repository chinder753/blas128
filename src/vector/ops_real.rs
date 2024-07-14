use std::mem::size_of;
use std::simd::Simd;

use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use super::{VectorF32, VectorF64};


macro_rules! vector_ops_real_reload_impl {
    ($vec_name:ident, $t:ident, $assign_bound:ident, $assign_method:ident, $bound:ident, $method:ident) => {
        impl $assign_bound for $vec_name {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.len, rhs.len);
                const LANE: usize = 16 / size_of::<$t>();
                let mut i = 0;
                while(i<self.len-LANE){
                    let v1 = crate::v128_load_real_vector!($t, self, i);
                    let v2 = crate::v128_load_real_vector!($t, rhs, i);
                    let v3 = $bound::$method(v1, v2);
                    crate::v128_store_real_vector!($t, v3, self, i);
                    i+=LANE;
                }
                while(i<self.len){
                    $assign_bound::$assign_method(&mut self[i], rhs[i]);
                    i+=1;
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
    ($vec_name:ident, $t:ident) => {
        vector_ops_real_reload_impl!($vec_name, $t, AddAssign, add_assign, Add, add);
        vector_ops_real_reload_impl!($vec_name, $t, SubAssign, sub_assign, Sub, sub);
        vector_ops_real_reload_impl!($vec_name, $t, DivAssign, div_assign, Div, div);
        vector_ops_real_reload_impl!($vec_name, $t, MulAssign, mul_assign, Mul, mul);
    };
}

vector_ops_impl!(VectorF32, f32);
vector_ops_impl!(VectorF64, f64);