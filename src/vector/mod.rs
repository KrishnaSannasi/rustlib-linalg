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
