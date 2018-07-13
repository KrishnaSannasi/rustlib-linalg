pub mod vector_impl;
pub mod vector_ops;
pub mod iter;

#[cfg(test)]
mod tests;

use super::InVector;

#[derive(Clone, PartialEq)]
pub struct Vector<T>
    where T: InVector {
    value: Vec<T>
}
