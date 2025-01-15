use core::fmt;

use math::{Scalar, Vector3};

#[derive(Clone)]
pub struct Ray<S> {
    pub origin: Vector3<S>,
    pub dir: Vector3<S>,
}

impl<S: Scalar> fmt::Debug for Ray<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} -> {} {} {}",
            self.origin.x.round_dp(4),
            self.origin.y.round_dp(4),
            self.origin.z.round_dp(4),
            self.dir.x.round_dp(4),
            self.dir.y.round_dp(4),
            self.dir.z.round_dp(4)
        )
    }
}
