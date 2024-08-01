use crate::matrix::Matrix;
use crate::vector::vec_ele::VectorElement;

mod impl_matrix;
mod index;
mod ops;

#[derive(Debug, Clone)]
pub struct MatrixGe<T: VectorElement>
where
    Self: Matrix<T>,
{
    row: usize,
    col: usize,
    value: Vec<T>,
}
