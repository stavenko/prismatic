use core::fmt;
use std::ops::{AddAssign, Div, Mul, Sub, SubAssign};

use num_traits::{One, Zero};

use crate::scalar::Scalar;

pub trait Tensor:
    Mul<Self::Scalar, Output = Self>
    + Zero
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + One
    + Copy
    + fmt::Debug
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Div<Self::Scalar, Output = Self>
{
    type Scalar: Scalar;
    fn magnitude(&self) -> Self::Scalar;
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        *self / magnitude
    }
}
