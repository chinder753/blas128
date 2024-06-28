#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![feature(concat_idents)]

#[allow(unused)]
mod blas;
#[allow(unused)]
mod cint;
#[allow(unused)]
mod matrix;
#[allow(unused)]
mod num;
#[allow(unused)]
mod simd;
#[allow(unused)]
mod vector;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use crate::cint::CINTlen_cart;

// #[wasm_bindgen]
pub fn print_cint() -> i32 {
    CINTlen_cart(2)
}

#[cfg(test)]
mod tests {
    use crate::{cint::CINTlen_cart, num::complex::Complex, vector::Vector};

    #[test]
    fn float_test() {
        let a = Vector::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        let b = Vector::fill(2.0, 5);
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("a+b: {:?}", a.clone() + b.clone());
        println!("a-b: {:?}", a.clone() - b.clone());
        println!("a*b: {:?}", a.clone() * b.clone());
        println!("a/b: {:?}", a.clone() / b.clone());
    }

    #[test]
    fn complex64_test() {
        #[allow(unused_unsafe)]
        unsafe {
            println!("\ncomplex computation");
            let a = Complex { rel: 3.0, img: 5.0 };
            let b = Complex { rel: 1.0, img: 2.0 };
            println!("a: {:?}", a);
            println!("a.nrm2(): {:?}", a.nrm2());
            println!("b: {:?}", b);
            println!("b.nrm2(): {:?}", b.nrm2());
            println!("a.inv(): {:?}", a.inv());
            println!("a.conj: {:?}", a.conj());
            println!("a+b: {:?}", a + b);
            println!("a-b: {:?}", a - b);
            println!("a*b: {:?}", a * b);
            println!("a/b: {:?}", a / b);
            println!("a*b.conj: {:?}", a * b.conj());
            //
            println!("\ncomplex vector computation");
            let v1 = Vector::new(vec![a, a, a, b, b]);
            let v2 = Vector::new(vec![b, b, b, a, a]);
            println!("v1: {:?}", v1);
            println!("v2: {:?}", v2);
            println!("v1+v2: {:?}", v1.clone() + v2.clone());
            println!("v1-v2: {:?}", v1.clone() - v2.clone());
            println!("v1*v2: {:?}", v1.clone() * v2.clone());
            println!("v1/v2: {:?}", v1.clone() / v2.clone());
        }
    }

    #[test]
    fn complex32_test() {
        #[allow(unused_unsafe)]
        unsafe {
            println!("\ncomplex computation");
            let a: Complex<f32> = Complex { rel: 3.0, img: 5.0 };
            let b: Complex<f32> = Complex { rel: 1.0, img: 2.0 };
            println!("a: {:?}", a);
            println!("a.nrm2(): {:?}", a.nrm2());
            println!("b: {:?}", b);
            println!("b.nrm2(): {:?}", b.nrm2());
            println!("a.inv(): {:?}", a.inv());
            println!("a.conj: {:?}", a.conj());
            println!("a+b: {:?}", a + b);
            println!("a-b: {:?}", a - b);
            println!("a*b: {:?}", a * b);
            println!("a/b: {:?}", a / b);
            println!("a*b.conj: {:?}", a * b.conj());
            //
            println!("\ncomplex vector computation");
            let v1 = Vector::new(vec![a, a, a, b, b]);
            let v2 = Vector::new(vec![b, b, b, a, a]);
            println!("v1: {:?}", v1);
            println!("v2: {:?}", v2);
            println!("v1+v2: {:?}", v1.clone() + v2.clone());
            println!("v1-v2: {:?}", v1.clone() - v2.clone());
            println!("v1*v2: {:?}", v1.clone() * v2.clone());
            println!("v1/v2: {:?}", v1.clone() / v2.clone());
        }
    }

    #[test]
    fn cint_test() {
        println!("{:?}", CINTlen_cart(2));
    }
}
