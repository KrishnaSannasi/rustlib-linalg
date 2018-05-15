use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use rand::{Rng, Rand, thread_rng};

use super::Vector;

// related functions
impl<T> Vector<T>
    where T: Clone + Copy + From<u8> {
    /// creates a vector of 0.0s with the dimension given
    pub fn new(dim: usize) -> Self {
        Self::from(vec![T::from(0u8); dim])
    }
}

impl<T> Vector<T>
    where T: Clone + Copy + Rand + From<u8> + From<f64> + Into<f64> +
          Add<T, Output = T> + Mul<T, Output = T> +
          Sub<T, Output = T> + Div<T, Output = T> {
    /// creates a random unit vector with the dimension given
    pub fn rand(dim: usize) -> Self {
        let mut vec = Vec::new();
        let mut rng = thread_rng();
        
        for _ in 0..dim {
            vec.push(rng.gen());
        }

        Self::from(vec).norm()
    }
}

// immutable methods
impl<T> Vector<T>
    where T: Clone + Copy {
    /// get the dimension of the vector
    pub fn dim(&self) -> usize {
        self.value.len()
    }
}


impl<T> Vector<T> 
    where T: Clone + Copy + From<u8> + From<f64> + Into<f64> +
          Add<T, Output = T> + Mul<T, Output = T> +
          Sub<T, Output = T> + Div<T, Output = T> {
    /// the magnitude
    pub fn mag(&self) -> T {
        T::from(self.magsq().into().sqrt())
    }

    /// returns a unit vector with the same 
    /// direction and dimension as the parent vector
    pub fn norm(&self) -> Self {
        self / self.mag()
    }

    /// gives the angle between two vectors
    pub fn angle(&self, other: &Self) -> f64 {
        let y = self.dot(other).into();
        let x = (self.magsq() * other.magsq()).into().sqrt();

        (y / x).acos()
    }
}

impl<T> Vector<T> 
    where T: Clone + Copy + From<u8> + Add<T, Output = T> {
    /// adds the shift value to all the elements in a vector
    pub fn shift(&self, value: T) -> Self {
        let mut vec = Vec::new();

        for i in self.value.iter() {
            vec.push(*i + value);
        }

        Self::from(vec)
    }

    /// sums up the elements of the vector
    pub fn sum(&self) -> T {
        let mut sum = T::from(0u8);
        for i in self.value.iter() {
            sum = sum + *i;
        }
        sum
    }
}

impl<T> Vector<T> 
    where T: Clone + Copy + From<u8> +
          Add<T, Output = T> + Mul<T, Output = T> {
    /// the square of the magnitude
    pub fn magsq(&self) -> T {
        self.dot(self)
    }
    
    /// takes the dot product of the two vectors
    pub fn dot(&self, other: &Self) -> T {
        let mut dot = T::from(0u8);
        
        for (i, j) in self.value.iter().zip(other.value.iter()) {
            dot = dot + *i * *j;
        }

        dot
    }
}


impl<T> Vector<T> 
    where T: Clone + Copy + From<u8> + Add<T, Output = T> +
          Sub<T, Output = T> + Mul<T, Output = T> {
    /// linearly interpolates between two vectors
    pub fn lerp(&self, other: &Vector<T>, w: T) -> Result<Self, String> {
        self * (T::from(1u8) - w) + other * w
    }
}

// traits
impl<'a, T> From<&'a [T]> for Vector<T>
    where T: Clone + Copy {
    fn from(value: &'a [T]) -> Self {
        Self::from(Vec::from(value))
    }
}

impl<T> From<Vec<T>> for Vector<T>
    where T: Clone + Copy {
    fn from(value: Vec<T>) -> Self {
        Self {
            value
        }
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
