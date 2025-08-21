use std::f64::consts::PI;

use num_traits::{One, ToPrimitive};

use crate::{dot::Dot, CrossProduct, Quaternion, Scalar, Vector3};

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

    pub fn offset_z(mut self, amount: impl ToPrimitive) -> Self {
        self.center = self.z() * F::from(amount).unwrap() + self.center;
        self
    }

    pub fn offset_x(mut self, amount: impl ToPrimitive) -> Self {
        self.center = self.x() * F::from(amount).unwrap() + self.center;
        self
    }

    pub fn offset_y(mut self, amount: impl ToPrimitive) -> Self {
        self.center = self.y() * F::from(amount).unwrap() + self.center;
        self
    }

    pub fn rotate_z(self, amount: F) -> Self {
        self.rotate_axisangle(Vector3::z() * amount)
    }

    pub fn rotate_x(self, amount: F) -> Self {
        self.rotate_axisangle(Vector3::x() * amount)
    }

    pub fn rotate_y(self, amount: F) -> Self {
        self.rotate_axisangle(Vector3::y() * amount)
    }

    pub fn offset(self, axis: Vector3<F>) -> Self {
        self.offset_x(axis.x).offset_y(axis.y).offset_z(axis.z)
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

    pub fn apply_mut(&mut self, origin: &BaseOrigin<F>) {
        self.center = origin.rotation * (self.center) + origin.center;
        self.rotation = origin.rotation * self.rotation;
    }

    pub fn apply(mut self, origin: &BaseOrigin<F>) -> Self {
        self.center = origin.rotation * (self.center) + origin.center;
        self.rotation = origin.rotation * self.rotation;
        self
    }

    pub fn align_z<V>(mut self, normal: V) -> Self
    where
        V: CrossProduct<Vector3<F>, Output = Vector3<F>> + Dot<Vector3<F>, Output = F>,
    {
        let angle = normal.dot(&self.z()).acos();
        let angle_p = angle / F::pi();
        println!("Angle: {angle}, {angle_p}");
        if angle.is_zero() {
            self
        } else if angle_p.is_one() {
            let axisangle = self.x() * -angle;
            self.rotation = Quaternion::from_scaled_axis(axisangle) * self.rotation;
            self
        } else {
            let axisangle = normal.cross_product(&self.z()).normalize() * -angle;

            self.rotation = Quaternion::from_scaled_axis(axisangle) * self.rotation;
            self
        }
    }

    pub fn align_x<V>(mut self, normal: V) -> Self
    where
        V: CrossProduct<Vector3<F>, Output = Vector3<F>> + Dot<Vector3<F>, Output = F>,
    {
        let angle = normal.dot(&self.x()).acos();
        if angle.is_zero() {
            self
        } else {
            let axisangle = normal.cross_product(&self.x()).normalize() * -angle;
            self.rotation = Quaternion::from_scaled_axis(axisangle) * self.rotation;
            self
        }
    }
}
