use super::{Vector, InVector};

use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use std::fmt;

use std::convert::Into;
use rand::{Rng, Rand, thread_rng};
use num::traits::*;
use std::result;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

pub type Result<T> = result::Result<T, String>;

// convienience accessors methods for common vector usages
impl<T: InVector> Vector<T> {
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn x(&self) -> T {
        self.value[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn y(&self) -> T {
        self.value[1]
    }
    
    // extracts the third element of the vector, equivalent to vector[2]
    pub fn z(&self) -> T {
        self.value[2]
    }
    
    // extracts the first element of the vector, equivalent to vector[0]
    pub fn r(&self) -> T {
        self.value[0]
    }

    // extracts the second element of the vector, equivalent to vector[1]
    pub fn theta(&self) -> T {
        self.value[1]
    }
}

impl<T: InVector> Vector<T> {
    /// creates a vector of 0.0s
    pub fn new(dim: usize) -> Self
    where T: Zero {
        Vector {
            value: vec![T::zero(); dim],
        }
    }

    /// get the dimension of the vector
    pub fn dim(&self) -> usize {
        self.value.len()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(&self) -> Vector<U>
        where U: InVector,
              T: Into<U>  {
        self.map(|&x| x.clone().into())
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: InVector, F>(&self, f: F) -> Vector<U>
        where F: Fn(&T) -> U {
        Vector { value: self.value.iter().map(f).collect::<Vec<U>>() }
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Mul<Output = T> {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U, O>(&self, other: &Vector<U>) -> O
    where U: InVector,
          O: InVector + Zero,
          T: InVector + Mul<U, Output = O> {
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

        Vector { value }.norm()
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
    pub fn shift(&self, value: T) -> Self {
        let mut vec = Vec::new();

        for i in self.value.iter() {
            vec.push(*i + value);
        }

        Self { value: vec }
    }

    /// sums up the elements of the vector
    pub fn sum(&self) -> T
    where T: Zero {
        self.value.iter().fold(T::zero(), |acc, &x| acc + x)
    }
}

impl<T: InVector> Vector<T> 
where T: Mul<Output = T> + One {
    /// sums up the elements of the vector
    pub fn product(&self) -> T {
        self.value.iter().fold(T::one(), |acc, &x| acc * x)
    }
}

impl<T: InVector> Vector<T> 
    where T: One + Add<Output = T> + Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T>, w: T) -> Self {
        self * (T::one() - w) + other * w
    }
}

// traits
impl<T: InVector> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<T: InVector> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.value[index]
    }
}

impl<T: InVector + Sized + fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = format!("{:?}", self.value);
        write!(f, "<{}>", &ret[1..ret.len()-1])
    }
}

impl<T: InVector + Sized + Serialize> Serialize for Vector<T> {
    fn serialize<Ser>(&self, serializer: Ser) -> result::Result<Ser::Ok, Ser::Error>
    where Ser: Serializer {
        self.value.serialize(serializer)
    }
}

impl<'de, T: InVector + Sized + Deserialize<'de>> Deserialize<'de> for Vector<T> {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where D: Deserializer<'de> {
        let vec: Vec<T> = <Vec<T> as Deserialize<'de>>::deserialize(deserializer)?;

        Ok(Self { value: vec })
    }
}

impl<T: InVector + Eq + PartialEq> Eq for Vector<T> { }

impl<T: InVector + Hash> Hash for Vector<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.value.iter() {
            i.hash(state);
        }
    }
}