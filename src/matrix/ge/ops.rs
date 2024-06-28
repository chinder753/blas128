use std::ops::{Add, AddAssign};
use std::ops::{Div, DivAssign};
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use crate::num::float::Float;

use super::MatrixGe;

impl<T: Float> Index<(usize, usize)> for MatrixGe<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &(self.value[index.0 * self.col + index.1])
    }
}

impl<T: Float> IndexMut<(usize, usize)> for MatrixGe<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut (self.value[index.0 * self.col + index.1])
    }
}

macro_rules! matrix_ops_reload_impl {
    ($assign_bound:ident, $assign_method:ident, $bound:ident, $method:ident) => {
        impl<T: Float> $assign_bound for MatrixGe<T> {
            fn $assign_method(&mut self, rhs: Self) {
                assert_eq!(self.row, rhs.row);
                assert_eq!(self.col, rhs.col);
                for i in 0..self.row {
                    for j in 0..self.col {
                        $assign_bound::$assign_method(&mut self[(i, j)], rhs[(i, j)]);
                    }
                }
            }
        }

        impl<T: Float> $bound for MatrixGe<T> {
            type Output = MatrixGe<T>;
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
    };
}

matrix_ops_impl!();
