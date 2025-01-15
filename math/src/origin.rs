use num_traits::One;

use crate::{Quaternion, Scalar, Vector3};

#[derive(Clone, Debug)]
pub struct BaseOrigin<F: Scalar> {
    pub center: Vector3<F>,
    pub rotation: Quaternion<F>,
}

impl<F> Default for BaseOrigin<F>
where
    F: Scalar,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F> BaseOrigin<F>
where
    F: Scalar,
{
    pub fn new() -> Self {
        Self {
            center: Vector3::new(F::zero(), F::zero(), F::zero()),
            rotation: Quaternion::one(),
        }
    }

    pub fn project(&self, v: Vector3<F>) -> Vector3<F> {
        let v = v - self.center;
        self.center + self.x() * v.dot(&self.x()) + self.y() * v.dot(&self.y())
    }

    pub fn project_unit(&self, v: Vector3<F>) -> Vector3<F> {
        (self.x() * v.dot(&self.x()) + self.y() * v.dot(&self.y())).normalize()
    }

    pub fn offset_z(mut self, amount: impl Into<F>) -> Self {
        self.center = self.z() * amount.into() + self.center;
        self
    }

    pub fn offset_x(mut self, amount: impl Into<F>) -> Self {
        self.center = self.x() * amount.into() + self.center;
        self
    }

    pub fn offset_y(mut self, amount: impl Into<F>) -> Self {
        self.center = self.y() * amount.into() + self.center;
        self
    }

    pub fn offset(mut self, axis: Vector3<F>) -> Self {
        self.center = axis + self.center;
        self
    }

    pub fn rotate(mut self, quat: Quaternion<F>) -> Self {
        self.rotation *= quat;
        self
    }

    pub fn rotate_axisangle(mut self, axisangle: Vector3<F>) -> Self {
        let quat = Quaternion::from_scaled_axis(axisangle);
        self.rotation *= quat;
        self
    }

    pub fn left(&self) -> Vector3<F> {
        -self.x()
    }

    pub fn right(&self) -> Vector3<F> {
        self.x()
    }

    pub fn top(&self) -> Vector3<F> {
        self.y()
    }

    pub fn x(&self) -> Vector3<F> {
        self.rotation * Vector3::x()
    }

    pub fn y(&self) -> Vector3<F> {
        self.rotation * Vector3::y()
    }

    pub fn z(&self) -> Vector3<F> {
        self.rotation * Vector3::z()
    }

    pub fn apply(&mut self, origin: &BaseOrigin<F>) {
        self.center = origin.rotation * (self.center) + origin.center;
        self.rotation = origin.rotation * self.rotation;
    }
}
