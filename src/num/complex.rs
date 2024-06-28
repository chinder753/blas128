use super::float::Float;

mod ops;
mod ops_simd;
mod simd;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Complex<T: Float> {
    pub rel: T,
    pub img: T,
}

impl<T: Float> Complex<T> {
    pub fn conj(self) -> Self {
        Self {
            rel: self.rel,
            img: -self.img,
        }
    }
    pub fn inv(self) -> Self {
        let nrm2 = self.nrm2();
        Self {
            rel: self.rel / nrm2,
            img: self.img / nrm2,
        }
    }
    pub fn nrm2(self) -> T {
        self.rel.powi(2) + self.img.powi(2)
    }
    pub fn max(self, other: Self) -> Self {
        let a_sum = self.rel + self.img;
        let b_sum = other.rel + other.img;
        if a_sum > b_sum {
            self
        } else {
            other
        }
    }
    pub fn min(self, other: Self) -> Self {
        let a_sum = self.rel + self.img;
        let b_sum = other.rel + other.img;
        if a_sum < b_sum {
            self
        } else {
            other
        }
    }
}
