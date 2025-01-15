use num_traits::Zero;

use crate::{
    indexes::quadtree::STABILITY_ROUNDING,
    linear::{line::Line, segment::Segment},
    planar::{plane::Plane, polygon::Polygon},
};
use math::{Matrix2, Scalar, Vector2, Vector3};

use super::relation::Relation;

#[derive(Debug)]
pub enum PlanarRelation<S: Scalar> {
    Coplanar,
    Intersect(Line<S>),
    Opposite,
    Parallel,
}

#[allow(unused)]
pub enum PolygonRelation<S: Scalar> {
    Coplanar,
    Opposite,
    CommonVertex(Vector3<S>),
    CommonEdge(Segment<S>),
    Split(Polygon<S>, Polygon<S>), // TODO: Replace to vectors
    NonIntersecting,
}

impl<S: Scalar> Relation<Plane<S>> for Plane<S> {
    type Relate = PlanarRelation<S>;

    fn relate(&self, to: &Plane<S>) -> Self::Relate {
        let dir = self.normal().cross(&to.normal());

        if dir
            .magnitude_squared()
            .round_dp(STABILITY_ROUNDING)
            .is_zero()
        {
            if self
                .normal()
                .dot(&to.normal())
                .round_dp(STABILITY_ROUNDING - 3)
                .is_one()
            {
                if (self.d() - to.d())
                    .round_dp(STABILITY_ROUNDING - 3)
                    .is_zero()
                {
                    PlanarRelation::Coplanar
                } else {
                    PlanarRelation::Parallel
                }
            } else if (self.d() + to.d()).round_dp(STABILITY_ROUNDING).is_zero() {
                PlanarRelation::Opposite
            } else {
                PlanarRelation::Parallel
            }
        } else {
            let dir_len = dir.magnitude_squared().round_dp(STABILITY_ROUNDING).sqrt();
            let dir = dir / dir_len;
            let x = dir.x.abs();
            let y = dir.y.abs();
            let z = dir.z.abs();

            #[allow(clippy::nonminimal_bool)]
            if (x > y && x > z) || (x == y && x > z) {
                let mat = Matrix2::new(
                    self.normal().y,
                    self.normal().z,
                    to.normal().y,
                    to.normal().z,
                );
                let inv_mat = mat.try_inverse().unwrap();
                let ds = Vector2::new(self.d(), to.d());
                let r = inv_mat * ds;

                let mut origin = Vector3::zero();
                origin.y = r.x;
                origin.z = r.y;

                PlanarRelation::Intersect(Line { origin, dir })
            } else if y > x && y > z || (y == z && y > x) {
                let mat = Matrix2::new(
                    self.normal().x,
                    self.normal().z,
                    to.normal().x,
                    to.normal().z,
                );
                let inv_mat = mat.try_inverse().unwrap();
                let ds = Vector2::new(self.d(), to.d());
                let r = inv_mat * ds;

                let mut origin = Vector3::zero();
                origin.x = r.x;
                origin.z = r.y;

                PlanarRelation::Intersect(Line { origin, dir })
            } else if (z > x && z > y) || (x == z && x > y) {
                let mat = Matrix2::new(
                    self.normal().x,
                    self.normal().y,
                    to.normal().x,
                    to.normal().y,
                );
                let inv_mat = mat.try_inverse().unwrap();
                let ds = Vector2::new(self.d(), to.d());
                let r = inv_mat * ds;

                let mut origin = Vector3::zero();
                origin.x = r.x;
                origin.y = r.y;

                PlanarRelation::Intersect(Line { origin, dir })
            } else {
                unreachable!();
            }
        }
    }
}
