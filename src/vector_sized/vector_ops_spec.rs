use super::Vector;

use std::ops::*;
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
    (block own own => $Op:ident, $fun:ident, $opass:tt, $Ty:ident, $S:ident) => {
        impl $Op<Vector<$Ty, $S>> for Vector<$Ty, $S> {
            fn $fun(mut self, other: Vector<$Ty, $S>) -> Self::Output {
                {
                    impl_spec!(block own => $opass, $S, self, other)
                }
            }
        }
    };
    (block own borrow => $Op:ident, $fun:ident, $opass:tt, $Ty:ident, $S:ident) => {
        impl<'a> $Op<&'a Vector<$Ty, $S>> for Vector<$Ty, $S> {
            type Output = Vector<$Ty, $S>;
            
            fn $fun(mut self, other: &'a Vector<$Ty, $S>) -> Self::Output {
                {
                    impl_spec!(block own => $opass, $S, self, other)
                }
            }
        }
    };
    (block borrow own => $Op:ident, $fun:ident, $op:tt, $Ty:ident, $S:ident) => {
        impl<'b> $Op<Vector<$Ty, $S>> for &'b Vector<$Ty, $S> {
            fn $fun(self, other: Vector<$Ty, $S>) -> Self::Output {
                {
                    impl_spec!(block borrow => $op, $S, self, other)
                }
            }
        }
    };
    (block borrow borrow => $Op:ident, $fun:ident, $op:tt, $Ty:ident, $S:ident) => {
        impl<'a, 'b> $Op<&'a Vector<$Ty, $S>> for &'b Vector<$Ty, $S> {
            fn $fun(self, other: &'a Vector<$Ty, $S>) -> Self::Output {
                {
                    impl_spec!(block borrow => $op, $S, self, other)
                }
            }
        }
    };
    (block all => $Op:ident, $fun:ident, $opass:tt, $op:tt, $Ty:ident, $S:ident) => {
        impl_spec!(block own own => $Op, $fun, $opass, $Ty, $S);
        impl_spec!(block own borrow => $Op, $fun, $opass, $Ty, $S);
        impl_spec!(block borrow own => $Op, $fun, $op, $Ty, $S);
        impl_spec!(block borrow borrow => $Op, $fun, $op, $Ty, $S);
    };
    (block op => $Ty:ident, $S:ident) => {
        impl_spec!(block all => Add, add, +=, +, $Ty, $S);
        impl_spec!(block all => Sub, sub, -=, -, $Ty, $S);
        impl_spec!(block all => Mul, mul, *=, *, $Ty, $S);
        impl_spec!(block all => Div, div, /=, /, $Ty, $S);
    };
    (block op ty => $Ty:ident) => {
        impl_spec!(block op => $Ty, U0);
        impl_spec!(block op => $Ty, U1);
        impl_spec!(block op => $Ty, U2);
        impl_spec!(block op => $Ty, U3);
        impl_spec!(block op => $Ty, U4);
        impl_spec!(block op => $Ty, U5);
        impl_spec!(block op => $Ty, U6);
        impl_spec!(block op => $Ty, U7);
        impl_spec!(block op => $Ty, U8);
        impl_spec!(block op => $Ty, U9);
        impl_spec!(block op => $Ty, U10);
    };
}

impl_spec!(block op ty => f32);
impl_spec!(block op ty => f64);
impl_spec!(block op ty => u8);
impl_spec!(block op ty => u16);
impl_spec!(block op ty => u32);
impl_spec!(block op ty => u64);
impl_spec!(block op ty => u128);
impl_spec!(block op ty => usize);
impl_spec!(block op ty => i8);
impl_spec!(block op ty => i16);
impl_spec!(block op ty => i32);
impl_spec!(block op ty => i64);
impl_spec!(block op ty => i128);
impl_spec!(block op ty => isize);

#[test]
fn t1() {
    use std::convert::TryFrom;

    let v: Vector<f32, U2> = Vector::try_from(vec![0.0, 1.0]).unwrap();
    let w: Vector<f32, U2> = Vector::try_from(vec![2.0, 3.0]).unwrap();
    let o: Vector<f32, U2> = Vector::try_from(vec![2.0, 4.0]).unwrap();

    assert_eq!(v + w, o);
}