use super::Vector;
use std::ops::*;
use num::complex::Complex;
use super::typenum::*;

macro_rules! impl_spec {
    (block own => $opass:tt, $S:ident, $self:ident, $other:ident) => {
        {
            for i in 0..$S::to_usize() {
                $self.value[i] $opass $other.value[i];
            }
        
            $self
        }
    };
    (block borrow => $op:tt, $S:ident, $self:ident, $other:ident) => {
        {
            let mut output: Self::Output = Vector::new();
            for i in 0..$S::to_usize() {
                output.value[i] = $self.value[i] $op $other.value[i];
            }
            output
        }
    };
    (block own own => $Op:ident, $fun:ident, $opass:tt, $Ty:ident) => {
        impl<S: Unsigned> $Op<Vector<$Ty, S>> for Vector<$Ty, S> {
            fn $fun(mut self, other: Vector<$Ty, S>) -> Self::Output {
                {
                    impl_spec!(block own => $opass, S, self, other)
                }
            }
        }
    };
    (block own borrow => $Op:ident, $fun:ident, $opass:tt, $Ty:ident) => {
        impl<'a, S: Unsigned> $Op<&'a Vector<$Ty, S>> for Vector<$Ty, S> {
            fn $fun(mut self, other: &'a Vector<$Ty, S>) -> Self::Output {
                {
                    impl_spec!(block own => $opass, S, self, other)
                }
            }
        }
    };
    (block borrow own => $Op:ident, $fun:ident, $op:tt, $Ty:ident) => {
        impl<'b, S: Unsigned> $Op<Vector<$Ty, S>> for &'b Vector<$Ty, S> {
            fn $fun(self, other: Vector<$Ty, S>) -> Self::Output {
                {
                    impl_spec!(block borrow => $op, S, self, other)
                }
            }
        }
    };
    (block borrow borrow => $Op:ident, $fun:ident, $op:tt, $Ty:ident) => {
        impl<'a, 'b, S: Unsigned> $Op<&'a Vector<$Ty, S>> for &'b Vector<$Ty, S> {
            fn $fun(self, other: &'a Vector<$Ty, S>) -> Self::Output {
                {
                    impl_spec!(block borrow => $op, S, self, other)
                }
            }
        }
    };
    (block all => $Op:ident, $fun:ident, $opass:tt, $op:tt, $Ty:ident) => {
        impl_spec!(block own own => $Op, $fun, $opass, $Ty);
        impl_spec!(block own borrow => $Op, $fun, $opass, $Ty);
        impl_spec!(block borrow own => $Op, $fun, $op, $Ty);
        impl_spec!(block borrow borrow => $Op, $fun, $op, $Ty);
    };
    (block ty op => $Ty:ident) => {
        impl_spec!(block all => Add, add, +=, +, $Ty);
        impl_spec!(block all => Sub, sub, -=, -, $Ty);
        impl_spec!(block all => Mul, mul, *=, *, $Ty);
        impl_spec!(block all => Div, div, /=, /, $Ty);
    };
    (block ty => $Ty:ident, $CTy:ident) => {
        pub type $CTy = Complex<$Ty>;
        impl_spec!(block ty op => $CTy);
        impl_spec!(block ty op => $Ty);
    };
}

impl_spec!(block ty => f32, ComplexF32);
impl_spec!(block ty => f64, ComplexF64);
impl_spec!(block ty => u8, ComplexU8);
impl_spec!(block ty => u16, ComplexU16);
impl_spec!(block ty => u32, ComplexU32);
impl_spec!(block ty => u64, ComplexU64);
impl_spec!(block ty => u128, ComplexU128);
impl_spec!(block ty => usize, ComplexUsize);
impl_spec!(block ty => i8, ComplexI8);
impl_spec!(block ty => i16, ComplexI16);
impl_spec!(block ty => i32, ComplexI32);
impl_spec!(block ty => i64, ComplexI64);
impl_spec!(block ty => i128, ComplexI128);
impl_spec!(block ty => isize, ComplexIsize);

#[test]
fn t1() {
    use std::convert::TryFrom;

    let v: Vector<f32, U2> = Vector::try_from(vec![0.0, 1.0]).unwrap();
    let w: Vector<f32, U2> = Vector::try_from(vec![2.0, 3.0]).unwrap();
    let o: Vector<f32, U2> = Vector::try_from(vec![2.0, 4.0]).unwrap();

    assert_eq!(v + w, o);
}
