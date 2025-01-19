use crate::{
    constants::STABILITY_ROUNDING,
    indexes::{
        geo_index::{rib::RibRef, seg::SegRef},
        vertex_index::PtId,
    },
    linear::{line::Line, ray::Ray, segment::Segment},
};
use math::{Scalar, Vector3};

use super::relation::Relation;

#[derive(PartialEq, Debug)]
pub enum PointOnLine {
    On,
    Outside,
    Origin,
}

impl<S: Scalar> Relation<Vector3<S>> for Line<S> {
    type Relate = PointOnLine;

    fn relate(&self, to: &Vector3<S>) -> Self::Relate {
        let q = to - self.origin;
        let t0 = self.dir.dot(&q);
        let maybe_to = self.origin + self.dir * t0;

        if (to - self.origin)
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            PointOnLine::Origin
        } else if (to - maybe_to)
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            PointOnLine::On
        } else {
            PointOnLine::Outside
        }
    }
}

impl<S: Scalar> Relation<Vector3<S>> for Ray<S> {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<S>) -> Self::Relate {
        let q = to - self.origin;
        let t0 = self.dir.dot(&q).round_dp(STABILITY_ROUNDING);
        let maybe_to = self.origin + self.dir * t0;
        if (to - maybe_to)
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            if t0.is_negative() {
                PointOnLine::Outside
            } else if t0.is_zero() {
                PointOnLine::Origin
            } else {
                PointOnLine::On
            }
        } else {
            PointOnLine::Outside
        }
    }
}

impl<S: Scalar> Relation<Vector3<S>> for Segment<S> {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<S>) -> Self::Relate {
        let q = to - self.from;
        let dir = self.dir();
        let len = dir.magnitude();
        let t0 = (self.dir().normalize().dot(&q) / len).round_dp(STABILITY_ROUNDING - 5);
        let maybe_to = self.from + self.dir() * t0;

        if (to - maybe_to)
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            if t0.is_negative() || t0 > S::one() {
                PointOnLine::Outside
            } else if t0.is_zero() || t0.is_one() {
                PointOnLine::Origin
            } else {
                PointOnLine::On
            }
        } else {
            PointOnLine::Outside
        }
    }
}

impl<S: Scalar> Relation<Vector3<S>> for SegRef<'_, S> {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<S>) -> Self::Relate {
        let q = to - self.from();
        let dir = self.dir();
        let len = dir.magnitude();
        let t0 = (self.dir().normalize().dot(&q) / len).round_dp(STABILITY_ROUNDING - 5);
        let maybe_to = self.from() + self.dir() * t0;

        if (to - maybe_to)
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            if t0.is_negative() || t0 > S::one() {
                PointOnLine::Outside
            } else if t0.is_zero() || t0.is_one() {
                PointOnLine::Origin
            } else {
                PointOnLine::On
            }
        } else {
            PointOnLine::Outside
        }
    }
}

impl<S: Scalar> Relation<PtId> for RibRef<'_, S> {
    type Relate = PointOnLine;
    fn relate(&self, to: &PtId) -> Self::Relate {
        if *to == self.to_pt() || *to == self.from_pt() {
            PointOnLine::Origin
        } else {
            let to = self.index.vertices.get_point(*to);
            let q = to - self.from();
            let dir = self.dir();
            let len = dir.magnitude();
            let t0 = (self.dir().normalize().dot(&q) / len).round_dp(STABILITY_ROUNDING - 5);
            let maybe_to = self.from() + self.dir() * t0;

            if (to - maybe_to)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                if t0.is_negative() || t0 > S::one() {
                    PointOnLine::Outside
                } else if t0.is_zero() || t0.is_one() {
                    panic!("not possible");
                } else {
                    PointOnLine::On
                }
            } else {
                PointOnLine::Outside
            }
        }
    }
}
impl<S: Scalar> Relation<Vector3<S>> for RibRef<'_, S> {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<S>) -> Self::Relate {
        let q = to - self.from();
        let dir = self.dir();
        let len = dir.magnitude();
        let t0 = (self.dir().normalize().dot(&q) / len).round_dp(STABILITY_ROUNDING - 5);
        let maybe_to = self.from() + self.dir() * t0;

        if (to - maybe_to)
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            if t0.is_negative() || t0 > S::one() {
                PointOnLine::Outside
            } else if t0.is_zero() || t0.is_one() {
                PointOnLine::Origin
            } else {
                PointOnLine::On
            }
        } else {
            PointOnLine::Outside
        }
    }
}
