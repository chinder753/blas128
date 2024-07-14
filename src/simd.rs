#[macro_export]
macro_rules! v128_load_real_vector {
    ($t: ident, $vec: expr, $offset: expr) => {{
        use crate::num::zero::zero;
        use std::{mem::size_of, simd::Simd};
        const LANE: usize = 16 / size_of::<$t>();
        let mut value = [zero::<$t>(); LANE];
        for i in 0..LANE {
            value[i] = $vec[$offset + i];
        }
        Simd::from_array(value)
    }};
}

#[macro_export]
macro_rules! v128_store_real_vector {
    ($t: ident, $v128: expr, $vec: expr, $offset: expr) => {{
        use std::{mem::size_of, simd::Simd};
        const LANE: usize = 16 / size_of::<$t>();
        for i in 0..LANE {
            $vec[$offset + i] = $v128[i];
        }
    }};
}

#[macro_export]
macro_rules! v128_load_complex_vector {
    ($t: ident, $vec: expr, $offset: expr) => {{
        use crate::num::zero::zero;
        use std::{mem::size_of, simd::Simd};
        const LANE: usize = 16 / size_of::<$t>();
        let mut value = [zero::<$t>(); LANE];
        for i in 0..(LANE / 2) {
            value[i * 2] = $vec[$offset + i].rel;
            value[i * 2 + 1] = $vec[$offset + i].img;
        }
        Simd::from_array(value)
    }};
}

#[macro_export]
macro_rules! v128_store_complex_vector {
    ($t: ident, $v128: expr, $vec: expr, $offset: expr) => {{
        use std::mem::size_of;
        const LANE: usize = 16 / size_of::<$t>();
        for i in 0..(LANE/2) {
            $vec[$offset + i].rel = $v128[i*2];
            $vec[$offset + i].img = $v128[i*2+1];
        }
    }};
}

#[macro_export]
macro_rules! v128_load_complex {
    ($t: ident, $complex: expr) => {{
        use crate::num::zero::zero;
        use std::{mem::size_of, simd::Simd};
        const LANE: usize = 16 / size_of::<$t>();
        let mut value = [zero::<$t>(); LANE];
        value[0] = $complex.rel;
        value[1] = $complex.img;
        Simd::from_array(value)
    }};
}
