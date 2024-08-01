#[test]
fn float_test() {
    use crate::vector::{Vector, VectorBasic};
    let a = Vector::new(vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    let b = Vector::fill(2.0, 5);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("a+b: {:?}", a.clone() + b.clone());
    println!("a-b: {:?}", a.clone() - b.clone());
    println!("a*b: {:?}", a.clone() * b.clone());
    println!("a/b: {:?}", a.clone() / b.clone());
}
