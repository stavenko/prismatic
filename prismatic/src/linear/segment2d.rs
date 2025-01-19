use core::fmt;

use crate::reversable::Reversable;
use math::{Scalar, Vector2};

#[derive(Clone)]
pub struct Segment2D<S> {
    pub from: Vector2<S>,
    pub to: Vector2<S>,
}

impl<S: Scalar> PartialEq for Segment2D<S> {
    fn eq(&self, other: &Self) -> bool {
        let fd = self.from - other.from;
        let td = self.to - other.to;

        let fd = fd.magnitude_squared().round_dp(14);
        let td = td.magnitude_squared().round_dp(14);

        fd == S::zero() && td == S::zero()
    }
}

impl<S: Scalar> Reversable for Segment2D<S> {
    fn flip(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }
}

impl<S: Scalar> fmt::Debug for Segment2D<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} -> {}, {}",
            self.from.x.round_dp(4),
            self.from.y.round_dp(4),
            self.to.x.round_dp(4),
            self.to.y.round_dp(4)
        )
    }
}

impl<S: Scalar> Segment2D<S> {
    pub fn remove_opposites(mut segments: Vec<Self>) -> Vec<Self> {
        let mut joined = Vec::new();
        while let Some(left) = segments.pop() {
            let inv = left.clone().flip();
            match segments.iter().position(|segment| *segment == inv) {
                None => joined.push(left),
                Some(ix) => {
                    segments.swap_remove(ix);
                }
            }
        }
        joined
    }

    pub fn new(from: Vector2<S>, to: Vector2<S>) -> Self {
        Self { from, to }
    }

    pub fn dir(&self) -> Vector2<S> {
        self.to - self.from
    }
}
