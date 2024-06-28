use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};
use std::simd::Simd;

use super::super::float::Float;
use super::Complex;

macro_rules! complex_ops_raw_impl {
    ($vec_name:ident, $t: ident, $lane: expr, $method_assign:ident, $method:ident, $bound:ident, $bound_method: ident) => {
        impl $vec_name<$t> {
            fn $method_assign(&mut self, rhs: Self) {
                let v1 = self.to_v128();
                let v2 = rhs.to_v128();
                let v3 = $bound::$bound_method(v1, v2);
                self.rel = v3[0];
                self.img = v3[1];
            }

            fn $method(self, rhs: Self) -> Self {
                let v1 = self.to_v128();
                let v2 = rhs.to_v128();
                let v3 = $bound::$bound_method(v1, v2);
                Self {
                    rel: v3[0],
                    img: v3[1],
                }
            }
        }
    };
}

macro_rules! complex_mul_div_impl {
    ($vec_name:ident, $t: ident) => {
        impl $vec_name<$t> {
            fn mul_simd(self, rhs: Self) -> Self {
                let mut r = self;
                r *= rhs;
                r
            }
            fn div_assign_simd(&mut self, rhs: Self) {
                let nrm2 = rhs.nrm2();
                *self *= rhs.conj();
                self.rel /= nrm2;
                self.img /= nrm2;
            }
            fn div_simd(self, rhs: Self) -> Self {
                let mut r = self;
                r /= rhs;
                r
            }
        }
    };
}

macro_rules! complex_ops_impl {
    ($vec_name:ident, $t: ident, $lane: expr) => {
        complex_ops_raw_impl!($vec_name, $t, $lane, add_assign_simd, add_simd, Add, add);
        complex_ops_raw_impl!($vec_name, $t, $lane, sub_assign_simd, sub_simd, Sub, sub);
        complex_mul_div_impl!($vec_name, $t);
    };
}

complex_ops_impl!(Complex, f64, F64_LANE);
complex_ops_impl!(Complex, f32, F32_LANE);

impl Complex<f32> {
    fn mul_assign_simd(&mut self, rhs: Self) {
        let v1 = Simd::from_array([self.rel, self.img, self.rel, self.img]);
        let v2 = Simd::from_array([rhs.rel, rhs.img, rhs.img, rhs.rel]);
        let v3 = v1 * v2;
        [self.rel, self.img] = [v3[0] - v3[1], v3[2] + v3[3]]
    }
}

impl Complex<f64> {
    fn mul_assign_simd(&mut self, rhs: Self) {
        let v_self = Simd::from_array([self.rel, self.img]);
        let v_rhs = Simd::from_array([rhs.rel, rhs.img]);
        let v_irhs = Simd::from_array([rhs.img, rhs.rel]);
        let vr = v_self * v_rhs;
        let vi = v_self * v_irhs;
        [self.rel, self.img] = [vr[0] - vr[1], vi[0] - vi[1]]
    }
}
