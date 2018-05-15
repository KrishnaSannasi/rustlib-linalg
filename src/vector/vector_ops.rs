use std::ops::{Add, Sub, Mul, Div};

use Vector;

impl<T> Add<Vector<T>> for Vector<T>
    where T: Clone + Copy + Add<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        &self + &rhs
    }
}

impl<T> Sub<Vector<T>> for Vector<T>
    where T: Clone + Copy + Sub<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        &self - &rhs
    }
}

impl<T> Mul<Vector<T>> for Vector<T>
    where T: Clone + Copy + Mul<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T> Div<Vector<T>> for Vector<T>
    where T: Clone + Copy + Div<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn div(self, rhs: Vector<T>) -> Self::Output {
        &self / &rhs
    }
}

impl<T> Mul<T> for Vector<T>
    where T: Clone + Copy + Mul<T, Output = T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

impl<T> Div<T> for Vector<T>
    where T: Clone + Copy + Div<T, Output = T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        &self / rhs
    }
}

impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T>
    where T: Clone + Copy + Add<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn add(self, rhs: &'b Vector<T>) -> Self::Output {
        if self.dim() != rhs.dim() {
            return Err(format!("incompatible sizes"))
        }

        let mut vec = Vec::new();

        for (i, j) in self.value.iter().zip(rhs.value.iter()) {
            vec.push(*i + *j)
        }

        Ok(Vector::from(vec))
    }
}

impl<'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T>
    where T: Clone + Copy + Sub<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn sub(self, rhs: &'b Vector<T>) -> Self::Output {
        if self.dim() != rhs.dim() {
            return Err(format!("incompatible sizes"))
        }

        let mut vec = Vec::new();

        for (i, j) in self.value.iter().zip(rhs.value.iter()) {
            vec.push(*i - *j)
        }

        Ok(Vector::from(vec))
    }
}

impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a Vector<T>
    where T: Clone + Copy + Mul<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn mul(self, rhs: &'b Vector<T>) -> Self::Output {
        if self.dim() != rhs.dim() {
            return Err(format!("incompatible sizes"))
        }

        let mut vec = Vec::new();

        for (i, j) in self.value.iter().zip(rhs.value.iter()) {
            vec.push(*i * *j)
        }

        Ok(Vector::from(vec))
    }
}

impl<'a, 'b, T> Div<&'b Vector<T>> for &'a Vector<T>
    where T: Clone + Copy + Div<T, Output = T> {
    type Output = Result<Vector<T>, String>;

    fn div(self, rhs: &'b Vector<T>) -> Self::Output {
        if self.dim() != rhs.dim() {
            return Err(format!("incompatible sizes"))
        }

        let mut vec = Vec::new();

        for (i, j) in self.value.iter().zip(rhs.value.iter()) {
            vec.push(*i / *j)
        }

        Ok(Vector::from(vec))
    }
}

impl<'a, T> Mul<T> for &'a Vector<T>
    where T: Clone + Copy + Mul<T, Output = T> {
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut vec = Vec::new();

        for i in self.value.iter() {
            vec.push(*i * rhs)
        }

        Vector::from(vec)
    }
}

impl<'a, T> Div<T> for &'a Vector<T>
    where T: Clone + Copy + Div<T, Output = T> {
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut vec = Vec::new();

        for i in self.value.iter() {
            vec.push(*i / rhs)
        }

        Vector::from(vec)
    }
}