use crate::wasm_bindgen;

mod real;
mod complex;
mod rot_complex;
mod rot_real;

use crate::num::complex::{Complex32, Complex64};
use crate::num::float;
use crate::num::zero::zero;
use crate::vector::{VectorC32, VectorC64};
use crate::vector::{VectorF32, VectorF64};

macro_rules! blas_leve1_impl {
    ($vec_name:ident, $t:ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            pub fn axpy(&mut self, a: $t, y: Self) {
                assert_eq!(self.get_len(), y.get_len());
                for i in 0..self.get_len() {
                    self[i] = self[i] * a + y[i];
                }
            }
            pub fn scal(&mut self, rhs: $t) {
                for i in 0..self.get_len() {
                    self[i] *= rhs;
                }
            }
            pub fn swap(x: &mut $vec_name, y: &mut $vec_name) -> () {
                assert_eq!(x.get_len(), y.get_len());
                for i in 0..x.get_len() {
                    let temp_v = x[i];
                    x[i] = y[i];
                    y[i] = temp_v;
                }
            }
            pub fn iamax(self) -> $t {
                let mut max: $t = self[0];
                for i in 1..self.get_len() {
                    max = max.max(self[i]);
                }
                max
            }
            pub fn iamin(self) -> $t {
                let mut min: $t = self[0];
                for i in 1..self.get_len() {
                    min = min.min(self[i]);
                }
                min
            }
        }
    };
}

blas_leve1_impl!(VectorF32, f32);
blas_leve1_impl!(VectorF64, f64);
blas_leve1_impl!(VectorC32, Complex32);
blas_leve1_impl!(VectorC64, Complex64);
