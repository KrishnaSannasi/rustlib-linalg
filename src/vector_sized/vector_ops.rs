use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::convert::TryFrom;

use super::{Vector, Vectorizable};
use super::typenum::Unsigned;

macro_rules! impl_op {
    (build => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            Vector::try_from(self.value.iter()
                                       .map(|&i| i $op rhs)
                                       .collect::<Vec<O>>())
                                       .unwrap()
        }
    };
    (build bin => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(self, rhs: $RHS) -> Self::Output {
            Vector::try_from(self.value.iter()
                                       .zip(rhs.value.iter())
                                       .map(|(&i, &j)| i $op j)
                                       .collect::<Vec<O>>())
                                       .unwrap()
        }
    };
    (build ass => $func:ident, $op:tt, $RHS:ty) => {
        default fn $func(&mut self, rhs: $RHS) {
            for i in 0..S::to_usize() {
                self.value[i] $op rhs;
            }
        }
    };
    (build bin ass => $func:ident, $op:tt, $RHS:ty) => {
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
    (ass, own, own => $Op:ident, $func:ident, $op:tt) => {
        impl<S, T, U> $Op<Vector<U, S>> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin ass => $func, $op, Vector<U, S>);
        }
    };
    (ass, own, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'b, S, T, U> $Op<&'b Vector<U, S>> for Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin ass => $func, $op, &'b Vector<U, S>);
        }
    };
    (ass, borrow, own => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, S, T, U> $Op<Vector<U, S>> for &'a mut Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin ass => $func, $op, Vector<U, S>);
        }
    };
    (ass, borrow, borrow => $Op:ident, $func:ident, $op:tt) => {
        impl<'a, 'b, S, T, U> $Op<&'b Vector<U, S>> for &'a mut Vector<T, S>
            where T: Vectorizable + Sized + $Op<U>,
                  U: Vectorizable + Sized, S: Unsigned {
            impl_op!(build bin ass => $func, $op, &'b Vector<U, S>);
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
    };
    (op ass all => $self_type:tt, $other_type:tt) => {
        impl_op!(op ass => AddAssign, add_assign, += => $self_type, $other_type);
        impl_op!(op ass => SubAssign, sub_assign, -= => $self_type, $other_type);
        impl_op!(op ass => MulAssign, mul_assign, *= => $self_type, $other_type);
        impl_op!(op ass => DivAssign, div_assign, /= => $self_type, $other_type);
    };
    (op => $Op:ident, $func:ident, $op:tt => $self_type:tt, $other_type:tt) => {
        impl_op!($self_type, $other_type => $Op, $func, $op);
    };
    (op ass => $Op:ident, $func:ident, $op:tt => $self_type:tt, $other_type:tt) => {
        impl_op!(ass, $self_type, $other_type => $Op, $func, $op);
    };
}

impl_op!(op all => own, own);
impl_op!(op all => own, borrow);
impl_op!(op all => borrow, own);
impl_op!(op all => borrow, borrow);

impl_op!(op ass all => own, own);
impl_op!(op ass all => own, borrow);
impl_op!(op ass all => borrow, own);
impl_op!(op ass all => borrow, borrow);

impl_op!(op => Mul, mul, * => own);
impl_op!(op => Mul, mul, * => borrow);

impl_op!(op => Div, div, / => own);
impl_op!(op => Div, div, / => borrow);
