use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};
use std::simd::Simd;

use super::super::float::Float;
use super::{Complex32, Complex64};

macro_rules! complex_ops_raw_impl {
    ($vec_name:ident, $t:ident, $bound_assign:ident, $method_assign:ident, $bound:ident, $method:ident) => {
        impl $bound_assign for $vec_name {
            fn $method_assign(&mut self, rhs: Self) {
                let v1 = crate::v128_load_complex!($t, self);
                let v2 = crate::v128_load_complex!($t, rhs);
                let v3 = $bound::$method(v1, v2);
                self.rel = v3[0];
                self.img = v3[1];
            }
        }

        impl $bound for $vec_name {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self {
                let v1 = crate::v128_load_complex!($t, self);
                let v2 = crate::v128_load_complex!($t, rhs);
                let v3 = $bound::$method(v1, v2);
                Self {
                    rel: v3[0],
                    img: v3[1],
                }
            }
        }
    };
}

macro_rules! complex_mul_div_impl {
    ($vec_name:ident, $t:ident) => {

        impl Mul for $vec_name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut r = self;
                r *= rhs;
                r
            }
        }

        impl DivAssign for $vec_name {
            fn div_assign(&mut self, rhs: Self) {
                let nrm2 = rhs.nrm2();
                *self *= rhs.conj();
                self.rel /= nrm2;
                self.img /= nrm2;
            }
        }

        impl Div for $vec_name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let mut r = self;
                r /= rhs;
                r
            }
        }
    };
}

macro_rules! complex_ops_impl {
    ($vec_name:ident, $t:ident) => {
        complex_ops_raw_impl!($vec_name, $t, AddAssign, add_assign, Add, add);
        complex_ops_raw_impl!($vec_name, $t, SubAssign, sub_assign, Sub, sub);
        complex_mul_div_impl!($vec_name, $t);
    };
}

complex_ops_impl!(Complex32, f32);
complex_ops_impl!(Complex64, f64);



impl MulAssign for Complex32 {
    fn mul_assign(&mut self, rhs: Self) {
        let v1 = Simd::from_array([self.rel, self.img, self.rel, self.img]);
        let v2 = Simd::from_array([rhs.rel, rhs.img, rhs.img, rhs.rel]);
        let v3 = v1 * v2;
        [self.rel, self.img] = [
            v3[0] - v3[1],
            v3[2] + v3[3],
        ]
    }
}

impl MulAssign for Complex64 {
    fn mul_assign(&mut self, rhs: Self) {
        let v_self = Simd::from_array([self.rel, self.img]);
        let v_rhs = Simd::from_array([rhs.rel, rhs.img]);
        let v_irhs = Simd::from_array([rhs.img, rhs.rel]);
        let vr = v_self * v_rhs;
        let vi = v_self * v_irhs;
        [self.rel, self.img] = [
            vr[0] - vr[1],
            vi[0] - vi[1],
        ]
    }
}
