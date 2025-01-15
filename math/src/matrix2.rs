use std::ops::Mul;

use crate::Scalar;

use super::Vector2;

pub struct Matrix2<T> {
    inner: [[T; 2]; 2],
}

impl<T: Scalar> Matrix2<T> {
    pub fn new(m11: T, m12: T, m21: T, m22: T) -> Self {
        Self {
            inner: [[m11, m12], [m21, m22]],
        }
    }

    pub fn determinant(&self) -> T {
        let [[m11, m12], [m21, m22]] = self.inner;

        m11 * m22 - m21 * m12
    }
    pub fn try_inverse_mut(&mut self) -> Option<()> {
        let [[m11, m12], [m21, m22]] = self.inner;

        let determinant = m11 * m22 - m21 * m12;

        if determinant.is_zero() {
            None
        } else {
            self.inner[0][0] = m22 / determinant;
            self.inner[0][1] = -m12 / determinant;

            self.inner[1][0] = -m21 / determinant;
            self.inner[1][1] = m11 / determinant;

            Some(())
        }
    }

    pub fn try_inverse(mut self) -> Option<Self> {
        self.try_inverse_mut()?;
        Some(self)
    }
}

impl<T: Scalar> Mul<Vector2<T>> for Matrix2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        let col1 = Vector2::from(self.inner[0]);
        let col2 = Vector2::from(self.inner[1]);
        Vector2::new(rhs.dot(&col1), rhs.dot(&col2))
    }
}
