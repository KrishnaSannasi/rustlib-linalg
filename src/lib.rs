#![feature(try_from, specialization)]

extern crate rand;
extern crate num;
extern crate serde;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! vectorize {
        [$($e: expr),*] => {
            Vector::from(vec![$($e),*])
        };
        [$e: expr;$c: expr] => {
            Vector::from(vec![$e;$c])
        }
    }
}

pub mod vector;
#[cfg(feature = "sized")]
pub mod vector_sized;

/// Marker trait to for anything that can be put in a vector
pub trait InVector: Copy {}

pub trait UpdateWith<T> { fn update_with(&mut self, t: T); }

macro_rules! specialize {
    (gen => $sized_name_gen:ident, $name_gen:ident, $type:ty) => {
        impl InVector for $type {}
        #[cfg(feature = "sized")]
        pub type $sized_name_gen<S> = vector_sized::Vector<$type, S>;
        pub type $name_gen = vector::Vector<$type>;
    };
}

specialize!(gen => SVectorBool, VectorBool, bool);

specialize!(gen => SVectorF32, VectorF32, f32);
specialize!(gen => SVectorF64, VectorF64, f64);

specialize!(gen => SVectorU8, VectorU8, u8);
specialize!(gen => SVectorU16, VectorU16, u16);
specialize!(gen => SVectorU32, VectorU32, u32);
specialize!(gen => SVectorU64, VectorU64, u64);
specialize!(gen => SVectorU128, VectorU128, u128);
specialize!(gen => SVectorUSize, VectorUSize, usize);

specialize!(gen => SVectorI8, VectorI8, i8);
specialize!(gen => SVectorI16, VectorI16, i16);
specialize!(gen => SVectorI32, VectorI32, i32);
specialize!(gen => SVectorI64, VectorI64, i64);
specialize!(gen => SVectorI128, VectorI128, i128);
specialize!(gen => SVectorISize, VectorISize, isize);

impl<'a, T: InVector> InVector for &'a T {}

use num::complex::Complex;

impl<T: Copy> InVector for Complex<T> {  }

