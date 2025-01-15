use core::fmt;
use std::ops::{AddAssign, Div, Mul, Sub, SubAssign};

use num_traits::{Bounded, Float, One, Pow, Signed, Zero};

pub trait Scalar:
    Float
    + Zero
    + One
    + Bounded
    + Signed
    + Pow<usize, Output = Self>
    + Ord
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Copy
    + fmt::Debug
    + fmt::Display
    + Into<usize>
    + Into<f64>
    + From<f64>
    + From<usize>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + std::iter::Sum
    + PartialOrd<Self>
{
    fn two() -> Self {
        Self::one() + Self::one()
    }

    fn ten() -> Self {
        Self::from_value(10)
    }

    fn pi() -> Self {
        std::f64::consts::PI.into()
    }

    fn two_pi() -> Self {
        Self::pi() * Self::two()
    }

    fn half() -> Self {
        Self::one() / Self::two()
    }

    fn from_value<V: Into<Self>>(v: V) -> Self {
        v.into()
    }

    fn round_dp(&self, point: usize) -> Self {
        let pt = Self::ten().pow(point);
        let maxed = *self * pt;
        let rounded = maxed.round();
        rounded / pt
    }
}
