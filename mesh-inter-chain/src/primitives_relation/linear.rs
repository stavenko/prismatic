use crate::{
    indexes::{geo_index::seg::SegRef, quadtree::STABILITY_ROUNDING},
    linear::{line::Line, ray::Ray, segment::Segment},
};
use math::{Matrix2, Scalar, Vector2, Vector3};

use super::relation::Relation;

pub const NORMAL_DOT_ROUNDING: usize = 4;

#[derive(PartialEq, Debug)]
pub enum LinearRelation<S> {
    Parallell,
    Crossed { this: Vector3<S>, to: Vector3<S> },
    Colinear,
    Opposite,
    Intersect(LinearIntersection<S>),
    Independent,
}

#[derive(PartialEq, Debug)]
pub enum LinearRefRelation<S> {
    Parallell,
    Crossed { this: Vector3<S>, to: Vector3<S> },
    Colinear,
    Opposite,
    Intersect(LinearRefIntersection<S>),
    Independent,
}

#[derive(PartialEq, Debug)]
pub enum LinearIntersection<S> {
    In(Vector3<S>),
    Origin(Vector3<S>),
}

#[derive(PartialEq, Debug)]
pub enum LinearRefIntersection<S> {
    In(S, S),
    One,
    Zero,
}

impl<S: Scalar> Relation<Line<S>> for Line<S> {
    type Relate = LinearRelation<S>;
    fn relate(&self, to: &Self) -> Self::Relate {
        let dot = self.dir.dot(&to.dir);
        let q = self.origin - to.origin;
        if dot.round_dp(STABILITY_ROUNDING).abs().is_one() {
            let magnitude_squared = q.magnitude_squared();
            let dot = q.dot(&self.dir);
            if (dot - magnitude_squared)
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                return LinearRelation::Colinear;
            }
            if (dot + magnitude_squared)
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                return LinearRelation::Opposite;
            }
        }

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&to.dir));

        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.origin + self.dir * st.x;
            let p2 = to.origin + to.dir * st.y;
            if (p1 - p2)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                LinearRelation::Intersect(LinearIntersection::In(p1))
            } else {
                LinearRelation::Crossed { this: p1, to: p2 }
            }
        } else {
            LinearRelation::Parallell
        }
    }
}

impl<S: Scalar> Relation<Ray<S>> for Line<S> {
    type Relate = LinearRelation<S>;
    fn relate(&self, to: &Ray<S>) -> Self::Relate {
        let dot = self.dir.dot(&to.dir);
        let q = self.origin - to.origin;
        if dot.round_dp(STABILITY_ROUNDING).abs().is_one() {
            let magnitude_squared = q.magnitude_squared();
            let dot = q.dot(&self.dir);
            if (dot - magnitude_squared)
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                return LinearRelation::Colinear;
            }
            if (dot + magnitude_squared)
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                return LinearRelation::Opposite;
            }
        }

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&to.dir));

        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.origin + self.dir * st.x;
            let p2 = to.origin + to.dir * st.y;
            if (p1 - p2)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                if st.y.is_negative() {
                    LinearRelation::Independent
                } else {
                    LinearRelation::Intersect(LinearIntersection::In(p1))
                }
            } else {
                LinearRelation::Crossed { this: p1, to: p2 }
            }
        } else {
            LinearRelation::Parallell
        }
    }
}

impl<'a, S: Scalar> Relation<SegRef<'a, S>> for Line<S> {
    type Relate = LinearRefRelation<S>;
    fn relate(&self, to: &SegRef<'a, S>) -> Self::Relate {
        let segment_dir = to.dir().normalize();
        let dot = (self.dir.dot(&segment_dir)).round_dp(STABILITY_ROUNDING - 1);
        let q = self.origin - to.from();

        if dot.abs().is_one() {
            let magnitude_squared = q.magnitude_squared().round_dp(NORMAL_DOT_ROUNDING).sqrt();
            let point_dot = q.dot(&self.dir).abs().round_dp(NORMAL_DOT_ROUNDING);
            if (point_dot - magnitude_squared)
                .round_dp(NORMAL_DOT_ROUNDING)
                .is_zero()
            {
                return if dot.is_positive() {
                    LinearRefRelation::Colinear
                } else {
                    LinearRefRelation::Opposite
                };
            }
        }
        let dot = self.dir.dot(&segment_dir).round_dp(STABILITY_ROUNDING);
        //dbg!(dot.round_dp(STABILITY_ROUNDING));

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&segment_dir));

        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.origin + self.dir * st.x;
            let p2 = to.from() + segment_dir * st.y;
            if (p1 - p2)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                let segment_len = to.dir().magnitude().round_dp(NORMAL_DOT_ROUNDING);
                let y = (st.y / segment_len).round_dp(NORMAL_DOT_ROUNDING);

                if y.is_negative() || y > S::one() {
                    LinearRefRelation::Independent
                } else if y.is_zero() {
                    LinearRefRelation::Intersect(LinearRefIntersection::Zero)
                } else if y.is_one() {
                    LinearRefRelation::Intersect(LinearRefIntersection::One)
                } else {
                    LinearRefRelation::Intersect(LinearRefIntersection::In(st.x, st.y))
                }
            } else {
                LinearRefRelation::Crossed { this: p1, to: p2 }
            }
        } else {
            LinearRefRelation::Parallell
        }
    }
}

impl<S: Scalar> Relation<Segment<S>> for Line<S> {
    type Relate = LinearRelation<S>;
    fn relate(&self, to: &Segment<S>) -> Self::Relate {
        let segment_dir = to.dir().normalize();
        let dot = (self.dir.dot(&segment_dir)).round_dp(STABILITY_ROUNDING - 1);
        let q = self.origin - to.from;

        if dot.abs().is_one() {
            let magnitude_squared = q.magnitude_squared().round_dp(NORMAL_DOT_ROUNDING).sqrt();
            let point_dot = q.dot(&self.dir).abs().round_dp(NORMAL_DOT_ROUNDING);
            if (point_dot - magnitude_squared)
                .round_dp(NORMAL_DOT_ROUNDING)
                .is_zero()
            {
                return if dot.is_positive() {
                    LinearRelation::Colinear
                } else {
                    LinearRelation::Opposite
                };
            }
        }
        let dot = self.dir.dot(&segment_dir).round_dp(STABILITY_ROUNDING);
        //dbg!(dot.round_dp(STABILITY_ROUNDING));

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&segment_dir));

        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.origin + self.dir * st.x;
            let p2 = to.from + segment_dir * st.y;
            if (p1 - p2)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                let segment_len = to.dir().magnitude().round_dp(NORMAL_DOT_ROUNDING);
                let y = (st.y / segment_len).round_dp(NORMAL_DOT_ROUNDING);

                if y.is_negative() || y > S::one() {
                    LinearRelation::Independent
                } else if y.is_zero() || y.is_one() {
                    LinearRelation::Intersect(LinearIntersection::Origin(p2))
                } else {
                    LinearRelation::Intersect(LinearIntersection::In(p1))
                }
            } else {
                LinearRelation::Crossed { this: p1, to: p2 }
            }
        } else {
            LinearRelation::Parallell
        }
    }
}

impl<S: Scalar> Relation<Segment<S>> for Ray<S> {
    type Relate = LinearRelation<S>;
    fn relate(&self, to: &Segment<S>) -> Self::Relate {
        let segment_dir = to.dir().normalize();
        let dot = (self.dir.dot(&segment_dir)).round_dp(STABILITY_ROUNDING - 1);
        let q = self.origin - to.from;
        if dot.abs().is_one() {
            let magnitude_squared = q.magnitude();
            let point_dot = q.dot(&self.dir).abs();
            if (point_dot - magnitude_squared)
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                return if dot.is_positive() {
                    LinearRelation::Colinear
                } else {
                    LinearRelation::Opposite
                };
            }
        }

        let dot = self.dir.dot(&segment_dir).round_dp(STABILITY_ROUNDING - 1);

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&segment_dir));

        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.origin + self.dir * st.x;
            let p2 = to.from + segment_dir * st.y;
            if (p1 - p2)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                let segment_len = to.dir().magnitude().round_dp(STABILITY_ROUNDING);
                let y = (st.y / segment_len).round_dp(STABILITY_ROUNDING - 3);

                if y.is_negative() || y > S::one() || st.x.is_negative() {
                    LinearRelation::Independent
                } else if y.is_zero() || y.is_one() {
                    LinearRelation::Intersect(LinearIntersection::Origin(p2))
                } else {
                    LinearRelation::Intersect(LinearIntersection::In(p1))
                }
            } else {
                LinearRelation::Crossed { this: p1, to: p2 }
            }
        } else {
            LinearRelation::Parallell
        }
    }
}

impl<'a, S: Scalar> Relation<SegRef<'a, S>> for Ray<S> {
    type Relate = LinearRefRelation<S>;
    fn relate(&self, to: &SegRef<'a, S>) -> Self::Relate {
        let segment_dir = to.dir().normalize();
        let dot = (self.dir.dot(&segment_dir)).round_dp(STABILITY_ROUNDING - 1);
        let q = self.origin - to.from();
        if dot.abs().is_one() {
            let magnitude_squared = q.magnitude();
            let point_dot = q.dot(&self.dir).abs();
            if (point_dot - magnitude_squared)
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                return if dot.is_positive() {
                    LinearRefRelation::Colinear
                } else {
                    LinearRefRelation::Opposite
                };
            }
        }

        let dot = self.dir.dot(&segment_dir).round_dp(STABILITY_ROUNDING - 1);

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&segment_dir));

        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.origin + self.dir * st.x;
            let p2 = to.from() + segment_dir * st.y;
            if (p1 - p2)
                .magnitude_squared()
                .round_dp(STABILITY_ROUNDING)
                .is_zero()
            {
                let segment_len = to.dir().magnitude().round_dp(STABILITY_ROUNDING);
                let y = (st.y / segment_len).round_dp(STABILITY_ROUNDING - 3);

                if y.is_negative() || y > S::one() || st.x.is_negative() {
                    LinearRefRelation::Independent
                } else if y.is_zero() {
                    LinearRefRelation::Intersect(LinearRefIntersection::Zero)
                } else if y.is_one() {
                    LinearRefRelation::Intersect(LinearRefIntersection::One)
                } else {
                    LinearRefRelation::Intersect(LinearRefIntersection::In(st.x, y))
                }
            } else {
                LinearRefRelation::Crossed { this: p1, to: p2 }
            }
        } else {
            LinearRefRelation::Parallell
        }
    }
}
#[cfg(test)]
mod tests {
    use std::ops::Neg;

    use num_traits::One;

    use crate::{
        linear::{line::Line, segment::Segment},
        primitives_relation::{
            linear::{LinearIntersection, LinearRelation},
            relation::Relation,
        },
    };
    use math::{Dec, Vector3};

    #[test]
    fn segment_line_relation() {
        let segment: Segment<Dec> = Segment {
            from: Vector3::new(Dec::one(), Dec::one(), Dec::one()),
            to: Vector3::new(Dec::one(), Dec::one(), Dec::one().neg()),
        };

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one(), Dec::one()),
            dir: -Vector3::z(),
        };

        assert_eq!(line.relate(&segment), LinearRelation::Colinear);

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one(), Dec::one()),
            dir: Vector3::z(),
        };

        assert_eq!(line.relate(&segment), LinearRelation::Opposite);

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one(), Dec::one() * 100),
            dir: -Vector3::z(),
        };

        assert_eq!(line.relate(&segment), LinearRelation::Colinear);

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one(), Dec::one() * 100),
            dir: Vector3::z(),
        };

        assert_eq!(line.relate(&segment), LinearRelation::Opposite);

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one() * 2, Dec::one()),
            dir: Vector3::z(),
        };

        assert_eq!(line.relate(&segment), LinearRelation::Parallell);

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one() * 2, Dec::one()),
            dir: -Vector3::z(),
        };

        assert_eq!(line.relate(&segment), LinearRelation::Parallell);

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one() * 2, Dec::one()),
            dir: -Vector3::y(),
        };

        assert_eq!(
            line.relate(&segment),
            LinearRelation::Intersect(LinearIntersection::Origin(Vector3::new(
                Dec::one(),
                Dec::one(),
                Dec::one()
            ),))
        );

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one() * 2, -Dec::one()),
            dir: -Vector3::y(),
        };

        assert_eq!(
            line.relate(&segment),
            LinearRelation::Intersect(LinearIntersection::Origin(Vector3::new(
                Dec::one(),
                Dec::one(),
                -Dec::one()
            ),))
        );

        let line = Line {
            origin: Vector3::new(Dec::one(), Dec::one() * 2, Dec::one() * Dec::from(0.5)),
            dir: -Vector3::y(),
        };

        assert_eq!(
            line.relate(&segment),
            LinearRelation::Intersect(LinearIntersection::In(Vector3::new(
                Dec::one(),
                Dec::one(),
                Dec::one() * Dec::from(0.5)
            ),))
        );
    }

    /*
    #[test]
    fn ray_point_relation() {
        let ray: Ray<Dec> = Ray {
            origin: Vector3::zero(),
            dir: Vector3::x(),
        };

        let pt = Vector3::y();

        assert_eq!(ray.relate(&pt), PointOnLine::Outside);

        let pt = Vector3::zero();
        assert_eq!(ray.relate(&pt), PointOnLine::Origin);

        let pt = Vector3::x();
        assert_eq!(ray.relate(&pt), PointOnLine::On);
    }
    */
}
