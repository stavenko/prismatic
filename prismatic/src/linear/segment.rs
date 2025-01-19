use std::fmt;

use math::{Scalar, Vector3};

use super::{line::Line, ray::Ray};

#[derive(Clone)]
pub struct Segment<S> {
    pub from: Vector3<S>,
    pub to: Vector3<S>,
}

impl<S: Scalar> From<Segment<S>> for Line<S> {
    fn from(value: Segment<S>) -> Self {
        Self {
            origin: value.from,
            dir: value.dir().normalize(),
        }
    }
}

impl<S: Scalar> fmt::Debug for Segment<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} -> {} {} {}",
            self.from.x.round_dp(4),
            self.from.y.round_dp(4),
            self.from.z.round_dp(4),
            self.to.x.round_dp(4),
            self.to.y.round_dp(4),
            self.to.z.round_dp(4)
        )
    }
}

impl<S: Scalar> PartialEq for Segment<S> {
    fn eq(&self, other: &Self) -> bool {
        let fd = self.from - other.from;
        let fd = fd.magnitude_squared().round_dp(14);
        if fd.is_zero() {
            let td = self.to - other.to;

            let td = td.magnitude_squared().round_dp(14);
            td.is_zero()
        } else {
            false
        }
    }
}

impl<S: Scalar> Segment<S> {
    pub fn has(&self, point: Vector3<S>) -> bool {
        let d = self.from - point;
        let q = self.to - point;
        d.magnitude_squared().round_dp(14 - 5).abs().is_zero()
            || q.magnitude_squared().round_dp(14 - 5).abs().is_zero()
    }
    pub fn new(from: Vector3<S>, to: Vector3<S>) -> Self {
        Self { from, to }
    }

    pub(crate) fn flip(self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }

    pub fn get_ray(&self) -> Ray<S> {
        Ray {
            origin: self.from,
            dir: self.dir().normalize(),
        }
    }

    pub fn get_line(&self) -> Line<S> {
        Line {
            origin: self.from,
            dir: self.dir().normalize(),
        }
    }

    pub(crate) fn dir(&self) -> Vector3<S> {
        self.to - self.from
    }

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
}
