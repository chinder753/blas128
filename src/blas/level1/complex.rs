use crate::{
    num::{complex::Complex, zero::zero},
    vector::Vector,
};

macro_rules! blas_complex_leve1_impl {
    ($t:ident) => {
        impl Vector<Complex<$t>> {
            pub fn asum(&self) -> $t {
                let mut r: $t = zero();
                for i in 0..self.get_len() {
                    r = r + self[i].nrm2();
                }
                r
            }
            pub fn dotc(&self, rhs: Self) -> Complex<$t> {
                let mut sum = zero();
                for i in 0..self.get_len() {
                    sum += self[i].conj() * rhs[i]
                }
                sum
            }
            pub fn dotu(&self, rhs: Self) -> Complex<$t> {
                let mut sum = zero();
                for i in 0..self.get_len() {
                    sum += self[i] * rhs[i]
                }
                sum
            }
            pub fn nrm2(&self) -> $t {
                let mut r: $t = zero();
                for i in 0..self.get_len() {
                    r += self[i].nrm2();
                }
                r
            }
        }
    };
}

blas_complex_leve1_impl!(f32);
blas_complex_leve1_impl!(f64);
