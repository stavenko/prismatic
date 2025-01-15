use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use num_traits::{One, Zero};

use crate::Scalar;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<[T; 2]> for Vector2<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T: Scalar> Sum for Vector2<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut acc = Self::zero();
        for item in iter {
            acc += item
        }
        acc
    }
}

impl<T> Vector2<T>
where
    T: Scalar,
{
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn magnitude_squared(&self) -> T {
        self.dot(self)
    }

    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn lerp(&self, to: &Self, t: T) -> Self {
        *self * (T::one() - t) + *to * t
    }
}

impl<T> Neg for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -T::one()
    }
}

impl<T> Zero for Vector2<T>
where
    T: Scalar,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T> One for Vector2<T>
where
    T: Scalar,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }
}

impl<T> Add for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vector2<T>
where
    T: Scalar,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign for Vector2<T>
where
    T: Scalar,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Sub for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Scalar,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
