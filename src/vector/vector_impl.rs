use std::ops::{Add, Sub, Mul, Index, IndexMut};
use rand::{Rng, Rand, thread_rng};
use num::traits::*;

use super::Vector;

#[macro_export]
macro_rules! vector {
    [$($e: expr),*] => {
        Vector::from(vec![$($e),*])
    };
}

// related functions
impl<T> Vector<T>
    where T: Clone + Copy {
    /// creates a vector of 0.0s with the dimension given
    pub fn new(dim: usize) -> Self
    where T: Zero {
        Self::from(vec![T::zero(); dim])
    }

    /// creates a random unit vector with the dimension given
    pub fn rand(dim: usize) -> Self
    where T: Rand + Float {
        let mut vec = Vec::new();
        let mut rng = thread_rng();
        let one = T::one();
        let two = one + one;
        
        for _ in 0..dim {
            let v: T = rng.gen();
            vec.push(v * two - one);
        }

        Self::from(vec).norm()
    }
}

impl<T> Vector<T> 
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

impl<T> Vector<T> 
where T: Clone + Copy + Add<Output = T> {
    /// adds the shift value to all the elements in a vector
    pub fn shift(&self, value: T) -> Self {
        let mut vec = Vec::new();

        for i in self.value.iter() {
            vec.push(*i + value);
        }

        Self::from(vec)
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

impl<T> Vector<T> 
where T: Clone + Copy {
    /// get the dimension of the vector
    pub fn dim(&self) -> usize {
        self.value.len()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(&self) -> Vector<U>
        where U: Clone + Copy + From<T> {
        self.convert(|&x| U::from(x.clone()))
    }

    /// conversion functions between different vector types using a provided function
    pub fn convert<U, F>(&self, f: F) -> Vector<U>
        where U: Clone + Copy,
              F: Fn(&T) -> U {
        Vector::from(self.value.iter().map(f).collect::<Vec<U>>())
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Mul<Output = T> {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U, O>(&self, other: &Vector<U>) -> O 
    where O: Clone + Copy + Zero,
          U: Clone + Copy,
          T: Clone + Copy + Mul<U, Output = O> {
        self.value.iter()
                  .zip(other.value.iter())
                  .fold(O::zero(), |sum, (&t, &u)| sum + t * u)
    }
}

impl<T> Vector<T> 
    where T: Clone + Copy + One + 
             Add<Output = T> +
             Sub<Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T>, w: T) -> Result<Self, String> {
        self * &(T::one() - w) + other * &w
    }
}

// traits
impl<'a, T> From<&'a [T]> for Vector<T>
    where T: Clone + Copy {
    
    // get a vector from a slice
    fn from(value: &'a [T]) -> Self {
        Self::from(Vec::from(value))
    }
}

impl<T> From<Vec<T>> for Vector<T>
    where T: Clone + Copy {
    
    // get a vector from a vec
    fn from(value: Vec<T>) -> Self {
        Self { value }
    }
}

impl<T> Index<usize> for Vector<T>
    where T: Clone + Copy {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<T> IndexMut<usize> for Vector<T>
    where T: Clone + Copy {

    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.value[index]
    }

}
