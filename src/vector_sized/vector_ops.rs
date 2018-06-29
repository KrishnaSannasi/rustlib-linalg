use std::ops::*;

use super::{Vector, Vectorizable};
use super::typenum::Unsigned;

macro_rules! impl_op {
    (build => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            Vector::make(self.value.iter()
                                   .map(|&i| i $op rhs)
                                   .collect::<Vec<O>>())
        }
    };
    (build bin => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            Vector::make(self.value.iter()
                                   .zip(rhs.value.iter())
                                   .map(|(&i, &j)| i $op j)
                                   .collect::<Vec<O>>())
        }
    };
    (build assign => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            for i in 0..S::to_usize() {
                self.value[i] $op rhs;
            }
        }
    };
    (build bin assign => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            for i in 0..S::to_usize() {
                self.value[i] $op rhs.value[i];
            }
        }
    };
    (own => $Op:ident, $func:ident, $op:tt) => {
        impl<S, T, U, O> $Op<U> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U, Output = O>,
                  U: Vectorizable + Sized, O: Vectorizable + Sized, S: Unsigned {
            type Output = Vector<O, S>;

            impl_op!(build => $func, $op, U);
        }
    };
    (borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, S, T, U, O> $Op<U> for &'a Vector<T, S>
            where T: Vectorizable + $Op<U, Output = O>,
                  U: Vectorizable, O: Vectorizable, S: Unsigned {
            type Output = Vector<O, S>;

            impl_op!(build => $func, $op, U);
        }
    };
    (own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<S, T, U, O> $Op<Vector<U, S>> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U, Output = O>,
                  U: Vectorizable + Sized, O: Vectorizable + Sized, S: Unsigned {
            type Output = Vector<O, S>;

            impl_op!(build bin => $func, $op, Vector<U, S>);
        }
    };
    (own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'b, S, T, U, O> $Op<&'b Vector<U, S>> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U, Output = O>,
                  U: Vectorizable + Sized, O: Vectorizable + Sized, S: Unsigned {
            type Output = Vector<O, S>;

            impl_op!(build bin => $func, $op, &'b Vector<U, S>);
        }
    };
    (borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, S, T, U, O> $Op<Vector<U, S>> for &'a Vector<T, S>
            where T: Vectorizable + Sized + $Op<U, Output = O>,
                  U: Vectorizable + Sized, O: Vectorizable + Sized, S: Unsigned {
            type Output = Vector<O, S>;

            impl_op!(build bin => $func, $op, Vector<U, S>);
        }
    };
    (borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, 'b, S, T, U, O> $Op<&'b Vector<U, S>> for &'a Vector<T, S>
            where T: Vectorizable + Sized + $Op<U, Output = O>,
                  U: Vectorizable + Sized, O: Vectorizable + Sized, S: Unsigned {
            type Output = Vector<O, S>;

            impl_op!(build bin => $func, $op, &'b Vector<U, S>);
        }
    };
    (assign, own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<S, T, U> $Op<Vector<U, S>> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin assign => $func, $op, Vector<U, S>);
        }
    };
    (assign, own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'b, S, T, U> $Op<&'b Vector<U, S>> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin assign => $func, $op, &'b Vector<U, S>);
        }
    };
    (assign, borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, S, T, U> $Op<Vector<U, S>> for &'a mut Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin assign => $func, $op, Vector<U, S>);
        }
    };
    (assign, borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, 'b, S, T, U> $Op<&'b Vector<U, S>> for &'a mut Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin assign => $func, $op, &'b Vector<U, S>);
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

impl<T: Vectorizable, S: Unsigned, O: Vectorizable> Neg for Vector<T, S>
where T: Neg<Output = O> {
    type Output = Vector<O, S>;
    default fn neg(self) -> Self::Output { -&self }
}

impl<'a, T: Vectorizable, S: Unsigned, O: Vectorizable> Neg for &'a Vector<T, S>
where T: Neg<Output = O> {
    type Output = Vector<O, S>;
    fn neg(self) -> Self::Output {
        self.map(|&x| -x)
    }
}

impl<T: Vectorizable, S: Unsigned, O: Vectorizable> Not for Vector<T, S>
where T: Not<Output = O> {
    type Output = Vector<O, S>;
    default fn not(self) -> Self::Output { !&self }
}

impl<'a, T: Vectorizable, S: Unsigned, O: Vectorizable> Not for &'a Vector<T, S>
where T: Not<Output = O> {
    type Output = Vector<O, S>;
    fn not(self) -> Self::Output {
        self.map(|&x| !x)
    }
}
