use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use math::{Scalar, Tensor};
use num_traits::{One, Zero};
use path::GetPosition;
use surface::EdgeTensor;

use math::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct SuperPoint<T: Scalar> {
    pub side_dir: Vector3<T>,
    pub point: Vector3<T>,
}

impl<T> GetPosition for SuperPoint<T>
where
    T: Scalar,
{
    type Position = Vector3<T>;

    fn get_position(&self) -> Self::Position {
        self.point
    }

    fn get_position_mut(&mut self) -> &mut Self::Position {
        &mut self.point
    }
}

impl<T: Scalar + 'static> EdgeTensor for SuperPoint<T> {
    type Vector = Vector3<T>;

    fn get_point(&self) -> Self::Vector {
        self.point
    }

    fn get_edge_dir(&self) -> Self::Vector {
        self.side_dir
    }
}

impl<T: Scalar + 'static> Tensor for SuperPoint<T> {
    type Scalar = T;

    fn magnitude(&self) -> <Self as Tensor>::Scalar {
        let dot = self.point.x.powi(2) + self.point.y.powi(2) + self.point.z.powi(2);
        dot.sqrt()
    }
}

impl<T: Scalar> Div for SuperPoint<T>
where
    Vector3<T>: Div<Vector3<T>, Output = Vector3<T>>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir / rhs.side_dir,
            point: self.point / rhs.point,
        }
    }
}

impl<T: Scalar> Div<T> for SuperPoint<T>
where
    Vector3<T>: Div<T, Output = Vector3<T>>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            side_dir: self.side_dir / rhs,
            point: self.point / rhs,
        }
    }
}

impl<T: Scalar> Mul<T> for SuperPoint<T>
where
    Vector3<T>: Mul<T, Output = Vector3<T>>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            side_dir: self.side_dir * rhs,
            point: self.point * rhs,
        }
    }
}

impl<T: Scalar> Mul for SuperPoint<T>
where
    Vector3<T>: Mul<Vector3<T>, Output = Vector3<T>>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir * rhs.side_dir,
            point: self.point * rhs.point,
        }
    }
}

impl<T: Scalar> MulAssign for SuperPoint<T>
where
    Vector3<T>: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.point *= rhs.point;
        self.side_dir *= rhs.side_dir;
    }
}
impl<T: Scalar> AddAssign for SuperPoint<T>
where
    Vector3<T>: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.point += rhs.point;
        self.side_dir += rhs.side_dir;
    }
}

impl<T: Scalar> SubAssign for SuperPoint<T>
where
    Vector3<T>: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.point -= rhs.point;
        self.side_dir -= rhs.side_dir;
    }
}

impl<T: Scalar + 'static> Add<Self> for SuperPoint<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir + rhs.side_dir,
            point: self.point + rhs.point,
        }
    }
}
impl<T: Scalar + 'static> Sub<Self> for SuperPoint<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir - rhs.side_dir,
            point: self.point - rhs.point,
        }
    }
}

impl<T: Scalar + 'static> Zero for SuperPoint<T> {
    fn zero() -> Self {
        Self {
            side_dir: Vector3::zero(),
            point: Vector3::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.point.is_zero() && self.side_dir.is_zero()
    }
}

impl<T: Scalar> One for SuperPoint<T> {
    fn one() -> Self {
        Self {
            side_dir: Vector3::one(),
            point: Vector3::one(),
        }
    }
}
