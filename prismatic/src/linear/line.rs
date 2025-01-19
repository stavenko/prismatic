use core::fmt;

use crate::indexes::geo_index::seg::SegRef;
use math::{Matrix2, Scalar, Vector2, Vector3};

#[derive(Clone)]
pub struct Line<S> {
    pub origin: Vector3<S>,
    pub dir: Vector3<S>,
}

impl<S: Scalar> Line<S> {
    pub(crate) fn get_intersection_params_seg_ref(&self, to: &SegRef<'_, S>) -> Option<(S, S)> {
        let vertex_pulling = S::from_value(0.001); // one micrometer
        let vertex_pulling_sq = vertex_pulling * vertex_pulling;

        let segment_dir = to.dir().normalize();
        let q = self.origin - to.from();

        let dot = self.dir.dot(&segment_dir);

        let m = Matrix2::new(S::one(), -dot, dot, -S::one());
        let b = -Vector2::new(q.dot(&self.dir), q.dot(&segment_dir));

        if m.determinant().abs() < vertex_pulling_sq {
            return None;
        }
        if let Some(mi) = m.try_inverse() {
            let st = mi * b;
            let p1 = self.dir * st.x + self.origin;
            let p2 = to.dir().normalize() * st.y + to.from();
            let dist = p1 - p2;
            if dist.magnitude_squared() < vertex_pulling_sq {
                Some((st.x, st.y / to.dir().magnitude()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn distance_to_pt_squared(&self, pt: Vector3<S>) -> S {
        let v = pt - self.origin;
        if v.magnitude_squared().is_zero() {
            S::zero()
        } else {
            let t = v.dot(&self.dir);
            v.dot(&v) - t * t
        }
    }
}

impl<S: Scalar> fmt::Debug for Line<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} -> {} {} {}",
            self.origin.x.round_dp(5),
            self.origin.y.round_dp(5),
            self.origin.z.round_dp(5),
            self.dir.x.round_dp(5),
            self.dir.y.round_dp(5),
            self.dir.z.round_dp(5)
        )
    }
}
