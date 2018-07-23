use super::Vector;
use super::generic_array::ArrayLength;

#[cfg(feature = "no_std")]
use core::ops::*;
#[cfg(not(feature = "no_std"))]
use std::ops::*;
use num::complex::Complex;

macro_rules! impl_spec {
    (block => $fun:ident, $output:ident $(,$other:tt)*) => {{ // using star because ? is unstable
        $output.iter_mut()$(.zip($other.iter()))*.for_each($fun);
        $output
    }};
    (block own own => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<N: ArrayLength<$Ty>> $Op<Vector<$Ty, N>> for Vector<$Ty, N> {
            fn $fun(mut self, other: Vector<$Ty, N>) -> Self::Output {
                let fun = |(s, &o): (&mut $Ty, &$Ty)| *s = (*s).$fun(o);
                impl_spec!(block => fun, self, other)
            }
        }
    };
    (block own borrow => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, N: ArrayLength<$Ty>> $Op<&'a Vector<$Ty, N>> for Vector<$Ty, N> {
            fn $fun(mut self, other: &'a Vector<$Ty, N>) -> Self::Output {
                let fun = |(s, &o): (&mut $Ty, &$Ty)| *s = (*s).$fun(o);
                impl_spec!(block => fun, self, other)
            }
        }
    };
    (block borrow own => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, N: ArrayLength<$Ty>> $Op<Vector<$Ty, N>> for &'a Vector<$Ty, N> {
            fn $fun(self, mut other: Vector<$Ty, N>) -> Self::Output {
                let fun = |(o, &s): (&mut $Ty, &$Ty)| *o = s.$fun(*o);
                impl_spec!(block => fun, other, self)
            }
        }
    };
    (block unary => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, N: ArrayLength<$Ty>> $Op for Vector<$Ty, N> {
            fn $fun(mut self) -> Self::Output {
                let fun = |o: &mut $Ty| *o = (*o).$fun();
                impl_spec!(block => fun, self)
            }
        }
    };
    (block all => $Op:ident, $fun:ident, $Ty:ty) => {
        impl_spec!(block own own => $Op, $fun, $Ty);
        impl_spec!(block own borrow => $Op, $fun, $Ty);
        impl_spec!(block borrow own => $Op, $fun, $Ty);
    };
    (block ty op => $Ty:ty) => {};
    (block ty op => $Ty:ty, neg $(,$rest:tt)*) => {
        impl_spec!(block ty op => $Ty $(,$rest)*);
        
        impl_spec!(block unary => Neg, neg, $Ty);
    };
    (block ty op => $Ty:ty, bin $(,$rest:tt)*) => {
        impl_spec!(block ty op => $Ty $(,$rest)*);

        impl_spec!(block all => BitOr, bitor, $Ty);
        impl_spec!(block all => BitAnd, bitand, $Ty);
        impl_spec!(block all => BitXor, bitxor, $Ty);
        impl_spec!(block unary => Not, not, $Ty);
    };
    (block ty op => $Ty:ty, norm $(,$rest:tt)*) => {
        impl_spec!(block ty op => $Ty $(,$rest)*);

        impl_spec!(block all => Add, add, $Ty);
        impl_spec!(block all => Sub, sub, $Ty);
        impl_spec!(block all => Mul, mul, $Ty);
        impl_spec!(block all => Div, div, $Ty);
        impl_spec!(block all => Rem, rem, $Ty);
    };
    (block ty unsign => $Ty:ident, $CTy:ident) => {
        impl_spec!(block ty op => Complex<$Ty>, norm);
        impl_spec!(block ty op => $Ty, norm, bin);
    };
    (block ty sign => $Ty:ident, $CTy:ident) => {
        impl_spec!(block ty op => Complex<$Ty>, norm, neg);
        impl_spec!(block ty op => $Ty, norm, neg, bin);
    };
    (block ty float => $Ty:ident, $CTy:ident) => {
        impl_spec!(block ty op => Complex<$Ty>, norm, neg);
        impl_spec!(block ty op => $Ty, norm, neg);
    };
    (block ty => bool, $CTy:ident) => {
        // impl_spec!(block ty op => Complex<bool>, bin);
        // impl_spec!(block ty op => bool, bin);
    };
}

#[cfg(feature = spec_bool)]
impl_spec!(block ty => bool, ComplexBool);
#[cfg(feature = spec_f32_f64)]
impl_spec!(block ty float => f32, ComplexF32);
#[cfg(feature = spec_f32_f64)]
impl_spec!(block ty float => f64, ComplexF64);
#[cfg(feature = spec_u8_u16)]
impl_spec!(block ty unsign => u8, ComplexU8);
#[cfg(feature = spec_u8_u16)]
impl_spec!(block ty unsign => u16, ComplexU16);
#[cfg(feature = spec_u32_u64)]
impl_spec!(block ty unsign => u32, ComplexU32);
#[cfg(feature = spec_u32_u64)]
impl_spec!(block ty unsign => u64, ComplexU64);
#[cfg(feature = spec_u128)]
impl_spec!(block ty unsign => u128, ComplexU128);
#[cfg(feature = spec_usize)]
impl_spec!(block ty unsign => usize, ComplexUsize);
#[cfg(feature = spec_i8_i16)]
impl_spec!(block ty sign => i8, ComplexI8);
#[cfg(feature = spec_i8_i16)]
impl_spec!(block ty sign => i16, ComplexI16);
#[cfg(feature = spec_i32_i64)]
impl_spec!(block ty sign => i32, ComplexI32);
#[cfg(feature = spec_i32_i64)]
impl_spec!(block ty sign => i64, ComplexI64);
#[cfg(feature = spec_i128)]
impl_spec!(block ty sign => i128, ComplexI128);
#[cfg(feature = spec_isize)]
impl_spec!(block ty sign => isize, ComplexIsize);

#[test]
fn t1() {
    use std::convert::TryFrom;

    let v: Vector<f32, U2> = Vector::try_from(vec![0.0, 1.0]).unwrap();
    let w: Vector<f32, U2> = Vector::try_from(vec![2.0, 3.0]).unwrap();
    let o: Vector<f32, U2> = Vector::try_from(vec![2.0, 4.0]).unwrap();

    assert_eq!(v + w, o);
}
