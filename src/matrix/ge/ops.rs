use std::ops::{Add, AddAssign, Neg};
use std::ops::{Div, DivAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use crate::vector::vec_ele::VectorElement;

use super::MatrixGe;

macro_rules! matrix_ops_reload_impl {
    ($assign_bound:ident, $assign_method:ident, $bound:ident, $method:ident) => {
        impl<T: VectorElement> $assign_bound for MatrixGe<T> {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.row, rhs.row);
                assert_eq!(self.col, rhs.col);
                for i in 0..self.row {
                    for j in 0..self.col {
                        $assign_bound::$assign_method(&mut self[i][j], rhs[i][j]);
                    }
                }
            }
        }

        impl<T: VectorElement> $bound for MatrixGe<T> {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self::Output {
                assert_eq!(self.row, rhs.row);
                assert_eq!(self.col, rhs.col);
                let mut r = self.clone();
                $assign_bound::$assign_method(&mut r, rhs);
                r
            }
        }
    };
}

macro_rules! matrix_ops_impl {
    () => {
        matrix_ops_reload_impl!(AddAssign, add_assign, Add, add);
        matrix_ops_reload_impl!(SubAssign, sub_assign, Sub, sub);
        matrix_ops_reload_impl!(DivAssign, div_assign, Div, div);
        matrix_ops_reload_impl!(MulAssign, mul_assign, Mul, mul);

        impl<T: VectorElement> Neg for MatrixGe<T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let mut r = self;
                for i in 0..r.row {
                    for j in 0..r.col {
                        r[i][j] = -r[i][j];
                    }
                }
                r
            }
        }
    };
}

matrix_ops_impl!();
