use super::{Vector, InVector};

use std::iter::IntoIterator;
use std::vec;
use std::slice;

impl<T: InVector> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: InVector> Vector<T> {
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}