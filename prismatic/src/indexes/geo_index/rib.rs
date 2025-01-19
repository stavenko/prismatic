use std::{fmt, ops::Deref};

use crate::indexes::vertex_index::PtId;
use math::{Scalar, Vector3};

use super::{
    geo_object::{GeoObject, UnRef},
    index::GeoIndex,
    seg::SegmentDir,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct RibId(pub(super) usize);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rib(pub(super) PtId, pub(super) PtId);

impl Rib {
    pub(crate) fn build(from: PtId, to: PtId) -> (Self, SegmentDir) {
        if from > to {
            (Rib(to, from), SegmentDir::Rev)
        } else {
            (Rib(from, to), SegmentDir::Fow)
        }
    }
}

pub struct RibRef<'a, S: Scalar> {
    pub(crate) index: &'a GeoIndex<S>,
    pub(super) rib_id: RibId,
}

impl<S: Scalar> fmt::Display for RibRef<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?} -> {:?}",
            self.rib_id,
            self.from_pt(),
            self.to_pt(),
        )
    }
}

#[allow(dead_code)]
pub struct RibRefMut<'a, S: Scalar> {
    pub(crate) index: &'a mut GeoIndex<S>,
    pub(super) rib_id: RibId,
}

impl<S: Scalar> RibRef<'_, S> {
    pub(crate) fn from(&self) -> Vector3<S> {
        self.index
            .vertices
            .get_point(self.index.ribs[&self.rib_id].0)
    }

    pub(crate) fn middle(&self) -> Vector3<S> {
        self.from().lerp(&self.to(), S::half())
    }

    pub(crate) fn to(&self) -> Vector3<S> {
        self.index
            .vertices
            .get_point(self.index.ribs[&self.rib_id].1)
    }

    pub(crate) fn dir(&self) -> Vector3<S> {
        self.to() - self.from()
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_pt(&self) -> PtId {
        self.index.ribs[&self.rib_id].0
    }

    pub(crate) fn to_pt(&self) -> PtId {
        self.index.ribs[&self.rib_id].1
    }

    pub(crate) fn magnitude(&self) -> S {
        (self.from() - self.to()).magnitude()
    }
}

impl<S: Scalar> Deref for RibRef<'_, S> {
    type Target = RibId;

    fn deref(&self) -> &Self::Target {
        &self.rib_id
    }
}

impl fmt::Debug for RibId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RibId:{}", self.0)
    }
}

impl PartialEq<usize> for RibId {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl<'a, S: Scalar> UnRef<'a, S> for RibRef<'a, S> {
    type Obj = RibId;

    fn un_ref(self) -> Self::Obj {
        self.rib_id
    }
}

impl<'a, S: Scalar> UnRef<'a, S> for RibRefMut<'a, S> {
    type Obj = RibId;

    fn un_ref(self) -> Self::Obj {
        self.rib_id
    }
}

impl<'a, S: Scalar + 'a> GeoObject<'a, S> for RibId {
    type Ref = RibRef<'a, S>;

    type MutRef = RibRefMut<'a, S>;

    fn make_ref(&self, index: &'a GeoIndex<S>) -> Self::Ref {
        RibRef {
            index,
            rib_id: *self,
        }
    }

    fn make_mut_ref(&self, index: &'a mut GeoIndex<S>) -> Self::MutRef {
        RibRefMut {
            index,
            rib_id: *self,
        }
    }
}
