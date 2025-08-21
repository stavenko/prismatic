use core::fmt;
use std::ops::{AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use num_traits::{AsPrimitive, Bounded, Float, FromPrimitive, One, Pow, Signed, ToPrimitive, Zero};

pub trait Scalar:
    Float
    + Zero
    + One
    + Default
    + Bounded
    + Signed
    + Pow<i32, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Copy
    + FromPrimitive
    + ToPrimitive
    + fmt::Debug
    + fmt::Display
    + Mul<Self, Output = Self>
    + MulAssign<Self>
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
        Self::from_f64(std::f64::consts::PI).expect("Implement `FromPrimitive` trait")
    }

    fn two_pi() -> Self {
        Self::pi() * Self::two()
    }

    fn half() -> Self {
        Self::one() / Self::two()
    }

    fn from_value<V: AsPrimitive<f64>>(v: V) -> Self {
        let v: f64 = v.as_();
        Self::from_f64(v).expect("Cannot build float number")
    }

    fn round_dp(&self, point: i32) -> Self {
        let pt = Self::ten().pow(point);
        let maxed = *self * pt;
        let rounded = maxed.round();
        rounded / pt
    }
}

impl Scalar for f64 {}
impl Scalar for f32 {}
