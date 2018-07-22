use super::{Vector, InVector, TryFromVectorError};
use super::typenum::marker_traits::NonZero;

use std::ops::{Add, Sub, Mul, Deref, DerefMut};
use std::convert::{TryFrom, Into};
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use std::error;
use std::fmt;

use rand::{Rng, Rand, thread_rng};
use num::traits::*;

use super::generic_array::{GenericArray, ArrayLength};

use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error};

#[macro_export]
macro_rules! vector {
    [$vec:expr] => {
        {
            use std::convert::TryFrom;
            Vector::try_from($vec).unwrap()
        }
    };
    [$($e: expr),*] => {
        vector![vec![$($e),*]]
    };
    [$e: expr;$c: expr] => {
        vector![vec![$e;$c]]
    }
}

use super::typenum::{IsGreater, Diff, U1, U2};

// convienience accessors methods for common vector usages
impl<T: InVector, N: ArrayLength<T>> Vector<T, N> {
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn x(&self) -> T
    where N: NonZero {
        self[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn y(&self) -> T
    where N: Sub<U1>,
          Diff<N, U1>: NonZero {
        self[1]
    }
    
    // extracts the third element of the vector, equivalent to vector[2]
    pub fn z(&self) -> T
    where N: Sub<U2>,
          Diff<N, U2>: NonZero {
        self[2]
    }
    
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn r(&self) -> T
    where N: NonZero {
        self[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn theta(&self) -> T
    where N: IsGreater<U1> {
        self[1]
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> {
    /// creates a vector of 0.0s
    pub fn new() -> Self
    where T: Zero {
        Vector(GenericArray::clone_from_slice(&vec![T::zero(); N::to_usize()]))
    }

    /// get the dimension of the vector
    pub fn dim(&self) -> usize {
        N::to_usize()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(&self) -> Vector<U, N>
        where U: InVector,
              T: Into<U>,
              N: ArrayLength<U>  {
        self.map(|&x| x.clone().into())
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: InVector, F>(&self, f: F) -> Vector<U, N>
        where F: Fn(&T) -> U,
              N: ArrayLength<U> {
        Vector(self.iter().map(f).collect())
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Mul<Output = T>,
              N: NonZero {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U, O>(&self, other: &Vector<U, N>) -> O
    where U: InVector,
          O: InVector + Zero,
          T: InVector + Mul<U, Output = O>,
          N: ArrayLength<U> + ArrayLength<O> {
        (self * other).sum()
    }

    /// creates a random unit vector
    pub fn rand() -> Self
    where T: Rand + Float, N: NonZero {
        let mut vec = Vec::new();
        let mut rng = thread_rng();
        let one = T::one();
        let two = one + one;
        
        for _ in 0..N::to_usize() {
            let v: T = rng.gen();
            vec.push(v * two - one);
        }

        Self::try_from(vec).unwrap().norm()
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
    pub fn shift(&self, value: T) -> Self {
        let mut vec = Vec::new();

        for i in self.iter() {
            vec.push(*i + value);
        }

        Self::try_from(vec).unwrap()
    }

    /// sums up the elements of the vector
    pub fn sum(&self) -> T
    where T: Zero {
        self.iter().fold(T::zero(), |acc, &x| acc + x)
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> 
where T: Mul<Output = T> + One {
    /// sums up the elements of the vector
    pub fn product(&self) -> T {
        self.iter().fold(T::one(), |acc, &x| acc * x)
    }
}

impl<T: InVector, N: ArrayLength<T>> Vector<T, N> 
    where T: One + Add<Output = T> + Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T, N>, w: T) -> Self {
        self * (T::one() - w) + other * w
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
        write!(f, "{:?}", self)
    }
}

impl error::Error for TryFromVectorError {}

impl<'a, T: InVector, N: ArrayLength<T>> TryFrom<&'a [T]> for Vector<T, N> {
    type Error = TryFromVectorError;

    // get a vector from a slice
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        if value.len() == N::to_usize() {
            Ok(Vector(GenericArray::clone_from_slice(value)))
        } else {
            Err(TryFromVectorError(format!("the input's size {}, does not match the type size {}", value.len(), N::to_usize())))
        }
    }
}

impl<T: InVector, N: ArrayLength<T>> TryFrom<Vec<T>> for Vector<T, N> {
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

impl<T: InVector + Sized + fmt::Debug, N: ArrayLength<T>> fmt::Debug for Vector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = format!("{:?}", self.0);
        write!(f, "<{}>", &ret[1..ret.len()-1])
    }
}

impl<T: InVector + Sized + Serialize, N: ArrayLength<T>> Serialize for Vector<T, N> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where Ser: Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de, T: InVector + Sized + Deserialize<'de>, N: ArrayLength<T>> Deserialize<'de> for Vector<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let vec: Vec<T> = <Vec<T> as Deserialize<'de>>::deserialize(deserializer)?;
        let error = Error::invalid_length(vec.len(), &&*format!("expected {}", N::to_usize()));

        if vec.len() == N::to_usize() {
            Self::try_from(vec).map_err(|_| error)
        } else {
            Err(error)
        }
    }
}

impl<T: InVector + Eq, N: ArrayLength<T> + PartialEq> Eq for Vector<T, N> { }

impl<T: InVector + Hash, N: ArrayLength<T>> Hash for Vector<T, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.iter() {
            i.hash(state);
        }
    }
}