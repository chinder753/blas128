use crate::num::{float::Float, one::one, zero::zero};

mod ops;

#[derive(Debug, Clone)]
struct MatrixGe<T: Float> {
    row: usize,
    col: usize,
    value: Vec<T>,
}

impl<T: Float> MatrixGe<T> {
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
            r[(i, i)] = one();
        }
        r
    }
}



impl<T: Float> MatrixGe<T> {
    fn get_shape(&self) -> (usize, usize) {
        (self.row, self.row)
    }
}
