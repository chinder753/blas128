use std::ops::{Index, IndexMut};
use crate::vector::vecele::VectorElement;
use super::MatrixGe;

impl<T: VectorElement> Index<[usize; 2]> for MatrixGe<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &(self.value[index[0] * self.col + index[1]])
    }
}

impl<T: VectorElement> IndexMut<[usize; 2]> for MatrixGe<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut (self.value[index[0] * self.col + index[1]])
    }
}

impl<T: VectorElement> Index<usize> for MatrixGe<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index * self.col..(index + 1) * self.col]
    }
}

impl<T: VectorElement> IndexMut<usize> for MatrixGe<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index * self.col..(index + 1) * self.col]
    }
}
