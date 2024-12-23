#![feature(vec_into_raw_parts)]
#![feature(stdarch_x86_avx512)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
pub mod scalar;
#[cfg(target_arch = "x86_64")]
pub mod simd;
