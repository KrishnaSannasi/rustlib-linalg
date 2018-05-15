use super::*;
use super::vector_impl::*;
use super::vector_ops::*;

#[test]
fn init() {
    let v = VectorI::new(2);

    assert_eq!(v.value, vec![0, 0])
}

#[test]
fn from_vec() {
    let v = VectorI::from(vec![1, 2, 3]);

    assert_eq!(v.value, vec![1, 2, 3])
}

#[test]
fn from_slice() {
    let a = [1, 2, 3, 4, 5, 6];
    let v = VectorI::from(&a[1..3]);

    assert_eq!(v.value, Vec::from(&a[1..3]))
}

#[test]
fn dot() {
    let v1 = VectorI::from(vec![1, 2]);
    let v2 = VectorI::from(vec![4, 9]);

    assert_eq!(v1.dot(&v2), 1 * 4 + 2 * 9)
}

#[test]
fn magsq() {
    let v1 = VectorD::from(vec![1.0, 2.0]);

    assert_eq!(v1.magsq(), (1.0 * 1.0 + 2.0 * 2.0f64))
}

#[test]
fn mag() {
    let v1 = VectorD::from(vec![1.0, 2.0]);

    assert_eq!(v1.mag(), (1.0 * 1.0 + 2.0 * 2.0f64).sqrt())
}
