extern crate typenum;

use self::typenum::{Unsigned, U2, U3};
use std::marker::PhantomData;

pub mod vector_impl;
pub mod vector_ops;

#[cfg(test)]
mod tests;

#[derive(Clone, PartialEq)]
pub struct Vector<T, S>
    where T: Copy,
          S: Unsigned {
    value: Vec<T>,
    phantom: PhantomData<S>
}

pub type VectorD<S> = Vector<f64, S>;
pub type VectorF<S> = Vector<f32, S>;
pub type VectorI<S> = Vector<i32, S>;
pub type VectorU<S> = Vector<u32, S>;
pub type VectorS<S> = Vector<usize, S>;

pub type Vector2D = Vector<f64, U2>;
pub type Vector2F = Vector<f32, U2>;
pub type Vector2I = Vector<i32, U2>;
pub type Vector2U = Vector<u32, U2>;
pub type Vector2S = Vector<usize, U2>;

pub type Vector3D = Vector<f64, U3>;
pub type Vector3F = Vector<f32, U3>;
pub type Vector3I = Vector<i32, U3>;
pub type Vector3U = Vector<u32, U3>;
pub type Vector3S = Vector<usize, U3>;

