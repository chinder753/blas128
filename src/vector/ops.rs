use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{Vector, VectorElement};

mod ops_complex;
mod ops_real;

macro_rules! vector_ops_real_reload_impl {
    ($assign_bound:ident, $assign_method:ident, $bound:ident, $method:ident) => {
        impl<T: VectorElement> $assign_bound for Vector<T> {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.len, rhs.len);
                for i in 0..self.len {
                    $assign_bound::$assign_method(&mut self[i], rhs[i]);
                }
            }
        }

        impl<T: VectorElement> $bound for Vector<T> {
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
    () => {
        vector_ops_real_reload_impl!(AddAssign, add_assign, Add, add);
        vector_ops_real_reload_impl!(SubAssign, sub_assign, Sub, sub);
        vector_ops_real_reload_impl!(DivAssign, div_assign, Div, div);
        vector_ops_real_reload_impl!(MulAssign, mul_assign, Mul, mul);
    };
}

vector_ops_impl!();

impl<T: VectorElement> Neg for Vector<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = self;
        r.iter_mut().for_each(|x| *x = -(*x));
        r
    }
}
