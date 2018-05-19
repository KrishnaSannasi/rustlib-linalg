use std::ops::{Add, Sub, Mul, Div};

use super::Vector;

macro_rules! apply {
    (vec => $t: expr, $u: expr, $e: expr) => {
        if $t.dim() != $u.dim() {
            return Err(format!("incompatible sizes"))
        }
        else {
            Ok(Vector::from($t.value.iter()
                                    .zip($u.value.iter())
                                    .map($e)
                                    .collect::<Vec<O>>()))
        }
    };
    (scl => $t: expr, $e: expr) => {
        Vector::from($t.value.iter()
                       .map($e)
                       .collect::<Vec<O>>())
    };
}

impl<T, U, O> Add<Vector<U>> for Vector<T>
    where T: Clone + Copy + Add<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn add(self, rhs: Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i + j)
    }
}

impl<T, U, O> Sub<Vector<U>> for Vector<T>
    where T: Clone + Copy + Sub<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn sub(self, rhs: Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i - j)
    }
}

impl<T, U, O> Mul<Vector<U>> for Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i * j)
    }
}

impl<T, U, O> Div<Vector<U>> for Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn div(self, rhs: Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i / j)
    }
}

impl<T, U, O> Mul<U> for Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn mul(self, rhs: U) -> Self::Output {
        apply!(scl => self, |&i| i * rhs)
    }
}

impl<T, U, O> Div<U> for Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn div(self, rhs: U) -> Self::Output {
        apply!(scl => self, |&i| i / rhs)
    }
}

impl<'a, 'b, T, U, O> Add<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Add<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn add(self, rhs: &'b Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i + j)
    }
}

impl<'a, 'b, T, U, O> Sub<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Sub<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn sub(self, rhs: &'b Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i - j)
    }
}

impl<'a, 'b, T, U, O> Mul<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn mul(self, rhs: &'b Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i * j)
    }
}

impl<'a, 'b, T, U, O> Div<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn div(self, rhs: &'b Vector<U>) -> Self::Output {
        apply!(vec => self, rhs, |(&i, &j)| i / j)
    }
}

impl<'a, 'b, T, U, O> Mul<&'b U> for &'a Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn mul(self, rhs: &'b U) -> Self::Output {
        apply!(scl => self, |&i| i * *rhs)
    }
}

impl<'a, 'b, T, U, O> Div<&'b U> for &'a Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn div(self, rhs: &'b U) -> Self::Output {
        apply!(scl => self, |&i| i / *rhs)
    }
}
