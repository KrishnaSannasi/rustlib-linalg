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

#[cfg(feature = "no_std")]
use core::iter::{Iterator, ExactSizeIterator};
#[cfg(not(feature = "no_std"))]
#[cfg(feature = "sized")]
use std::iter::{Iterator, ExactSizeIterator};

use super::InVector;

// Generic array will be changed to [T; N] when const generic numerics comes to nightly
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Vector<T: InVector, N: ArrayLength<T>>(pub GenericArray<T, N>);

impl<T, N> !InVector for Vector<T, N> {}

#[derive(Debug)]
pub struct TryFromVectorError;

pub struct RepeatN<T: Clone> {
    count: usize,
    value: T
}

pub struct RepeatNWith<T, F: FnMut() -> T> {
    count: usize,
    value: F
}

impl<T: Clone> Iterator for RepeatN<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.value.clone())
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) { (self.count, Some(self.count)) }
}

impl<T, F: FnMut() -> T> Iterator for RepeatNWith<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            Some((self.value)())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.count, Some(self.count)) }
}

impl<T: Clone> ExactSizeIterator for RepeatN<T> { }

impl<T, F: FnMut() -> T> ExactSizeIterator for RepeatNWith<T, F> { }