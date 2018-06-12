use std::ops::{Add, Sub, Mul, Div};
use std::convert::TryFrom;

use super::Vector;
use super::typenum::Unsigned;

macro_rules! apply {
    (vec => $t: expr, $u: expr, $e: expr) => {
        Vector::try_from($t.value.iter()
                             .zip($u.value.iter())
                             .map($e)
                             .collect::<Vec<O>>())
                             .unwrap()
    };
    (scl => $t: expr, $e: expr) => {
        Vector::try_from($t.value.iter()
                       .map($e)
                       .collect::<Vec<O>>())
                       .unwrap()
    };
}

impl<S: Unsigned, T, U, O> Add<Vector<U, S>> for Vector<T, S>
    where T: Copy + Add<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn add(self, rhs: Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i + j)
    }
}

impl<S: Unsigned, T, U, O> Sub<Vector<U, S>> for Vector<T, S>
    where T: Copy + Sub<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn sub(self, rhs: Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i - j)
    }
}

impl<S: Unsigned, T, U, O> Mul<Vector<U, S>> for Vector<T, S>
    where T: Copy + Mul<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn mul(self, rhs: Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i * j)
    }
}

impl<S: Unsigned, T, U, O> Div<Vector<U, S>> for Vector<T, S>
    where T: Copy + Div<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn div(self, rhs: Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i / j)
    }
}

impl<S: Unsigned, T, U, O> Mul<U> for Vector<T, S>
    where T: Copy + Mul<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn mul(self, rhs: U) -> Self::Output {
        apply!(scl => self, |&i| i * rhs)
    }
}

impl<S: Unsigned, T, U, O> Div<U> for Vector<T, S>
    where T: Copy + Div<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn div(self, rhs: U) -> Self::Output {
        apply!(scl => self, |&i| i / rhs)
    }
}

impl<'a, 'b, S: Unsigned, T, U, O> Add<&'b Vector<U, S>> for &'a Vector<T, S>
    where T: Copy + Add<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn add(self, rhs: &'b Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i + j)
    }
}

impl<'a, 'b, S: Unsigned, T, U, O> Sub<&'b Vector<U, S>> for &'a Vector<T, S>
    where T: Copy + Sub<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn sub(self, rhs: &'b Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i - j)
    }
}

impl<'a, 'b, S: Unsigned, T, U, O> Mul<&'b Vector<U, S>> for &'a Vector<T, S>
    where T: Copy + Mul<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn mul(self, rhs: &'b Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i * j)
    }
}

impl<'a, 'b, S: Unsigned, T, U, O> Div<&'b Vector<U, S>> for &'a Vector<T, S>
    where T: Copy + Div<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn div(self, rhs: &'b Vector<U, S>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i / j)
    }
}

impl<'a, 'b, S: Unsigned, T, U, O> Mul<&'b U> for &'a Vector<T, S>
    where T: Copy + Mul<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn mul(self, rhs: &'b U) -> Self::Output {
        apply!(scl => self, |&i| i * *rhs)
    }
}

impl<'a, 'b, S: Unsigned, T, U, O> Div<&'b U> for &'a Vector<T, S>
    where T: Copy + Div<U, Output = O>,
          U: Copy,
          O: Copy {
    type Output = Vector<O, S>;

    fn div(self, rhs: &'b U) -> Self::Output {
        apply!(scl => self, |&i| i / *rhs)
    }
}
