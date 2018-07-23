#[cfg(feature = "no_std")]
use core::ops::*;
#[cfg(not(feature = "no_std"))]
use std::ops::*;

use super::{Vector, InVector};
use super::generic_array::{GenericArray, ArrayLength};

macro_rules! impl_op {
    (build => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            self.iter()
                      .map(|&i| i $op rhs)
                      .collect::<GenericArray<_, _>>()
                      .into()
        }
    };
    (build bin => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            self.iter()
                      .zip(rhs.iter())
                      .map(|(&i, &j)| i $op j)
                      .collect::<GenericArray<_, _>>()
                      .into()
        }
    };
    (build assign => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            for i in 0..N::to_usize() {
                self[i] $op rhs;
            }
        }
    };
    (build bin assign => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            for i in 0..N::to_usize() {
                self[i] $op rhs[i];
            }
        }
    };
    (own => $Op:ident, $func:ident, $op:tt) => {
        impl<N, T, U, O> $Op<U> for Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector, N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            impl_op!(build => $func, $op, U);
        }
    };
    (borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U, O> $Op<U> for &'a Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector, N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            impl_op!(build => $func, $op, U);
        }
    };
    (own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<N, T, U, O> $Op<Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            impl_op!(build bin => $func, $op, Vector<U, N>);
        }
    };
    (own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U, O> $Op<&'a Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            impl_op!(build bin => $func, $op, &'a Vector<U, N>);
        }
    };
    (borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U, O> $Op<Vector<U, N>> for &'a Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            impl_op!(build bin => $func, $op, Vector<U, N>);
        }
    };
    (borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U, O> $Op<&'a Vector<U, N>> for &'a Vector<T, N>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector,
                  N: ArrayLength<T> + ArrayLength<U> + ArrayLength<O> {
            type Output = Vector<O, N>;

            impl_op!(build bin => $func, $op, &'a Vector<U, N>);
        }
    };
    (assign, own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<N, T, U> $Op<Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            impl_op!(build bin assign => $func, $op, Vector<U, N>);
        }
    };
    (assign, own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U> $Op<&'a Vector<U, N>> for Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            impl_op!(build bin assign => $func, $op, &'a Vector<U, N>);
        }
    };
    (assign, borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U> $Op<Vector<U, N>> for &'a mut Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            impl_op!(build bin assign => $func, $op, Vector<U, N>);
        }
    };
    (assign, borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, N, T, U> $Op<&'a Vector<U, N>> for &'a mut Vector<T, N>
            where T: InVector + $Op<U>,
                  U: InVector, N: ArrayLength<T> + ArrayLength<U> {
            impl_op!(build bin assign => $func, $op, &'a Vector<U, N>);
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
    default fn neg(self) -> Self::Output { -&self }
}

impl<'a, T: InVector, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Neg for &'a Vector<T, N>
where T: Neg<Output = O> {
    type Output = Vector<O, N>;
    fn neg(self) -> Self::Output {
        self.map(|&x| -x)
    }
}

impl<T: InVector, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Not for Vector<T, N>
where T: Not<Output = O> {
    type Output = Vector<O, N>;
    default fn not(self) -> Self::Output { !&self }
}

impl<'a, T: InVector, N: ArrayLength<T> + ArrayLength<O>, O: InVector> Not for &'a Vector<T, N>
where T: Not<Output = O> {
    type Output = Vector<O, N>;
    fn not(self) -> Self::Output {
        self.map(|&x| !x)
    }
}
