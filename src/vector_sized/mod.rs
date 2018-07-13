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

use super::InVector;

pub trait UpdateWith<T> { fn update_with(&mut self, t: T); }

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
