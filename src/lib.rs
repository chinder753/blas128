#![feature(generic_const_exprs)]
#![feature(concat_idents, portable_simd)]

#[allow(unused)]
#[warn(unused_imports)]
mod blas;

#[allow(unused)]
#[warn(unused_imports)]
mod matrix;

#[allow(unused)]
#[warn(unused_imports)]
mod num;

#[allow(unused)]
#[warn(unused_imports)]
mod simd;

#[allow(unused)]
#[warn(unused_imports)]
mod vector;

#[cfg(test)]
mod test;
