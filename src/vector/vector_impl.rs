use super::{Vector, InVector};

use std::{
    vec::Vec,
    ops::{Add, Sub, Mul, Deref, DerefMut},
    convert::Into, mem
};

use rand::{Rng, Rand, thread_rng};
use num::traits::*;

#[macro_export]
macro_rules! vectorize {
    [$($e: expr),*] => ( Vector(vector![$($e)*]) );
    [$e:expr; $c:expr] => {{
        Vector(vector![$e; $c])
    }};
    [use $e:expr; $c:expr] => {{
        Vector(::std::iter::repeat_with($e).take($c).collect())
    }};
}

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
        use self::mem::{uninitialized, swap, forget};
        let mut vec = vectorize![use || unsafe { uninitialized() }; dim];

        for i in vec.iter_mut() {
            let mut var = T::zero();
            swap(&mut var, i);
            forget(var); // so that uninitialized mem does not drop
        }

        vec
    }

    /// get the dimension (length) of the vector
    pub fn dim(&self) -> usize {
        self.0.len()
    }
    
    /// gets value at index, and clones it. This is unnecessary if `T` is `Copy`.
    pub fn get(&self, index: usize) -> T
    where T: Clone {
        self[index].clone()
    }

    /// conversion functions between different vector types (if the type implements from)
    pub fn into<U>(self) -> Vector<U>
        where U: InVector,
              T: Into<U> {
        self.map(|x| x.into())
    }

    /// maps the vector's component's according to the function provided
    pub fn map<U: InVector, F>(self, f: F) -> Vector<U>
        where F: Fn(T) -> U {
        Vector(self.into_iter().map(f).collect())
    }

    /// maps the vector's component's according to the function provided
    pub fn map_ref<U: InVector, F>(&self, f: F) -> Vector<U>
        where F: Fn(&T) -> U {
        Vector(self.iter().map(f).collect())
    }

    /// the square of the magnitude
    pub fn magsq(&self) -> T
        where T: Zero + Clone + Mul<Output = T> {
        self.dot(self)
    }

    /// takes the dot product of the two vectors
    pub fn dot<U, O>(&self, other: &Vector<U>) -> O
    where U: InVector + Clone,
          O: InVector + Zero,
          T: InVector + Clone + Mul<U, Output = O> {
        (self * other).sum()
    }

    /// creates a random unit vector
    pub fn rand(dim: usize) -> Self
    where T: Rand + Float {
        let mut vec = Self::new(dim);
        let mut rng = thread_rng();
        let one = T::one();
        let two = one + one;
        
        for i in vec.iter_mut() {
            let v: T = rng.gen();
            *i = v * two - one;
        }

        vec.norm()
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
    pub fn shift(mut self, value: &T) -> Self
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

impl<T: InVector> Vector<T> 
where T: Mul<Output = T> + One {
    /// multiplies up the elements of the vector
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
    // get a vector from a slice
    fn from(value: &'a [T]) -> Self {
        Vector(value.into())
    }
}

impl<T: InVector> From<Vec<T>> for Vector<T> {
    // get a vector from a vec
    fn from(value: Vec<T>) -> Self {
        Vector(value)
    }
}

impl<T: InVector> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: InVector> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}