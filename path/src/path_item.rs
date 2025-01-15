use math::{CrossProduct, Tensor, Vector3};

use crate::{
    get_length::GetLength, get_t::GetT, lerp::Lerp, shift_in_plane::GetPosition,
    update_start_end::UpdateStartEnd, Curve, ShiftInPlane,
};

#[derive(Clone, Debug)]
pub enum PathItem<T> {
    Curve(Curve<T>),
}

impl<T> PathItem<T> {
    pub fn for_each_point(self, map: impl Fn(T) -> T) -> Self {
        match self {
            PathItem::Curve(curve) => Self::Curve(curve.for_each_point(map)),
        }
    }
}

impl<T> ShiftInPlane for PathItem<T>
where
    T: Tensor
        + Lerp<Scalar = <T as Tensor>::Scalar>
        + GetPosition<Position = Vector3<<T as Tensor>::Scalar>>,
    Vector3<<T as Tensor>::Scalar>:
        CrossProduct<Vector3<<T as Tensor>::Scalar>, Output = Vector3<<T as Tensor>::Scalar>>,
{
    type Vector3 = Vector3<<T as Tensor>::Scalar>;

    type Scalar = <T as Tensor>::Scalar;

    fn shift_in_plane(self, normal: Self::Vector3, amount: Self::Scalar) -> Self {
        match self {
            PathItem::Curve(curve) => Self::Curve(curve.shift_in_plane(normal, amount)),
        }
    }
}

impl<T> GetT for PathItem<T>
where
    T: Tensor,
{
    type Tensor = T;

    fn get_t(&self, t: <Self::Tensor as Tensor>::Scalar) -> Self::Tensor {
        match self {
            PathItem::Curve(curve) => curve.get_t(t),
        }
    }
}

impl<T> GetLength for PathItem<T>
where
    T: Tensor,
{
    type Tensor = T;

    fn get_length(&self) -> <Self::Tensor as Tensor>::Scalar {
        match self {
            PathItem::Curve(curve) => curve.get_length(),
        }
    }
}

impl<T: Tensor> UpdateStartEnd for PathItem<T> {
    type Tensor = T;

    fn update_start(&mut self, start: Self::Tensor) {
        match self {
            PathItem::Curve(curve) => curve.update_start(start),
        }
    }

    fn update_end(&mut self, end: Self::Tensor) {
        match self {
            PathItem::Curve(curve) => curve.update_end(end),
        }
    }
}
