use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vector::vecele::VectorElement;

pub mod ge;
pub mod sy;


macro_rules! matrix_ops_reload_impl {
    ($assign_bound:ident, $assign_method:ident, $bound:ident, $method:ident) => {
            fn $assign_method(&mut self, rhs: Self) {
                self.is_same_shape(&rhs);
                for i in 0..self.row() {
                    for j in 0..self.col() {
                        $assign_bound::$assign_method(&mut self[i][j], rhs[i][j]);
                    }
                }
            }

            fn $method(self, rhs: Self) -> Self {
                self.is_same_shape(&rhs);
                let mut r = self.clone();
                $assign_bound::$assign_method(&mut r, rhs);
                r
            }
    };
}

macro_rules! matrix_ops_impl {
    () => {
        matrix_ops_reload_impl!(AddAssign, add_assign, Add, add);
        matrix_ops_reload_impl!(SubAssign, sub_assign, Sub, sub);
        matrix_ops_reload_impl!(DivAssign, div_assign, Div, div);
        matrix_ops_reload_impl!(MulAssign, mul_assign, Mul, mul);

        fn neg(self) -> Self {
            let mut r = self;
            for i in 0..r.row(){
                for j in 0..r.col(){
                    r[i][j] = -r[i][j];
                }
            }
            r
        }
    };
}

pub trait Matrix<T: VectorElement>
where
    Self: Debug + Clone,
    Self: Index<usize, Output = [T]> + IndexMut<usize, Output = [T]>,
    Self: Index<[usize;2], Output = T> + IndexMut<[usize;2], Output = T>,
    Self: Neg<Output=Self>,
    Self: Add<Output=Self> + AddAssign,
    Self: Sub<Output=Self> + SubAssign,
    Self: Div<Output=Self> + DivAssign,
    Self: Mul<Output=Self> + MulAssign,
{
    fn row(&self) -> usize;
    fn col(&self) -> usize;

    fn new(value: Vec<Vec<T>>) -> Self;
    fn zero(shape: (usize, usize)) -> Self;
    fn eye(shape: (usize, usize)) -> Self;
    fn from_array<const R: usize, const C: usize>(array: [[T; C]; R]) -> Self;

    fn is_same_shape(&self, rhs: &Self){
        assert_eq!(self.row(), rhs.row());
        assert_eq!(self.col(), rhs.col());
    }

    matrix_ops_impl!();
}

