use crate::vector::{vec_ele::VectorElement, Vector, VectorBasic};

mod complex;
mod real;
mod rot_complex;
mod rot_real;

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
    pub fn iamax(self) -> usize {
        let mut max: T = self[0];
        match self
            .iter()
            .enumerate()
            .max_by(|(i, x), (j, y)| match x.partial_cmp(y) {
                Some(cmp_res) => cmp_res,
                None => panic!(""),
            }) {
            Some((index, max)) => index,
            None => panic!(""),
        }
    }
    pub fn iamin(self) -> usize {
        let mut max: T = self[0];
        match self
            .iter()
            .enumerate()
            .min_by(|(i, x), (j, y)| match x.partial_cmp(y) {
                Some(cmp_res) => cmp_res,
                None => panic!(""),
            }) {
            Some((index, max)) => index,
            None => panic!(""),
        }
    }
}
