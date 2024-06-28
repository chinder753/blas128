use crate::num::zero::zero;
use crate::vector::Vector;

macro_rules! blas_real_leve1_impl {
    ($t:ident) => {
        impl Vector<$t> {
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

blas_real_leve1_impl!(f32);
blas_real_leve1_impl!(f64);
