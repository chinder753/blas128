use crate::wasm_bindgen;

use crate::num::{float, zero::zero};
use crate::vector::{VectorF32, VectorF64};

macro_rules! blas_real_leve1_impl {
    ($vec_name:ident, $t:ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            pub fn asum(&self) -> $t {
                let mut r: $t = zero();
                for i in 0..self.get_len() {
                    r = r + self[i];
                }
                r
            }
            pub fn dot(&self, rhs: Self) -> $t {
                let mut sum = zero();
                for i in 0..self.get_len() {
                    sum += self[i] * rhs[i]
                }
                sum
            }
            pub fn nrm2(&self) -> $t {
                let mut r: $t = zero();
                for i in 0..self.get_len() {
                    r += self[i].powi(2);
                }
                r
            }
        }
    };
}

blas_real_leve1_impl!(VectorF32, f32);
blas_real_leve1_impl!(VectorF64, f64);
