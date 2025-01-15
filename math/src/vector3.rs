use core::fmt;
use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use num_traits::{One, Zero};

use crate::{cross::CrossProduct, Scalar, Tensor};

#[derive(Default, Clone, Copy, Eq, PartialEq, PartialOrd, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Sum for Vector3<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, i| a + i)
    }
}

impl<T: Scalar> Tensor for Vector3<T> {
    type Scalar = T;

    fn magnitude(&self) -> Self::Scalar {
        Vector3::magnitude(self)
    }
}

impl<T> CrossProduct for Vector3<T>
where
    T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
    type Output = Self;

    fn cross_product(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> fmt::Display for Vector3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T: Scalar> Vector3<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(&self) -> Vector3<T> {
        let m = T::one() / self.magnitude();
        *self * m
    }

    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    pub fn cross(&self, rhs: &Vector3<T>) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn lerp(&self, to: &Self, t: T) -> Self {
        *self * (T::one() - t) + *to * t
    }

    pub fn magnitude_squared(&self) -> T {
        self.dot(self)
    }

    pub fn x() -> Self {
        Self {
            x: One::one(),
            y: Zero::zero(),
            z: Zero::zero(),
        }
    }

    pub fn y() -> Self {
        Self {
            y: One::one(),
            x: Zero::zero(),
            z: Zero::zero(),
        }
    }

    pub fn z() -> Self {
        Self {
            z: One::one(),
            x: Zero::zero(),
            y: Zero::zero(),
        }
    }
}

impl<T> Zero for Vector3<T>
where
    T: Scalar,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T> Neg for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -T::one()
    }
}

impl<T> One for Vector3<T>
where
    T: Scalar,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: Scalar,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign for Vector3<T>
where
    T: Scalar,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Sub for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub for &Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub<&Vector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: &Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Sub<Vector3<T>> for &Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Scalar,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
