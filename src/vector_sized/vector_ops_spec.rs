use super::Vector;
use std::ops::*;
use num::complex::Complex;
use super::typenum::*;

macro_rules! impl_spec {
    (block => $fun:tt, $S:ident, $output:ident, $self:ident $(,$other:tt)*) => { // using star because ? is unstable
        {
            for i in 0..$S::to_usize() {
                $output.value[i] = $self.value[i].$fun($($other[i])*);
            }
            $output
        }
    };
    (block own own => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<S: Unsigned> $Op<Vector<$Ty, S>> for Vector<$Ty, S> {
            fn $fun(mut self, other: Vector<$Ty, S>) -> Self::Output {
                impl_spec!(block => $fun, S, self, self, other)
            }
        }
    };
    (block own borrow => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, S: Unsigned> $Op<&'a Vector<$Ty, S>> for Vector<$Ty, S> {
            fn $fun(mut self, other: &'a Vector<$Ty, S>) -> Self::Output {
                impl_spec!(block => $fun, S, self, self, other)
            }
        }
    };
    (block borrow own => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, S: Unsigned> $Op<Vector<$Ty, S>> for &'a Vector<$Ty, S> {
            fn $fun(self, mut other: Vector<$Ty, S>) -> Self::Output {
                impl_spec!(block => $fun, S, other, self, other)
            }
        }
    };
    (block unary => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, S: Unsigned> $Op for Vector<$Ty, S> {
            fn $fun(mut self) -> Self::Output {
                impl_spec!(block => $fun, S, self, self)
            }
        }
    };
    (block all => $Op:ident, $fun:ident, $Ty:ty) => {
        impl_spec!(block own own => $Op, $fun, $Ty);
        impl_spec!(block own borrow => $Op, $fun, $Ty);
        impl_spec!(block borrow own => $Op, $fun, $Ty);
    };
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
    (block ty op => $Ty:ty) => {
        impl_spec!(block all => Add, add, $Ty);
        impl_spec!(block all => Sub, sub, $Ty);
        impl_spec!(block all => Mul, mul, $Ty);
        impl_spec!(block all => Div, div, $Ty);
        impl_spec!(block all => Rem, rem, $Ty);
    };
    (block ty unsign => $Ty:ident, $CTy:ident) => {
        impl_spec!(block ty op => Complex<$Ty>);
        impl_spec!(block ty op => $Ty, bin);
    };
    (block ty sign => $Ty:ident, $CTy:ident) => {
        impl_spec!(block ty op => Complex<$Ty>, neg);
        impl_spec!(block ty op => $Ty, neg, bin);
    };
    (block ty float => $Ty:ident, $CTy:ident) => {
        impl_spec!(block ty op => Complex<$Ty>, neg);
        impl_spec!(block ty op => $Ty, neg);
    };
}

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
