use crate::wasm_bindgen;

use crate::num::zero::zero;
use crate::num::complex::{Complex32, Complex64};
use crate::vector::{VectorC32, VectorC64};

macro_rules! blas_complex_leve1_impl {
    ($vec_name:ident, $t:ident, $ot: ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            pub fn asum(&self) -> $ot {
                let mut r: $ot = zero();
                for i in 0..self.get_len() {
                    r = r + self[i].nrm2();
                }
                r
            }
            pub fn dotc(&self, rhs: Self) -> $t {
                let mut sum = zero();
                for i in 0..self.get_len() {
                    sum += self[i].conj() * rhs[i]
                }
                sum
            }
            pub fn dotu(&self, rhs: Self) -> $t {
                let mut sum = zero();
                for i in 0..self.get_len() {
                    sum += self[i] * rhs[i]
                }
                sum
            }
            pub fn nrm2(&self) -> $ot {
                let mut r: $ot = zero();
                for i in 0..self.get_len() {
                    r += self[i].nrm2();
                }
                r
            }
        }
    };
}

blas_complex_leve1_impl!(VectorC32, Complex32, f32);
blas_complex_leve1_impl!(VectorC64, Complex64, f64);