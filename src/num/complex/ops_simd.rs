use std::{
    ops::{Add, Sub},
    simd::{LaneCount, Simd, SupportedLaneCount},
};

use crate::{num::float::Float, simd::V128};

use super::{
    ComplexBasis, ComplexMath,
    {simd::ComplexToV128, Complex},
};

macro_rules! complex_ops_simd_trait_impl {
    ($(($method_assign:ident, $method:ident, $bound:ident, $bound_method: ident)),*) => {
        pub trait ComplexOpsSimd<T: Float>
        where
        Self: Sized,
        Self: ComplexBasis<T>+ComplexMath<T>+ComplexToV128<T>,
        LaneCount<{ 16 / size_of::<T>() }>: SupportedLaneCount,
        V128<T>: Add<Output=V128<T>>,
        V128<T>: Sub<Output=V128<T>>,
        {
            $(
                fn $method_assign(&mut self, rhs: Self) {
                    let v1 = self.to_v128();
                    let v2 = rhs.to_v128();
                    let v3 = $bound::$bound_method(v1, v2);
                    self.set_re(v3[0]);
                    self.set_im(v3[1]);
                }

                fn $method(self, rhs: Self) -> Self {
                    let v1 = self.to_v128();
                    let v2 = rhs.to_v128();
                    let v3 = $bound::$bound_method(v1, v2);
                    Self::new(v3[0], v3[1])
                }
            )*

            fn mul_simd(self, rhs: Self) -> Self {
                let mut r = self;
                r *= rhs;
                r
            }
            fn div_assign_simd(&mut self, rhs: Self) {
                let nrm2 = rhs.nrm2();
                *self *= rhs.conj();
                self.set_re(self.re()/nrm2);
                self.set_im(self.im()/nrm2);
            }
            fn div_simd(self, rhs: Self) -> Self {
                let mut r = self;
                r /= rhs;
                r
            }
        }
    };
}

complex_ops_simd_trait_impl!(
    (add_assign_simd, add_simd, Add, add),
    (sub_assign_simd, sub_simd, Sub, sub)
);

impl ComplexOpsSimd<f32> for Complex<f32> {}
impl ComplexOpsSimd<f64> for Complex<f64> {}

impl Complex<f32> {
    fn mul_assign_simd(&mut self, rhs: Self) {
        let v1 = Simd::from_array([self.re, self.im, self.re, self.im]);
        let v2 = Simd::from_array([rhs.re, rhs.im, rhs.im, rhs.re]);
        let v3 = v1 * v2;
        [self.re, self.im] = [v3[0] - v3[1], v3[2] + v3[3]]
    }
}

impl Complex<f64> {
    fn mul_assign_simd(&mut self, rhs: Self) {
        let v_self = Simd::from_array([self.re, self.im]);
        let v_rhs = Simd::from_array([rhs.re, rhs.im]);
        let v_irhs = Simd::from_array([rhs.im, rhs.re]);
        let vr = v_self * v_rhs;
        let vi = v_self * v_irhs;
        [self.re, self.im] = [vr[0] - vr[1], vi[0] - vi[1]]
    }
}
