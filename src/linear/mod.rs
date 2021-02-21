//! Bivectors and Roters are not abstracted over their dimension because I don't know how
//! I will probably fix this in the future!

use crate::linear::scalar::Scalar;
use crate::numeric::float::Float;

pub mod bivec;
pub mod mat;
pub mod rotor;
pub mod scalar;
pub mod specific_impls;
pub mod vec;

pub fn from_float<T: Scalar + From<U>, U: Scalar + Float>(val: U) -> T {
    From::from(val)
}
