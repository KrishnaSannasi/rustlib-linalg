extern crate rand;

pub mod vector_impl;
pub mod vector_ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>
    where T: Clone + Copy {
    value: Vec<T>

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
