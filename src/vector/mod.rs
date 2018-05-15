pub mod vector_impl;
pub mod vector_ops;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>
    where T: Clone + Copy {
    value: Vec<T>
}

pub type VectorD = Vector<f64>;
pub type VectorF = Vector<f32>;
pub type VectorI = Vector<i32>;
pub type VectorU = Vector<u32>;
pub type VectorS = Vector<usize>;

impl<T> Vector<T>
    where T: Clone + Copy {
    pub fn into<U>(&self) -> Vector<U>
        where U: Clone + Copy + From<T> {
        self.convert(|x| U::from(x.clone()))
    }

    pub fn convert<U, F>(&self, f: F) -> Vector<U>
        where U: Clone + Copy,
              F: Fn(T) -> U {
        let val: Vec<U> = self.value.iter().map(|x| f(*x)).collect();

        Vector::from(val)
    }
}
