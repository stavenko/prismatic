use core::fmt;

use num_traits::{Bounded, Zero};
use rstar::AABB;

use crate::primitives_relation::relation::Relation;
use math::{Scalar, Vector3};

use super::{
    geo_index::poly_rtree::RtreePt,
    octree::{BoundNodeRelation, BoundRelation, Node},
    sphere::Sphere,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb<S> {
    pub(crate) min: Vector3<S>,
    pub(crate) max: Vector3<S>,
}

impl<S: Scalar> Default for Aabb<S> {
    fn default() -> Self {
        Self {
            min: Zero::zero(),
            max: Zero::zero(),
        }
    }
}

impl<S: Scalar> From<Aabb<S>> for AABB<RtreePt<S>>
where
    S: Copy + fmt::Debug + num_traits::Signed + num_traits::Bounded + PartialOrd,
{
    fn from(value: Aabb<S>) -> Self {
        let p1 = value.min;
        let p2 = value.max;
        AABB::from_corners(p1.into(), p2.into())
    }
}

impl<S> Aabb<S>
where
    S: Scalar,
{
    pub fn split_x(&self) -> [Self; 2] {
        let middle = self.min.lerp(&self.max, S::half());

        [
            Aabb::from_points(&[self.min, Vector3::new(middle.x, self.max.y, self.max.z)]),
            Aabb::from_points(&[self.max, Vector3::new(middle.x, self.min.y, self.min.z)]),
        ]
    }

    pub fn split_y(&self) -> [Self; 2] {
        let middle = self.min.lerp(&self.max, S::half());

        [
            Aabb::from_points(&[self.min, Vector3::new(self.max.x, middle.y, self.max.z)]),
            Aabb::from_points(&[self.max, Vector3::new(self.min.x, middle.y, self.min.z)]),
        ]
    }
    pub fn split_z(&self) -> [Self; 2] {
        let middle = self.min.lerp(&self.max, S::half());

        [
            Aabb::from_points(&[self.min, Vector3::new(self.max.x, self.max.y, middle.z)]),
            Aabb::from_points(&[self.max, Vector3::new(self.min.x, self.min.y, middle.z)]),
        ]
    }

    pub fn split_by_octs(&self) -> [Self; 8] {
        let x = self.split_x();
        let [ny, py] = x.map(|x| x.split_y());
        let [[[nxnynz, nxnypz], [nxpynz, nxpypz]], [[pxnynz, pxnypz], [pxpynz, pxpypz]]] =
            [ny.map(|y| y.split_z()), py.map(|y| y.split_z())];

        [
            nxnynz, nxnypz, nxpynz, nxpypz, pxnynz, pxnypz, pxpynz, pxpypz,
        ]
    }

    pub fn from_points(points: &[Vector3<S>]) -> Self {
        let mut min: Vector3<S> = Vector3::new(
            Bounded::max_value(),
            Bounded::max_value(),
            Bounded::max_value(),
        );
        let mut max: Vector3<S> = Vector3::new(
            Bounded::min_value(),
            Bounded::min_value(),
            Bounded::min_value(),
        );
        for p in points.iter() {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);

            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        let d = S::from_value(0.000_1);
        let min = min - Vector3::new(d, d, d);
        let max = max + Vector3::new(d, d, d);

        Aabb { min, max }
    }
    pub fn add(&mut self, pt: Vector3<S>) {
        self.min.x = num_traits::Float::min(self.min.x, pt.x);
        self.min.y = num_traits::Float::min(self.min.y, pt.y);
        self.min.z = num_traits::Float::min(self.min.x, pt.z);
    }

    #[allow(unused)]
    pub(crate) fn merge(mut self, aabb: Aabb<S>) -> Aabb<S> {
        self.min.x = num_traits::Float::min(self.min.x, aabb.min.x);
        self.min.y = num_traits::Float::min(self.min.y, aabb.min.y);
        self.min.z = num_traits::Float::min(self.min.x, aabb.min.z);

        self.max.x = num_traits::Float::max(self.max.x, aabb.max.x);
        self.max.y = num_traits::Float::max(self.max.y, aabb.max.y);
        self.max.z = num_traits::Float::max(self.max.x, aabb.max.z);

        self
    }
}

impl<S: Scalar> Relation<Sphere<S>> for Aabb<S> {
    type Relate = BoundRelation;

    fn relate(&self, sphere: &Sphere<S>) -> Self::Relate {
        let center = [sphere.center.x, sphere.center.y, sphere.center.z];
        let min_b = [self.min.x, self.min.y, self.min.z];
        let max_b = [self.max.x, self.max.y, self.max.z];

        let min_dist: S = center
            .iter()
            .zip(min_b)
            .map(|(&c, m)| if c < m { (c - m).pow(2_i32) } else { S::zero() })
            .sum();

        let max_dist: S = center
            .iter()
            .zip(max_b)
            .map(|(&c, m)| if c > m { (c - m).pow(2_i32) } else { S::zero() })
            .sum();

        //dbg!(max_dist, min_dist, sphere.radius);
        if (max_dist + min_dist) < sphere.radius.pow(2_i32) {
            BoundRelation::Intersects
        } else {
            BoundRelation::Unrelated
        }
    }
}

impl<T: Clone, S: Scalar> Relation<Node<T, S>> for Aabb<S> {
    type Relate = BoundNodeRelation;

    fn relate(&self, to: &Node<T, S>) -> Self::Relate {
        if to.point.x >= self.min.x
            && to.point.x <= self.max.x
            && to.point.y >= self.min.y
            && to.point.y <= self.max.y
            && to.point.z >= self.min.z
            && to.point.z <= self.max.z
        {
            BoundNodeRelation::Inside
        } else {
            BoundNodeRelation::Outside
        }
    }
}

impl<S: Scalar> Relation<Aabb<S>> for Aabb<S> {
    type Relate = BoundRelation;

    fn relate(&self, _to: &Aabb<S>) -> Self::Relate {
        todo!("implement aabb <> aabb");
    }
}
