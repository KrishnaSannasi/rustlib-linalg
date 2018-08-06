#[cfg(feature = "no_std")]
use core::ops::*;
#[cfg(not(feature = "no_std"))]
use std::ops::*;

use super::{Vector, InVector};
use super::generic_array::ArrayLength;

macro_rules! impl_op {
    (own => $Op:ident, $func:ident, $op:tt) => {
        impl<N, T, U: Clone, O> $Op<U> for Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector, N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;
        
            default fn $func(self, rhs: U) -> Self::Output {
                self.map(|i| i.$func(rhs.clone()))
            }
        }
    };
    (borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T: Clone, U: Clone, O> $Op<U> for &'a Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector, N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;
        
            default fn $func(self, rhs: U) -> Self::Output {
                self.map_ref(|i| i.clone().$func(rhs.clone()))
            }
        }
    };
    (own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<N, T, U, O> $Op<Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            default fn $func(self, rhs: Vector<U, N>) -> Self::Output {
                Vector(self.into_iter()
                           .zip(rhs.into_iter())
                           .map(|(i, j)| i.$func(j))
                           .collect())
            }
        }
    };
    (own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U: Clone, O> $Op<&'a Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            default fn $func(self, rhs: &'a Vector<U, N>) -> Self::Output {
                Vector(self.into_iter()
                           .zip(rhs.iter())
                           .map(|(i, j)| i.$func(j.clone()))
                           .collect())
            }
        }
    };
    (borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T: Clone, U, O> $Op<Vector<U, N>> for &'a Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            default fn $func(self, rhs: Vector<U, N>) -> Self::Output {
                Vector(self.iter()
                           .zip(rhs.into_iter())
                           .map(|(i, j)| i.clone().$func(j))
                           .collect())
            }
        }
    };
    (borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T: Clone, U: Clone, O> $Op<&'a Vector<U, N>> for &'a Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            default fn $func(self, rhs: &'a Vector<U, N>) -> Self::Output {
                Vector(self.iter()
                           .zip(rhs.iter())
                           .map(|(i, j)| i.clone().$func(j.clone()))
                           .collect())
            }
        }
    };
    (assign, own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<N, T, U> $Op<Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            
            default fn $func(&mut self, rhs: Vector<U, N>) {
                self.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(i, j)| i.$func(j))
            }
        }
    };
    (assign, own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U: Clone> $Op<&'a Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            
            default fn $func(&mut self, rhs: &'a Vector<U, N>) {
                self.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(i, j)| i.$func(j.clone()))
            }
        }
    };
    (assign, borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U> $Op<Vector<U, N>> for &'a mut Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            
            default fn $func(&mut self, rhs: Vector<U, N>) {
                self.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(i, j)| i.$func(j))
            }
        }
    };
    (assign, borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U: Clone> $Op<&'a Vector<U, N>> for &'a mut Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            
            default fn $func(&mut self, rhs: &'a Vector<U, N>) {
                self.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(i, j)| i.$func(j.clone()))
            }
        }
    };
    (op => $Op:ident, $func:ident, $op:tt => $self_type:tt) => {
        impl_op!($self_type => $Op, $func, $op);
    };
    (op all => $self_type:tt, $other_type:tt) => {
        impl_op!(op => Add, add, + => $self_type, $other_type);
        impl_op!(op => Sub, sub, - => $self_type, $other_type);
        impl_op!(op => Mul, mul, * => $self_type, $other_type);
        impl_op!(op => Div, div, / => $self_type, $other_type);
        impl_op!(op => Rem, rem, % => $self_type, $other_type);
        
        impl_op!(op => BitAnd, bitand, & => $self_type, $other_type);
        impl_op!(op => BitOr, bitor, | => $self_type, $other_type);
        impl_op!(op => BitXor, bitxor, ^ => $self_type, $other_type);
        impl_op!(op => Shl, shl, << => $self_type, $other_type);
        impl_op!(op => Shr, shr, >> => $self_type, $other_type);
    };
    (op assign all => $self_type:tt, $other_type:tt) => {
        impl_op!(op assign => AddAssign, add_assign, += => $self_type, $other_type);
        impl_op!(op assign => SubAssign, sub_assign, -= => $self_type, $other_type);
        impl_op!(op assign => MulAssign, mul_assign, *= => $self_type, $other_type);
        impl_op!(op assign => DivAssign, div_assign, /= => $self_type, $other_type);
        impl_op!(op assign => RemAssign, rem_assign, %= => $self_type, $other_type);
        
        impl_op!(op assign => BitAndAssign, bitand_assign, &= => $self_type, $other_type);
        impl_op!(op assign => BitOrAssign, bitor_assign, |= => $self_type, $other_type);
        impl_op!(op assign => BitXorAssign, bitxor_assign, ^= => $self_type, $other_type);
        impl_op!(op assign => ShlAssign, shl_assign, <<= => $self_type, $other_type);
        impl_op!(op assign => ShrAssign, shr_assign, >>= => $self_type, $other_type);
    };
    (op => $Op:ident, $func:ident, $op:tt => $self_type:tt, $other_type:tt) => {
        impl_op!($self_type, $other_type => $Op, $func, $op);
    };
    (op assign => $Op:ident, $func:ident, $op:tt => $self_type:tt, $other_type:tt) => {
        impl_op!(assign, $self_type, $other_type => $Op, $func, $op);
    };
}

impl_op!(op all => own, own);
impl_op!(op all => own, borrow);
impl_op!(op all => borrow, own);
impl_op!(op all => borrow, borrow);

impl_op!(op assign all => own, own);
impl_op!(op assign all => own, borrow);
impl_op!(op assign all => borrow, own);
impl_op!(op assign all => borrow, borrow);

impl_op!(op => Mul, mul, * => own);
impl_op!(op => Mul, mul, * => borrow);

impl_op!(op => Div, div, / => own);
impl_op!(op => Div, div, / => borrow);

impl<T: InVector, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Neg for Vector<T, N>
where T: Neg<Output = O> {
    type Output = Vector<O, N>;

    default fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}

impl<'a, T: InVector + Clone, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Neg for &'a Vector<T, N>
where T: Neg<Output = O> {
    type Output = Vector<O, N>;

    fn neg(self) -> Self::Output {
        self.map_ref(|x| -x.clone())
    }
}

impl<T: InVector, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Not for Vector<T, N>
where T: Not<Output = O> {
    type Output = Vector<O, N>;

    default fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}

impl<'a, T: InVector + Clone, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Not for &'a Vector<T, N>
where T: Not<Output = O> {
    type Output = Vector<O, N>;
    
    fn not(self) -> Self::Output {
        self.map_ref(|x| !x.clone())
    }
}
