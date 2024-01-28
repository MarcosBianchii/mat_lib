mod mat;
mod traits;

use std::fmt::Display;
use std::ops::{Add, Div, Mul};

pub use mat::*;

trait DiagImplTraits:
    Add<Output = Self> + Copy + Mul<Output = Self> + PartialEq + Div<Output = Self> + Display + From<u8>
{
}

impl DiagImplTraits for f64 {}
impl DiagImplTraits for f32 {}

impl DiagImplTraits for i128 {}
impl DiagImplTraits for i64 {}
impl DiagImplTraits for i32 {}
impl DiagImplTraits for i16 {}

impl DiagImplTraits for u128 {}
impl DiagImplTraits for u64 {}
impl DiagImplTraits for u32 {}
impl DiagImplTraits for u16 {}
impl DiagImplTraits for u8 {}
