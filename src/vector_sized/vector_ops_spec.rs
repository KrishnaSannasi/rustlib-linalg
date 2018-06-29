use super::Vector;
use std::ops::*;
use num::complex::Complex;
use super::typenum::*;

macro_rules! impl_spec {
    (block => $opass:tt, $S:ident, $self:ident, $other:ident) => {
        {
            $self $opass $other;
            $self
        }
    };
    (block own => $Op:ident, $fun:ident, $opass:tt, $Ty:ty) => {
        impl<S: Unsigned> $Op<Vector<$Ty, S>> for Vector<$Ty, S> {
            fn $fun(mut self, other: Vector<$Ty, S>) -> Self::Output {
                {
                    impl_spec!(block => $opass, S, self, other)
                }
            }
        }
    };
    (block borrow => $Op:ident, $fun:ident, $opass:tt, $Ty:ty) => {
        impl<'a, S: Unsigned> $Op<&'a Vector<$Ty, S>> for Vector<$Ty, S> {
            fn $fun(mut self, other: &'a Vector<$Ty, S>) -> Self::Output {
                {
                    impl_spec!(block => $opass, S, self, other)
                }
            }
        }
    };
    (block unary => $Op:ident, $fun:ident, $Ty:ty) => {
        impl<'a, S: Unsigned> $Op for Vector<$Ty, S> {
            fn $fun(mut self) -> Self::Output {
                {
                    for i in 0..S::to_usize() {
                        self.value[i] = self.value[i].$fun();
                    }
                    self
                }
            }
        }
    };
    (block all => $Op:ident, $fun:ident, $opass:tt, $Ty:ty) => {
        impl_spec!(block own => $Op, $fun, $opass, $Ty);
        impl_spec!(block borrow => $Op, $fun, $opass, $Ty);
    };
    (block ty op => $Ty:ty, neg $(,$rest:tt)*) => {
        impl_spec!(block ty op => $Ty $(,$rest)*);
        
        impl_spec!(block unary => Neg, neg, $Ty);
    };
    (block ty op => $Ty:ty, bin $(,$rest:tt)*) => {
        impl_spec!(block ty op => $Ty $(,$rest)*);

        impl_spec!(block all => BitOr, bitor, |=, $Ty);
        impl_spec!(block all => BitAnd, bitand, &=, $Ty);
        impl_spec!(block all => BitXor, bitxor, ^=, $Ty);
    };
    (block ty op => $Ty:ty) => {
        impl_spec!(block all => Add, add, +=, $Ty);
        impl_spec!(block all => Sub, sub, -=, $Ty);
        impl_spec!(block all => Mul, mul, *=, $Ty);
        impl_spec!(block all => Div, div, /=, $Ty);
        impl_spec!(block all => Rem, rem, %=, $Ty);
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

impl_spec!(block ty float => f32, ComplexF32);
impl_spec!(block ty float => f64, ComplexF64);
impl_spec!(block ty unsign => u8, ComplexU8);
impl_spec!(block ty unsign => u16, ComplexU16);
impl_spec!(block ty unsign => u32, ComplexU32);
impl_spec!(block ty unsign => u64, ComplexU64);
impl_spec!(block ty unsign => u128, ComplexU128);
impl_spec!(block ty unsign => usize, ComplexUsize);
impl_spec!(block ty sign => i8, ComplexI8);
impl_spec!(block ty sign => i16, ComplexI16);
impl_spec!(block ty sign => i32, ComplexI32);
impl_spec!(block ty sign => i64, ComplexI64);
impl_spec!(block ty sign => i128, ComplexI128);
impl_spec!(block ty sign => isize, ComplexIsize);

#[test]
fn t1() {
    use std::convert::TryFrom;

    let v: Vector<f32, U2> = Vector::try_from(vec![0.0, 1.0]).unwrap();
    let w: Vector<f32, U2> = Vector::try_from(vec![2.0, 3.0]).unwrap();
    let o: Vector<f32, U2> = Vector::try_from(vec![2.0, 4.0]).unwrap();

    assert_eq!(v + w, o);
}
