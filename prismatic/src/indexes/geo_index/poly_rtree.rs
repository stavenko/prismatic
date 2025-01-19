use rstar::{Point, RTreeObject, AABB};

use crate::indexes::aabb::Aabb;
use math::{Scalar, Vector3};

use super::face::FaceId;

impl<S> From<Vector3<S>> for RtreePt<S> {
    fn from(value: Vector3<S>) -> Self {
        Self([value.x, value.y, value.z])
    }
}

impl<S: Scalar> Point for RtreePt<S> {
    type Scalar = S;

    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        RtreePt([generator(0), generator(1), generator(2)])
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        self.0[index]
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        &mut self.0[index]
    }
}

impl<S: Scalar> RTreeObject for FaceRtreeRecord<S> {
    type Envelope = AABB<RtreePt<S>>;

    fn envelope(&self) -> Self::Envelope {
        self.1.into()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RtreePt<S>([S; 3]);

#[derive(Debug, PartialEq)]
pub struct FaceRtreeRecord<S>(pub(super) FaceId, pub(super) Aabb<S>);
