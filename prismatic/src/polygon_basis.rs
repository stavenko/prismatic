use core::fmt;

use math::Scalar;
use math::Vector2;
use math::Vector3;

#[derive(Clone)]
pub struct PolygonBasis<S> {
    pub center: Vector3<S>,
    pub x: Vector3<S>,
    pub y: Vector3<S>,
}

impl<S: Scalar> fmt::Debug for PolygonBasis<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  o {} {} {}",
            self.center.x, self.center.y, self.center.z
        )?;
        writeln!(f, "  x {} {} {}", self.x.x, self.x.y, self.x.z)?;
        writeln!(f, "  y {} {} {}", self.y.x, self.y.y, self.y.z)?;
        Ok(())
    }
}
impl<S: Scalar> PolygonBasis<S> {
    pub fn project_on_plane_z(&self, point: &Vector3<S>) -> Vector2<S> {
        let PolygonBasis { center, x, y, .. } = self;
        let x = (*point - *center).dot(x);
        let y = (*point - *center).dot(y);
        Vector2::new(x, y)
    }

    pub fn unproject(&self, point: &Vector2<S>) -> Vector3<S> {
        let PolygonBasis { center, x, y } = self;
        *center + *x * point.x + *y * point.y
    }
}
