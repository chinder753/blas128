mod ops;

use wasm_bindgen::prelude::wasm_bindgen;

use super::float::Float;

macro_rules! complex {
    ($complex_name:ident, $t:ident) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Copy)]
        pub(crate) struct $complex_name {
            pub rel: $t,
            pub img: $t
        }
    };
}

macro_rules! complex_impl {
    ($complex_name:ident, $t:ident) => {
        impl $complex_name {
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
            pub fn nrm2(self) -> $t {
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
    };
}

complex!(Complex32, f32);
complex_impl!(Complex32, f32);

complex!(Complex64, f64);
complex_impl!(Complex64, f64);
