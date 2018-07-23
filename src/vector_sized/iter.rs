use super::{Vector, InVector};
use super::generic_array::ArrayLength;

#[cfg(feature = "no_std")]
use core::iter::Iterator;
#[cfg(not(feature = "no_std"))]
use std::iter::Iterator;
use super::generic_array::GenericArrayIter;

impl<T: InVector, N: ArrayLength<T>> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = GenericArrayIter<T, N>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}

