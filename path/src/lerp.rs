use std::ops::{Add, Mul};

use math::{Scalar, Tensor};
use num_traits::One;

pub trait Lerp
where
    Self: Sized + Copy + Mul<Self::Scalar, Output = Self> + Add<Self, Output = Self>,
{
    type Scalar: Scalar;
    fn lerp(&self, rhs: &Self, t: Self::Scalar) -> Self {
        *self * (Self::Scalar::one() - t) + *rhs * t
    }
}

impl<T> Lerp for T
where
    T: Tensor,
{
    type Scalar = T::Scalar;
}
