use num_traits::{One, Signed, Zero};

use crate::{
    decimal::{Dec, STABILITY_ROUNDING},
    indexes::{
        geo_index::{rib::RibRef, seg::SegRef},
        vertex_index::PtId,
    },
    linear::{line::Line, ray::Ray, segment::Segment},
};
use math::Vector3;

use super::relation::Relation;

#[derive(PartialEq, Debug)]
pub enum PointOnLine {
    On,
    Outside,
    Origin,
}

impl Relation<Vector3<Dec>> for Line {
    type Relate = PointOnLine;

    fn relate(&self, to: &Vector3<Dec>) -> Self::Relate {
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

impl Relation<Vector3<Dec>> for Ray {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<Dec>) -> Self::Relate {
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

impl Relation<Vector3<Dec>> for Segment {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<Dec>) -> Self::Relate {
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
            if t0.is_negative() || t0 > Dec::one() {
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

impl<'a> Relation<Vector3<Dec>> for SegRef<'a> {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<Dec>) -> Self::Relate {
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
            if t0.is_negative() || t0 > Dec::one() {
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

impl<'a> Relation<PtId> for RibRef<'a> {
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
                if t0.is_negative() || t0 > Dec::one() {
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
impl<'a> Relation<Vector3<Dec>> for RibRef<'a> {
    type Relate = PointOnLine;
    fn relate(&self, to: &Vector3<Dec>) -> Self::Relate {
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
            if t0.is_negative() || t0 > Dec::one() {
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
