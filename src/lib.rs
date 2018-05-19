extern crate rand;
extern crate num;

pub mod vector;

#[macro_export]
macro_rules! vectorize {
    [$($e: expr),*] => {
        Vector::from(vec![$($e),*])
    };
}
