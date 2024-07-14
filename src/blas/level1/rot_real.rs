use crate::wasm_bindgen;

use crate::num::{float, zero::zero};
use crate::vector::{VectorF32, VectorF64};

macro_rules! blas_leve1_rot_real_impl {
    ($vec_name:ident, $t:ident) => {
        #[wasm_bindgen]
        impl $vec_name {
            pub fn rot(x: &mut $vec_name, y: &mut $vec_name, c: $t, s: $t) {
                assert_eq!(x.get_len(), y.get_len());
                for i in 0..x.get_len() {
                    let xi = x[i];
                    let yi = y[i];
                    x[i] = c * xi + s * yi;
                    y[i] = c * xi - s * yi;
                }
            }
            pub fn rotg(a: $t, b: $t) -> Vec<$t> {
                let r = (a.powi(2) + b.powi(2)).sqrt();
                let c = a / r;
                let s = b / r;
                let mut z: $t = 0.0;
                if (a.abs() > b.abs()) {
                    z = s;
                } else if (c != 0.0) {
                    z = 1.0 / c;
                } else {
                    z = 1.0
                }
                vec![r, z, c, s]
            }
            pub fn rotm(x: &mut $vec_name, y: &mut $vec_name, param: Vec<$t>) {
                assert_eq!(param.len(), 5);
                assert_eq!(x.get_len(), y.get_len());
                match param[0] {
                    -1.0 => {
                        for i in 0..x.get_len() {
                            let xi = x[i];
                            let yi = y[i];
                            x[i] = param[1] * xi + param[2] * yi;
                            y[i] = param[3] * xi + param[4] * yi;
                        }
                    }
                    0.0 => {
                        for i in 0..x.get_len() {
                            let xi = x[i];
                            let yi = y[i];
                            x[i] = xi + param[2] * yi;
                            y[i] = param[3] * xi + yi;
                        }
                    }
                    1.0 => {
                        for i in 0..x.get_len() {
                            let xi = x[i];
                            let yi = y[i];
                            x[i] = param[1] * xi + yi;
                            y[i] = -xi + param[4] * yi;
                        }
                    }
                    -2.0 => return (),
                    _ => println!(
                        "param[0] must be one of -1.0, 0.0, 1.0
                                    , but param[0] is {:?}",
                        param[0]
                    ),
                }
            }
            // pub fn rotmg(x1: $t, y1: $t, d1: $t, d2: $t){
            //     let x1s = x1*d1.sqrt();
            //     let y1s = y1*d2.sqrt();
            //     let r = (x1s.powi(2)+y1s.powi(2)).sqrt();
            // }
        }
    };
}

blas_leve1_rot_real_impl!(VectorF32, f32);
blas_leve1_rot_real_impl!(VectorF64, f64);
