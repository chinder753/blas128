mod complex;
mod real;
mod rot_complex;
mod rot_real;

use crate::num::complex::Complex;
use crate::num::float;
use crate::num::zero::zero;
use crate::vector::{vecele::VectorElement, Vector};

impl<T: VectorElement> Vector<T> {
    pub fn axpy(&mut self, a: T, y: Self) {
        assert_eq!(self.get_len(), y.get_len());
        for i in 0..self.get_len() {
            self[i] = self[i] * a + y[i];
        }
    }
    pub fn scal(&mut self, rhs: T) {
        for i in 0..self.get_len() {
            self[i] *= rhs;
        }
    }
    pub fn swap(x: &mut Self, y: &mut Self) -> () {
        assert_eq!(x.get_len(), y.get_len());
        for i in 0..x.get_len() {
            let temp_v = x[i];
            x[i] = y[i];
            y[i] = temp_v;
        }
    }
    pub fn iamax(self) -> T {
        let mut max: T = self[0];
        for i in 1..self.get_len() {
            max = max.max(self[i]);
        }
        max
    }
    pub fn iamin(self) -> T {
        let mut min: T = self[0];
        for i in 1..self.get_len() {
            min = min.min(self[i]);
        }
        min
    }
}
