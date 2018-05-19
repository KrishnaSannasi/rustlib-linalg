use std::ops::{Add, Sub, Mul, Div};

use super::Vector;

fn apply<T, U, O, F>(t: Vector<T>, u: Vector<U>, f: F) -> Result<Vector<O>, String>
    where T: Clone + Copy,
          U: Clone + Copy,
          O: Clone + Copy,
          F: Fn((&T, &U)) -> O {
    if t.dim() != u.dim() {
        return Err(format!("incompatible sizes"))
    }
    else {
        Ok(Vector::from(t.value.iter()
                               .zip(u.value.iter())
                               .map(f)
                               .collect::<Vec<O>>()))
    }
}

fn apply_ref<'a, 'b, T, U, O, F>(t: &'a Vector<T>, u: &'b Vector<U>, f: F) -> Result<Vector<O>, String>
    where T: Clone + Copy,
          U: Clone + Copy,
          O: Clone + Copy,
          F: Fn((&T, &U)) -> O {
    if t.dim() != u.dim() {
        return Err(format!("incompatible sizes"))
    }
    else {
        Ok(Vector::from(t.value.iter()
                               .zip(u.value.iter())
                               .map(f)
                               .collect::<Vec<O>>()))
    }
}

fn apply_const<T, O, F>(t: Vector<T>, f: F) -> Vector<O>
    where T: Clone + Copy,
          O: Clone + Copy,
          F: FnMut(&T) -> O {
    Vector::from(t.value.iter()
                        .map(f)
                        .collect::<Vec<O>>())
}

fn apply_const_ref<'a, T, O, F>(t: &'a Vector<T>, f: F) -> Vector<O>
    where T: Clone + Copy,
          O: Clone + Copy,
          F: FnMut(&'a T) -> O {
    Vector::from(t.value.iter()
                        .map(f)
                        .collect::<Vec<O>>())
}

impl<T, U, O> Add<Vector<U>> for Vector<T>
    where T: Clone + Copy + Add<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn add(self, rhs: Vector<U>) -> Self::Output {
        apply(self, rhs, |(&t, &u)| t + u)
    }
}

impl<T, U, O> Sub<Vector<U>> for Vector<T>
    where T: Clone + Copy + Sub<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn sub(self, rhs: Vector<U>) -> Self::Output {
        apply(self, rhs, |(&t, &u)| t - u)
    }
}

impl<T, U, O> Mul<Vector<U>> for Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn mul(self, rhs: Vector<U>) -> Self::Output {
        apply(self, rhs, |(&t, &u)| t * u)
    }
}

impl<T, U, O> Div<Vector<U>> for Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn div(self, rhs: Vector<U>) -> Self::Output {
        apply(self, rhs, |(&t, &u)| t / u)
    }
}

impl<T, U, O> Mul<U> for Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn mul(self, rhs: U) -> Self::Output {
        apply_const(self, |&t| t * rhs)
    }
}

impl<T, U, O> Div<U> for Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn div(self, rhs: U) -> Self::Output {
        apply_const(self, |&t| t / rhs)
    }
}

impl<'a, 'b, T, U, O> Add<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Add<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn add(self, rhs: &'b Vector<U>) -> Self::Output {
        apply_ref(self, rhs, |(&t, &u)| t + u)
    }
}

impl<'a, 'b, T, U, O> Sub<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Sub<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn sub(self, rhs: &'b Vector<U>) -> Self::Output {
        apply_ref(self, rhs, |(&t, &u)| t - u)
    }
}

impl<'a, 'b, T, U, O> Mul<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn mul(self, rhs: &'b Vector<U>) -> Self::Output {
        apply_ref(self, rhs, |(&t, &u)| t * u)
    }
}

impl<'a, 'b, T, U, O> Div<&'b Vector<U>> for &'a Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Result<Vector<O>, String>;

    fn div(self, rhs: &'b Vector<U>) -> Self::Output {
        apply_ref(self, rhs, |(&t, &u)| t / u)
    }
}

impl<'a, 'b, T, U, O> Mul<&'b U> for &'a Vector<T>
    where T: Clone + Copy + Mul<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn mul(self, rhs: &'b U) -> Self::Output {
        apply_const_ref(self, |&t| t * *rhs)
    }
}

impl<'a, 'b, T, U, O> Div<&'b U> for &'a Vector<T>
    where T: Clone + Copy + Div<U, Output = O>,
          U: Clone + Copy,
          O: Clone + Copy {
    type Output = Vector<O>;

    fn div(self, rhs: &'b U) -> Self::Output {
        apply_const_ref(self, |&t| t / *rhs)
    }
}
