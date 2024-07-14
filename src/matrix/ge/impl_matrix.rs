use crate::matrix::ge::MatrixGe;
use crate::matrix::Matrix;
use crate::num::one::one;
use crate::num::zero::zero;
use crate::vector::vecele::VectorElement;

impl<T: VectorElement> Matrix<T> for MatrixGe<T> {
    fn row(&self) -> usize {
        self.row
    }
    
    fn col(&self) -> usize {
        self.col
    }


    fn new(value: Vec<Vec<T>>) -> Self {
        let row = value.len();
        let mut col = value[0].len();
        for i in 1..row {
            assert_ne!(col, value[i].len());
            col = value[i].len();
        }
        Self {
            row,
            col,
            value: value.concat(),
        }
    }

    fn zero(shape: (usize, usize)) -> Self {
        Self {
            row: shape.0,
            col: shape.1,
            value: vec![zero::<T>(); shape.0 * shape.1],
        }
    }

    fn eye(shape: (usize, usize)) -> Self {
        let mut r = Self::zero(shape);
        for i in 0..shape.0.min(shape.1) {
            r[i][i] = one();
        }
        r
    }


    fn from_array<const R: usize, const C: usize>(array: [[T; C]; R]) -> Self {
        Self{
            row: R,
            col: C,
            value: array.iter().flat_map(|x| *x).collect(),
        }
    }
}
