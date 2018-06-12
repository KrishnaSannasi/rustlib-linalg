use super::Vector;

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::cmp::Eq;
use std::fmt;

use std::convert::TryFrom;
use rand::{Rng, Rand, thread_rng};
use num::traits::*;

use super::typenum::Unsigned;

impl<T: Copy, S: Unsigned + PartialEq> Eq for Vector<T, S> where T: Eq { }

impl <T: Copy, S: Unsigned> Vector<T, S> {
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
        self.value.len()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(&self) -> Vector<U, S>
        where U: Copy + From<T> {
        self.map(|&x| U::from(x.clone()))
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: Copy, F>(&self, f: F) -> Vector<U, S>
        where F: Fn(&T) -> U {
        Vector::try_from(self.value.iter().map(f).collect::<Vec<U>>()).unwrap()
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Mul<Output = T> {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U: Copy, O>(&self, other: &Vector<U, S>) -> O 
    where O: Copy + Zero,
          T: Copy + Mul<U, Output = O> {
        self.value.iter()
                  .zip(other.value.iter())
                  .fold(O::zero(), |sum, (&t, &u)| sum + t * u)
    }

    /// creates a random unit vector
    pub fn rand() -> Self
    where T: Rand + Float {
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

impl<T: Copy, S: Unsigned> Vector<T, S> 
    where T: Float {
    /// the magnitude
    pub fn mag(&self) -> T {
        self.magsq().sqrt()
    }

    /// returns a unit vector with the same 
    /// direction and dimension as the parent vector
    pub fn norm(&self) -> Self {
        self / &self.mag()
    }
    
    /// gives the angle between two vectors
    pub fn angle(&self, other: &Self) -> T {
        let y = self.dot(other);
        let x = (self.magsq() * other.magsq()).sqrt();

        (y / x).acos()
    }
}

impl<T: Copy, S: Unsigned> Vector<T, S> 
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

impl<T: Copy, S: Unsigned> Vector<T, S> 
    where T: One + Add<Output = T> +
             Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T, S>, w: T) -> Self {
        self * &(T::one() - w) + other * &w
    }
}

// traits
impl<'a, T: Copy, S: Unsigned> TryFrom<&'a [T]> for Vector<T, S> {
    type Error = String;

    // get a vector from a slice
    fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
        Self::try_from(Vec::from(value))
    }
}

impl<T: Copy, S: Unsigned> TryFrom<Vec<T>> for Vector<T, S> {
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

impl<T: Copy, S: Unsigned> Index<usize> for Vector<T, S> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<T: Copy, S: Unsigned> IndexMut<usize> for Vector<T, S> {

    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.value[index]
    }
}

impl<T: Copy + fmt::Debug, S: Unsigned> fmt::Debug for Vector<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = format!("{:?}", self.value);
        write!(f, "<{}>", &ret[1..ret.len()-1])
    }
}
