pub extern crate typenum;
pub extern crate generic_array;

#[macro_use]
pub mod vector_impl;
pub mod vector_impl_spec;
pub mod vector_ops;
pub mod vector_ops_spec;
pub mod iter;

#[cfg(test)]
mod tests;

use self::generic_array::{GenericArray, ArrayLength};

use super::InVector;

// Generic array will be changed to [T; N] when const generic numerics comes to nightly
#[derive(Clone, PartialEq)]
pub struct Vector<T: InVector, N: ArrayLength<T>>(pub GenericArray<T, N>);

#[derive(Debug)]
pub struct TryFromVectorError;

