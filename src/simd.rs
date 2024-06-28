use std::simd::Simd;

pub const F64_LANE: usize = 16 / size_of::<f64>();
pub const F32_LANE: usize = 16 / size_of::<f32>();

pub type V128<T> = Simd<T, { 16 / size_of::<T>() }>;
