use super::{Vector, Vectorizable};

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::fmt;

use std::convert::TryFrom;
use rand::{Rng, Rand, thread_rng};
use num::traits::*;

use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Error};

use super::typenum::Unsigned;
use super::typenum::marker_traits::NonZero;

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

use super::typenum::{IsGreater, U1, U2};

// convienience accessors methods for common vector usages
impl<T: Vectorizable, S: Unsigned> Vector<T, S> {
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn x(&self) -> T
    where T: NonZero {
        self.value[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn y(&self) -> T
    where T: IsGreater<U1> {
        self.value[1]
    }
    
    // extracts the third element of the vector, equivalent to vector[2]
    pub fn z(&self) -> T
    where T: IsGreater<U2> {
        self.value[2]
    }
    
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn r(&self) -> T
    where T: NonZero {
        self.value[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn theta(&self) -> T
    where T: IsGreater<U1> {
        self.value[1]
    }
}

impl<T: Vectorizable, S: Unsigned> Vector<T, S> {
    /// creates a vector of 0.0s
    pub fn new() -> Self
    where T: Zero {
        Vector {
            value: vec![T::zero(); S::to_usize()],
            phantom: PhantomData
        }
    }

    /// get the dimension of the vector
    pub fn dim(&self) -> usize {
        S::to_usize()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(&self) -> Vector<U, S>
        where U: Vectorizable + From<T> {
        self.map(|&x| U::from(x.clone()))
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: Vectorizable, F>(&self, f: F) -> Vector<U, S>
        where F: Fn(&T) -> U {
        Vector::make(self.value.iter().map(f).collect::<Vec<U>>())
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Mul<Output = T>,
              S: NonZero {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U: Vectorizable, O>(&self, other: &Vector<U, S>) -> O
    where O: Vectorizable + Zero,
          T: Vectorizable + Mul<U, Output = O>,
          S: NonZero {
        let mut sum = O::zero();
        for i in 0..S::to_usize() {
            sum = sum + self[i] * other[i];
        }
        sum
    }

    /// creates a random unit vector
    pub fn rand() -> Self
    where T: Rand + Float, S: NonZero {
        let mut vec = Vec::new();
        let mut rng = thread_rng();
        let one = T::one();
        let two = one + one;
        
        for _ in 0..S::to_usize() {
            let v: T = rng.gen();
            vec.push(v * two - one);
        }

        Self::try_from(vec).unwrap().norm()
    }
}

impl<T: Vectorizable, S: Unsigned> Vector<T, S> 
    where T: Float, S: NonZero {
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

impl<T: Vectorizable, S: Unsigned> Vector<T, S> 
where T: Add<Output = T> {
    /// adds the shift value to all the elements in a vector
    pub fn shift(&self, value: T) -> Self {
        let mut vec = Vec::new();

        for i in self.value.iter() {
            vec.push(*i + value);
        }

        Self::try_from(vec).unwrap()
    }

    /// sums up the elements of the vector
    pub fn sum(&self) -> T
    where T: Zero {
        let mut sum = T::zero();
        for i in self.value.iter() {
            sum = sum + *i;
        }
        sum
    }
}

impl<T: Vectorizable, S: Unsigned> Vector<T, S> 
    where T: One + Add<Output = T> + Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T, S>, w: T) -> Self {
        self * (T::one() - w) + other * w
    }
}

// traits
impl<'a, T: Vectorizable, S: Unsigned> TryFrom<&'a [T]> for Vector<T, S> {
    type Error = String;

    // get a vector from a slice
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        Self::try_from(Vec::from(value))
    }
}

impl<T: Vectorizable, S: Unsigned> TryFrom<Vec<T>> for Vector<T, S> {
    type Error = String;

    // get a vector from a vec
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() == S::to_usize() {
            Ok(Self { value, phantom: PhantomData })
        } else {
            Err(format!("the vector's size {}, does not match the vector size {}", value.len(), S::to_usize()))
        }
    }
}

impl<T: Vectorizable, S: Unsigned> Index<usize> for Vector<T, S> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<T: Vectorizable, S: Unsigned> IndexMut<usize> for Vector<T, S> {

    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.value[index]
    }
}

impl<T: Vectorizable + Sized + fmt::Debug, S: Unsigned> fmt::Debug for Vector<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = format!("{:?}", self.value);
        write!(f, "<{}>", &ret[1..ret.len()-1])
    }
}

impl<T: Vectorizable + Sized + Serialize, S: Unsigned> Serialize for Vector<T, S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where Ser: Serializer {
        self.value.serialize(serializer)
    }
}

impl<'de, T: Vectorizable + Sized + Deserialize<'de>, S: Unsigned> Deserialize<'de> for Vector<T, S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let vec: Vec<T> = <Vec<T> as Deserialize<'de>>::deserialize(deserializer)?;

        if vec.len() == S::to_usize() {
            Ok(Self { value: vec, phantom: PhantomData })
        } else {
            Err(Error::invalid_length(vec.len(), &&*format!("expected {}", S::to_usize())))
        }
    }
}

impl<T: Vectorizable + Eq, S: Unsigned + PartialEq> Eq for Vector<T, S> { }

impl<T: Vectorizable + Hash, S: Unsigned> Hash for Vector<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.value.iter() {
            i.hash(state);
        }
    }
}