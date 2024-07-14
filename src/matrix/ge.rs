use crate::matrix::Matrix;
use crate::vector::vecele::VectorElement;

mod ops;
mod index;
mod impl_matrix;

#[derive(Debug, Clone)]
pub struct MatrixGe<T: VectorElement>
where
    Self: Matrix<T>,
{
    row: usize,
    col: usize,
    value: Vec<T>,
}
