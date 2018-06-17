use super::{Vector, Vectorizable};

use std::marker::PhantomData;
use vector_sized::typenum::{U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10};

macro_rules! vector_create {
    ($size:ident $(,$var_name: ident)*) => {
        impl<T: Vectorizable> Vector<T, $size> {
            pub fn create($($var_name: T),*) -> Self {
                Self { value: vec![$($var_name),*], phantom: PhantomData }
            }
        }
    };
}

vector_create!(U0);
vector_create!(U1, x);
vector_create!(U2, x, y);
vector_create!(U3, x, y, z);
vector_create!(U4, w, x, y, z);
vector_create!(U5, x0, x1, x2, x3, x4);
vector_create!(U6, x0, x1, x2, x3, x4, x5);
vector_create!(U7, x0, x1, x2, x3, x4, x5, x6);
vector_create!(U8, x0, x1, x2, x3, x4, x5, x6, x7);
vector_create!(U9, x0, x1, x2, x3, x4, x5, x6, x7, x8);
vector_create!(U10, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9);