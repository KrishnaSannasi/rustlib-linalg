extern crate rand;
extern crate num;

pub mod vector;
#[cfg(feature = "compile-time-size")]
pub mod vector_sized;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! vectorize {
        [$($e: expr),*] => {
            Vector::from(vec![$($e),*])
        };
        [$e: expr;$c: expr] => {
            Vector::from(vec![$e;$c])
        }
    }
}
