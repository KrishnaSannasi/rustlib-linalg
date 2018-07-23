use super::{Vector, InVector};

use std::prelude::v1::*;
use std::ops::*;

macro_rules! impl_op {
    (build => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            Vector { value: self.value.iter()
                                      .map(|&i| i $op rhs)
                                      .collect::<Vec<O>>() }
        }
    };
    (build bin => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            assert!(self.dim() == rhs.dim());
            Vector { value: self.value.iter()
                                      .zip(rhs.value.iter())
                                      .map(|(&i, &j)| i $op j)
                                      .collect::<Vec<O>>() }
        }
    };
    (build assign => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            self.value.iter_mut()
                      .for_each(|a| *a $op rhs);
        }
    };
    (build bin assign => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            assert!(self.dim() == rhs.dim());
            self.value.iter_mut()
                      .zip(rhs.value.iter())
                      .for_each(|(a, b)| *a $op *b);
        }
    };
    (own => $Op:ident, $func:ident, $op:tt) => {
        impl<T, U, O> $Op<U> for Vector<T>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector {
            type Output = Vector<O>;

            impl_op!(build => $func, $op, U);
        }
    };
    (borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U, O> $Op<U> for &'a Vector<T>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector {
            type Output = Vector<O>;

            impl_op!(build => $func, $op, U);
        }
    };
    (own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<T, U, O> $Op<Vector<U>> for Vector<T>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector {
            type Output = Vector<O>;

            impl_op!(build bin => $func, $op, Vector<U>);
        }
    };
    (own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U, O> $Op<&'a Vector<U>> for Vector<T>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector {
            type Output = Vector<O>;

            impl_op!(build bin => $func, $op, &'a Vector<U>);
        }
    };
    (borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U, O> $Op<Vector<U>> for &'a Vector<T>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector {
            type Output = Vector<O>;

            impl_op!(build bin => $func, $op, Vector<U>);
        }
    };
    (borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U, O> $Op<&'a Vector<U>> for &'a Vector<T>
            where T: InVector + $Op<U, Output = O>,
                  U: InVector, O: InVector {
            type Output = Vector<O>;

            impl_op!(build bin => $func, $op, &'a Vector<U>);
        }
    };
    (assign, own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<T, U> $Op<Vector<U>> for Vector<T>
            where T: InVector + $Op<U>,
                  U: InVector {
            impl_op!(build bin assign => $func, $op, Vector<U>);
        }
    };
    (assign, own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U> $Op<&'a Vector<U>> for Vector<T>
            where T: InVector + $Op<U>,
                  U: InVector {
            impl_op!(build bin assign => $func, $op, &'a Vector<U>);
        }
    };
    (assign, borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U> $Op<Vector<U>> for &'a mut Vector<T>
            where T: InVector + $Op<U>,
                  U: InVector {
            impl_op!(build bin assign => $func, $op, Vector<U>);
        }
    };
    (assign, borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, T, U> $Op<&'a Vector<U>> for &'a mut Vector<T>
            where T: InVector + $Op<U>,
                  U: InVector {
            impl_op!(build bin assign => $func, $op, &'a Vector<U>);
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

impl<T: InVector, O: InVector> Neg for Vector<T>
where T: Neg<Output = O> {
    type Output = Vector<O>;
    default fn neg(self) -> Self::Output { -&self }
}

impl<'a, T: InVector, O: InVector> Neg for &'a Vector<T>
where T: Neg<Output = O> {
    type Output = Vector<O>;
    fn neg(self) -> Self::Output {
        self.map(|&x| -x)
    }
}

impl<T: InVector, O: InVector> Not for Vector<T>
where T: Not<Output = O> {
    type Output = Vector<O>;
    default fn not(self) -> Self::Output { !&self }
}

impl<'a, T: InVector, O: InVector> Not for &'a Vector<T>
where T: Not<Output = O> {
    type Output = Vector<O>;
    fn not(self) -> Self::Output {
        self.map(|&x| !x)
    }
}
