use super::{Vector, InVector, TryFromVectorError};
use super::typenum::marker_traits::NonZero;

#[cfg(feature = "no_std")]
use core::{
    ops::{Add, Sub, Mul, Deref, DerefMut},
    convert::{TryFrom, Into},
    fmt, mem
};
#[cfg(not(feature = "no_std"))]
use std::{
    vec::Vec,
    ops::{Add, Sub, Mul, Deref, DerefMut},
    convert::{TryFrom, Into},
    fmt, mem
};

use rand::{Rng, Rand, thread_rng};
use num::traits::*;

use super::typenum::{Diff, U1, U2};
use super::generic_array::{GenericArray, ArrayLength};
use serde::{Deserialize, Deserializer};

#[macro_export]
macro_rules! vector {
    [$($e: expr),*] => ( $crate::Vector($crate::vector_sized::GenericArray::from([$($e),*])) );
    [$e:expr; $c:expr] => {{
        Vector(GenericArray::from_exact_iter(
            $crate::vector_sized::RepeatN { value: $e, count: $c }
        ).unwrap())
    }};
    [use $e:expr; $c:expr] => {{
        Vector(GenericArray::from_exact_iter(
            $crate::vector_sized::RepeatNWith { value: $e, count: $c }
        ).unwrap())
    }};
}

// convienience accessors methods for common vector usages
impl<T: InVector, N: ArrayLength<T>> Vector<T, N> {
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn x(&self) -> &T
    where N: NonZero {
        &self[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn y(&self) -> &T
    where N: Sub<U1>,
          Diff<N, U1>: NonZero {
        &self[1]
    }
    
    // extracts the third element of the vector, equivalent to vector[2]
    pub fn z(&self) -> &T
    where N: Sub<U2>,
          Diff<N, U2>: NonZero {
        &self[2]
    }
    
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn r(&self) -> &T
    where N: NonZero {
        &self[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn theta(&self) -> &T
    where N: Sub<U1>,
          Diff<N, U1>: NonZero {
        &self[1]
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> {
    /// creates a vector of 0.0s
    pub fn new() -> Self
    where T: Zero {
        use self::mem::{uninitialized, swap, forget};
        let mut vec = vector![use || unsafe { uninitialized() }; N::to_usize()];

        for i in 0..N::to_usize() {
            let mut var = T::zero();
            swap(&mut var, &mut vec[i]);
            forget(var); // so that uninitialized mem does not drop
        }

        vec
    }

    /// get the dimension (length) of the vector
    pub fn dim(&self) -> usize {
        N::to_usize()
    }
    
    /// gets value at index, and clones it. This is unnecessary if `T` is `Copy`.
    pub fn get(&self, index: usize) -> T
    where T: Clone {
        self[index].clone()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(self) -> Vector<U, N>
        where U: InVector,
              T: Into<U>,
              N: ArrayLength<U>  {
        self.map(|x| x.into())
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: InVector, F>(self, f: F) -> Vector<U, N>
        where F: Fn(T) -> U,
              N: ArrayLength<U> {
        Vector(self.into_iter().map(f).collect())
    }

    /// maps the vector's component's according to the function provided
    pub fn map_ref<U: InVector, F>(&self, f: F) -> Vector<U, N>
        where F: Fn(&T) -> U,
              N: ArrayLength<U> {
        Vector(self.iter().map(f).collect())
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Clone + Mul<Output = T>,
              N: NonZero {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U, O>(&self, other: &Vector<U, N>) -> O
    where U: InVector + Clone,
          O: InVector + Zero,
          T: InVector + Clone + Mul<U, Output = O>,
          N: ArrayLength<U> + ArrayLength<O> {
        (self * other).sum()
    }

    /// creates a random unit vector
    pub fn rand() -> Self
    where T: Rand + Float, N: NonZero {
        let mut vec = Self::new();
        let mut rng = thread_rng();
        let one = T::one();
        let two = one + one;
        
        for i in 0..N::to_usize() {
            let v: T = rng.gen();
            vec[i] = v * two - one;
        }

        vec.norm()
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> 
    where T: Float, N: NonZero {
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

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> 
where T: Add<Output = T> {
    /// adds the shift value to all the elements in a vector
    pub fn shift(mut self, value: T) -> Self
    where T: Clone {
        use self::mem::{uninitialized, swap, forget};
        self.iter_mut().for_each(|i| {
            let mut out = unsafe { uninitialized() };
            swap(i, &mut out);
            
            let mut sum = out + value.clone();

            swap(i, &mut sum);
            forget(sum);
        });
        self
    }

    /// sums up the elements of the vector
    pub fn sum(self) -> T
    where T: Zero {
        self.into_iter().fold(T::zero(), |acc, x| acc + x)
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> 
where T: Mul<Output = T> + One {
    /// multiplies up the elements of the vector
    pub fn product(self) -> T {
        self.into_iter().fold(T::one(), |acc, x| acc * x)
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N>  {
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

impl<T: InVector + Clone, N: ArrayLength<T>> Vector<T, N> 
    where T: One + Add<Output = T> + Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T, N>, w: T) -> Self {
        self * (T::one() - w.clone()) + other * w
    }
}

// traits
impl<T: InVector, N: ArrayLength<T>> From<GenericArray<T, N>> for Vector<T, N> {
    fn from(value: GenericArray<T, N>) -> Self {
        Vector(value)
    }
}

impl fmt::Display for TryFromVectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Length")
    }
}

impl<'a, T: InVector + Clone, N: ArrayLength<T>> TryFrom<&'a [T]> for Vector<T, N> {
    type Error = TryFromVectorError;

    // get a vector from a slice
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        if value.len() == N::to_usize() {
            Ok(Vector(GenericArray::clone_from_slice(value)))
        } else {
            Err(TryFromVectorError)
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl<T: InVector + Clone, N: ArrayLength<T>> TryFrom<Vec<T>> for Vector<T, N> {
    type Error = TryFromVectorError;

    // get a vector from a vec
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from(&value as &[T])
    }
}

impl<T: InVector, N: ArrayLength<T>> Deref for Vector<T, N> {
    type Target = GenericArray<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: InVector, N: ArrayLength<T>> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de, T: InVector + Default + Deserialize<'de>, N: ArrayLength<T>> Deserialize<'de> for Vector<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let arr: GenericArray<T, N> = <GenericArray<T, N> as Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Vector(arr))
    }
}
