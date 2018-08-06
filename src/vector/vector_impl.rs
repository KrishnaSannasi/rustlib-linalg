use super::{Vector, InVector};

use std::prelude::v1::*;
use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::hash::{Hash, Hasher};
use std::slice::SliceIndex;
use std::convert::Into;
use std::result;

use rand::{Rng, Rand, thread_rng};
use num::traits::*;

pub type Result<T> = result::Result<T, String>;

// convienience accessors methods for common vector usages
impl<T: InVector> Vector<T> {
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn x(&self) -> &T {
        &self[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn y(&self) -> &T {
        &self[1]
    }
    
    // extracts the third element of the vector, equivalent to vector[2]
    pub fn z(&self) -> &T {
        &self[2]
    }
    
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn r(&self) -> &T {
        &self[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn theta(&self) -> &T {
        &self[1]
    }
}

impl<T: InVector> Vector<T> {
    /// creates a vector of 0.0s
    pub fn new(dim: usize) -> Self
    where T: Zero {
        Vector((0..dim).map(|_| T::zero()).collect())
    }

    /// get the dimension of the vector
    pub fn dim(&self) -> usize {
        self.0.len()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(self) -> Vector<U>
        where U: InVector,
              T: Into<U>  {
        self.map(|x| x.into())
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: InVector, F>(self, f: F) -> Vector<U>
        where F: Fn(T) -> U {
        Vector(self.into_iter().map(f).collect::<Vec<U>>())
    }

    /// maps the vector's component's according to the function provided
    pub fn map_ref<U: InVector, F>(&self, f: F) -> Vector<U>
        where F: Fn(&T) -> U {
        Vector(self.iter().map(f).collect::<Vec<U>>())
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Clone + Zero + Mul<Output = T> {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U, O>(&self, other: &Vector<U>) -> O
    where T: InVector + Clone + Mul<U, Output = O>,
          U: InVector + Clone,
          O: InVector + Zero {
        (self * other).sum()
    }

    /// creates a random unit vector
    pub fn rand(dim: usize) -> Self
    where T: Rand + Float {
        let mut value = Vec::new();
        let mut rng = thread_rng();
        let one = T::one();
        let two = one + one;
        
        for _ in 0..dim {
            let v: T = rng.gen();
            value.push(v * two - one);
        }

        Vector(value).norm()
    }
}

impl<T: InVector> Vector<T> 
    where T: Float {
    /// the magnitude
    pub fn mag(&self) -> T {
        self.magsq().sqrt()
    }

    /// returns a unit vector with the same 
    /// direction and dimension as the parent vector
    pub fn norm(&self) -> Self {
        self / self.mag()
    }
    
    /// gives the angle between two vectors
    pub fn angle(&self, other: &Self) -> T {
        let y = self.dot(other);
        let x = (self.magsq() * other.magsq()).sqrt();

        (y / x).acos()
    }
}

impl<T: InVector> Vector<T> 
where T: Add<Output = T> {
    /// adds the shift value to all the elements in a vector
    pub fn shift(mut self, value: T) -> Self
    where T: Clone {
        self.iter_mut().for_each(|i| {
            let mut out = unsafe { ::std::mem::uninitialized() };
            ::std::mem::swap(i, &mut out);
            let mut sum = out + value.clone();
            ::std::mem::swap(i, &mut sum);
            ::std::mem::forget(sum);
        });
        self
    }

    /// sums up the elements of the vector
    pub fn sum(self) -> T
    where T: Zero {
        self.into_iter().fold(T::zero(), |acc, x| acc + x)
    }
}

impl<T: InVector> Vector<T> 
where T: Mul<Output = T> + One {
    /// sums up the elements of the vector
    pub fn product(self) -> T {
        self.into_iter().fold(T::one(), |acc, x| acc * x)
    }
}

impl<T: InVector> Vector<T>  {
    /// adds the shift value to all the elements in a vector
    pub fn shift_ref(&self, value: T) -> Self
    where T: Clone,
          for<'a> &'a T: Add<T, Output = T> {
        self.map_ref(|i| i + value.clone())
    }

    /// sums up the elements of the vector
    pub fn sum_ref(&self) -> T
    where T: Zero,
          for<'a> &'a T: Add<T, Output = T> {
        self.iter().fold(T::zero(), |acc, x| x + acc)
    }

    /// multiplies up the elements of the vector
    pub fn product_ref(&self) -> T
    where T: One,
          for<'a> &'a T: Mul<T, Output = T> {
        self.iter().fold(T::one(), |acc, x| x * acc)
    }
}

impl<T: InVector + Clone> Vector<T> 
    where T: One + Add<Output = T> + Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T>, w: T) -> Self {
        self * (T::one() - w.clone()) + other * w
    }
}

// traits
impl<'a, T: InVector + Clone> From<&'a [T]> for Vector<T> {
    fn from(slice: &'a [T]) -> Self {
        Vector(slice.into())
    }
}

impl<T: InVector> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Vector(value)
    }
}

impl<T: InVector, I: SliceIndex<[T]>> Index<I> for Vector<T> {
    type Output = <Vec<T> as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: InVector, I: SliceIndex<[T]>> IndexMut<I> for Vector<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: InVector + Hash> Hash for Vector<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.iter() {
            i.hash(state);
        }
    }
}