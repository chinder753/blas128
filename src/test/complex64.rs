#[test]
fn complex64_test() {
    use crate::{
        num::complex::{Complex, ComplexMath},
        vector::{Vector, VectorBasic},
    };
    println!("\ncomplex computation");
    let a = Complex { re: 3.0, im: 5.0 };
    let b = Complex { re: 1.0, im: 2.0 };
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
