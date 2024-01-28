mod mat;
mod traits;

use std::{fmt::Display, ops::Mul};

pub use mat::*;

trait SparseImplTraits: From<u8> + Copy + Mul<Output = Self> + Display {}

impl SparseImplTraits for f64 {}
impl SparseImplTraits for f32 {}

impl SparseImplTraits for i128 {}
impl SparseImplTraits for i64 {}
impl SparseImplTraits for i32 {}
impl SparseImplTraits for i16 {}

impl SparseImplTraits for u128 {}
impl SparseImplTraits for u64 {}
impl SparseImplTraits for u32 {}
impl SparseImplTraits for u16 {}
impl SparseImplTraits for u8 {}
