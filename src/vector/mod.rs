pub mod vector_impl;
pub mod vector_impl_spec;
pub mod vector_ops;
pub mod iter;

#[cfg(test)]
mod tests;

use super::InVector;
use std::prelude::v1::*;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Vector<T: InVector>(pub Vec<T>);

impl<T> !InVector for Vector<T> {}
