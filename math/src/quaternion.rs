use std::ops::{Mul, MulAssign};

use num_traits::One;

use crate::Scalar;

use super::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Quaternion<T: Scalar> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: Scalar> Quaternion<T> {
    pub fn from_scaled_axis(axis: Vector3<T>) -> Self {
        let angle = axis.magnitude();
        if angle.is_zero() {
            Self::one()
        } else {
            let axis = axis.normalize();

            let half_angle = angle / T::from_value(2);
            let s = half_angle.sin();

            Self {
                x: axis.x * s,
                y: axis.y * s,
                z: axis.z * s,
                w: half_angle.cos(),
            }
        }
    }
}

impl<T: Scalar> Mul<Vector3<T>> for Quaternion<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        // quaternion q is assumed to have unit length

        let vx = rhs.x;
        let vy = rhs.y;
        let vz = rhs.z;
        let qx = self.x;
        let qy = self.y;
        let qz = self.z;
        let qw = self.w;

        // t = 2 * cross( q.xyz, v );
        let tx = T::two() * (qy * vz - qz * vy);
        let ty = T::two() * (qz * vx - qx * vz);
        let tz = T::two() * (qx * vy - qy * vx);

        // v + q.w * t + cross( q.xyz, t );
        Vector3 {
            x: vx + qw * tx + qy * tz - qz * ty,
            y: vy + qw * ty + qz * tx - qx * tz,
            z: vz + qw * tz + qx * ty - qy * tx,
        }
    }
}

impl<T: Scalar> MulAssign for Quaternion<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = rhs.x;
        let qby = rhs.y;
        let qbz = rhs.z;
        let qbw = rhs.w;

        self.x = qax * qbw + qaw * qbx + qay * qbz - qaz * qby;
        self.y = qay * qbw + qaw * qby + qaz * qbx - qax * qbz;
        self.z = qaz * qbw + qaw * qbz + qax * qby - qay * qbx;
        self.w = qaw * qbw - qax * qbx - qay * qby - qaz * qbz;
    }
}
impl<T: Scalar> Mul for Quaternion<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = rhs.x;
        let qby = rhs.y;
        let qbz = rhs.z;
        let qbw = rhs.w;

        Self {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }
}

impl<T: Scalar> Mul for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = rhs.x;
        let qby = rhs.y;
        let qbz = rhs.z;
        let qbw = rhs.w;

        Quaternion {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }
}

impl<T: Scalar> Mul<Quaternion<T>> for &Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: Quaternion<T>) -> Self::Output {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = rhs.x;
        let qby = rhs.y;
        let qbz = rhs.z;
        let qbw = rhs.w;

        Quaternion {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }
}
impl<T: Scalar> Mul<&Quaternion<T>> for Quaternion<T> {
    type Output = Quaternion<T>;

    fn mul(self, rhs: &Quaternion<T>) -> Self::Output {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = rhs.x;
        let qby = rhs.y;
        let qbz = rhs.z;
        let qbw = rhs.w;

        Quaternion {
            x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
            y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
            z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
            w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
        }
    }
}

impl<T> One for Quaternion<T>
where
    T: Scalar,
{
    fn one() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one(),
        }
    }
}
