use super::{Vector, InVector};

use std::iter::{IntoIterator, Iterator, ExactSizeIterator};
use std::marker::PhantomData;
use std::vec;
use std::slice;

use vector_sized::typenum::*;

impl<T: InVector, N: Unsigned> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            index: 0,
            iterator: self.value.into_iter(),
            phantom: self.phantom
        }
    }
}

impl<T: InVector, N: Unsigned> Vector<T, N> {
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter {
            index: 0,
            iterator: self.value.iter(),
            phantom: PhantomData
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut {
            index: 0,
            iterator: self.value.iter_mut(),
            phantom: PhantomData
        }
    }
}

pub struct IntoIter<T, N>
where T: InVector,
      N: Unsigned {
    index: usize,
    iterator: vec::IntoIter<T>,
    phantom: PhantomData<*const N>
}

impl<T, N> Iterator for IntoIter<T, N>
where T: InVector,
      N: Unsigned {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iterator.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = N::to_usize() - self.index;
        (rest, Some(rest))
    }
}

impl<T, N> ExactSizeIterator for IntoIter<T, N>
where T: InVector,
      N: Unsigned {
    fn len(&self) -> usize {
        N::to_usize()
    }
}

pub struct Iter<'a, T: 'a, N>
where T: InVector,
      N: Unsigned {
    index: usize,
    iterator: slice::Iter<'a, T>,
    phantom: PhantomData<*const N>
}

impl<'a, T: 'a, N> Iterator for Iter<'a, T, N>
where T: InVector,
      N: Unsigned {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iterator.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = N::to_usize() - self.index;
        (rest, Some(rest))
    }
}

impl<'a, T: 'a, N> ExactSizeIterator for Iter<'a, T, N>
where T: InVector ,
      N: Unsigned {
    fn len(&self) -> usize {
        N::to_usize()
    }
}

pub struct IterMut<'a, T: 'a, N>
where T: InVector,
      N: Unsigned {
    index: usize,
    iterator: slice::IterMut<'a, T>,
    phantom: PhantomData<*const N>
}

impl<'a, T: 'a, N> Iterator for IterMut<'a, T, N>
where T: InVector,
      N: Unsigned {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.iterator.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rest = N::to_usize() - self.index;
        (rest, Some(rest))
    }
}

impl<'a, T: 'a, N> ExactSizeIterator for IterMut<'a, T, N>
where T: InVector,
      N: Unsigned {
    fn len(&self) -> usize {
        N::to_usize()
    }
}
