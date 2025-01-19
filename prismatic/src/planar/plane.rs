use std::fmt;

use math::{Scalar, Vector3};

use crate::indexes::quadtree::STABILITY_ROUNDING;

#[derive(Clone, Eq, PartialOrd)]
pub struct Plane<S: Scalar> {
    normal: Vector3<S>,
    d: S,
}

impl<S: Scalar> PartialEq for Plane<S> {
    fn eq(&self, other: &Self) -> bool {
        (self.d - other.d).round_dp(STABILITY_ROUNDING).is_zero()
            && self
                .normal
                .dot(&other.normal)
                .round_dp(STABILITY_ROUNDING)
                .is_one()
    }
}

impl<S: Scalar> fmt::Debug for Plane<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}x  {}y {}z {}",
            self.normal.x.round_dp(STABILITY_ROUNDING),
            self.normal.y.round_dp(STABILITY_ROUNDING),
            self.normal.z.round_dp(STABILITY_ROUNDING),
            self.d.round_dp(STABILITY_ROUNDING)
        )
    }
}

impl<S: Scalar> Plane<S> {
    pub fn new(a: S, b: S, c: S, d: S) -> Self {
        Self {
            normal: Vector3::new(a, b, c).normalize(),
            d,
        }
    }

    pub fn flip(&mut self) {
        self.normal = -self.normal;
        self.d = -self.d;
    }

    pub fn is_point_on_plane(&self, point: Vector3<S>, tolerance: impl Into<S>) -> bool {
        let p = self.normal.dot(&point) - self.d;
        p.abs() < tolerance.into()
    }

    pub fn get_intersection_param(&self, from: Vector3<S>, to: Vector3<S>) -> Option<S> {
        let from_p = self.normal.dot(&from) - self.d;
        let to_p = self.normal.dot(&to) - self.d;
        if (from_p * to_p).is_negative() {
            let total = from_p.abs() + to_p.abs();
            Some((from_p / total).abs())
        } else {
            None
        }
    }
    pub fn get_intersection_param2(&self, from: Vector3<S>, to: Vector3<S>) -> Option<S> {
        let from_p = self.normal.dot(&from) - self.d;
        let to_p = self.normal.dot(&to) - self.d;
        let sum = from_p - to_p;
        if sum.is_zero() {
            None
        } else {
            Some(from_p / sum)
        }
    }

    pub fn new_from_normal_and_point(normal: Vector3<S>, point: Vector3<S>) -> Self {
        let d = normal.dot(&point);

        Self { normal, d }
    }

    pub fn normal(&self) -> Vector3<S> {
        self.normal
    }

    pub fn d(&self) -> S {
        self.d
    }

    pub fn point_on_plane(&self) -> Vector3<S> {
        self.normal * self.d
    }

    pub(crate) fn flipped(mut self) -> Plane<S> {
        self.normal = -self.normal;
        self.d = -self.d;
        self
    }
}
