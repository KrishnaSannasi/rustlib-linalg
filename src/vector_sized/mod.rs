pub extern crate typenum;

use self::typenum::Unsigned;
use std::marker::PhantomData;
#[macro_use]
pub mod vector_impl;
pub mod vector_impl_spec;
pub mod vector_ops;
pub mod vector_ops_spec;
pub mod iter;

#[cfg(test)]
mod tests;

pub trait UpdateWith<T> { fn update_with(&mut self, t: T); }

/// Marker trait to for anything that can be put in a vector
pub trait InVector: Copy {}

#[derive(Clone, PartialEq)]
pub struct Vector<T, N>
    where T: InVector,
          N: Unsigned {
    // Vec will be changed to [T; N] when const generic numerics comes to nightly
    value: Vec<T>,
    // *const S, so drop checker knows that Vector does not own a value of S
    phantom: PhantomData<*const N>
}

#[derive(Debug)]
pub struct TryFromVectorError(String);

impl <T: InVector, N: Unsigned> Vector<T, N> {
    // for internal use where the size is gaurenteed to be correct
    fn make(value: Vec<T>) -> Self {
        Self { value, phantom: PhantomData }
    }
}

macro_rules! specialize {
    (gen => $name_gen:ident, $type:ty) => {
        impl InVector for $type {}
        pub type $name_gen<S> = Vector<$type, S>;
    };
}

specialize!(gen => VectorBool, bool);

specialize!(gen => VectorF32, f32);
specialize!(gen => VectorF64, f64);

specialize!(gen => VectorU8, u8);
specialize!(gen => VectorU16, u16);
specialize!(gen => VectorU32, u32);
specialize!(gen => VectorU64, u64);
specialize!(gen => VectorU128, u128);
specialize!(gen => VectorUSize, usize);

specialize!(gen => VectorI8, i8);
specialize!(gen => VectorI16, i16);
specialize!(gen => VectorI32, i32);
specialize!(gen => VectorI64, i64);
specialize!(gen => VectorI128, i128);
specialize!(gen => VectorISize, isize);

impl<'a, T: InVector> InVector for &'a T {}

use num::complex::Complex;

impl<T: Copy> InVector for Complex<T> {  }