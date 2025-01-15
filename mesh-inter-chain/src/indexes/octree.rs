use crate::indexes::aabb::Aabb;
use itertools::Itertools;
use math::{Scalar, Vector3};

use crate::primitives_relation::relation::Relation;

use super::sphere::Sphere;

const MAX_NODES: usize = 3;

pub enum BoundRelation {
    Intersects,
    Unrelated,
}

pub enum BoundNodeRelation {
    Inside,
    Outside,
}

impl<T: Clone, S: Scalar> Relation<Node<T, S>> for Sphere<S> {
    type Relate = BoundNodeRelation;

    fn relate(&self, node: &Node<T, S>) -> Self::Relate {
        let m = (node.point - self.center).magnitude();
        if m < self.radius {
            BoundNodeRelation::Inside
        } else {
            BoundNodeRelation::Outside
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node<T: Clone, S> {
    pub data: T,
    pub point: Vector3<S>,
}

#[derive(Debug)]
pub enum OctreeContent<T: Clone, S> {
    Empty,
    Quadrants([Box<Octree<T, S>>; 8]),
    Container(Vec<Node<T, S>>),
}

#[derive(Debug)]
pub struct Octree<T: Clone, S> {
    aabb: Aabb<S>,
    contents: OctreeContent<T, S>,
}

impl<T: Clone, S: Scalar> Octree<T, S> {
    pub fn query_within_sphere(&self, bound: Sphere<S>) -> Vec<Node<T, S>> {
        match self.contents {
            OctreeContent::Empty => Vec::new(),
            OctreeContent::Container(ref items) => items
                .iter()
                .filter(|i| matches!(bound.relate(i), BoundNodeRelation::Inside))
                .cloned()
                .collect_vec(),
            OctreeContent::Quadrants(ref qs) => qs
                .iter()
                .filter(|q| matches!(q.aabb.relate(&bound), BoundRelation::Intersects))
                .flat_map(|q| q.query_within_sphere(bound))
                .collect(),
        }
    }

    pub fn query_within_aabb(&self, bound: Aabb<S>) -> Vec<Node<T, S>> {
        match self.contents {
            OctreeContent::Empty => Vec::new(),
            OctreeContent::Container(ref items) => items
                .iter()
                .filter(|i| matches!(bound.relate(*i), BoundNodeRelation::Inside))
                .cloned()
                .collect_vec(),
            OctreeContent::Quadrants(ref qs) => qs
                .iter()
                .filter(|q| matches!(q.aabb.relate(&bound), BoundRelation::Intersects))
                .flat_map(|q| q.query_within_aabb(bound))
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self.contents, OctreeContent::Empty)
    }

    fn empty(aabb: Aabb<S>) -> Self {
        Self {
            aabb,
            contents: OctreeContent::Empty,
        }
    }

    fn container(v: Vec<Node<T, S>>, aabb: Aabb<S>) -> Self {
        Self {
            aabb,
            contents: OctreeContent::Container(v),
        }
    }

    pub fn insert(&mut self, node: Node<T, S>) {
        let new_aabb = self.insert_recursive(node);
        if new_aabb != self.aabb {}
    }

    fn insert_recursive(&mut self, node: Node<T, S>) -> Aabb<S> {
        match &mut self.contents {
            OctreeContent::Empty => {
                self.contents = OctreeContent::Container(vec![node]);
                self.aabb
            }

            OctreeContent::Container(ref mut v) => {
                if let BoundNodeRelation::Outside = self.aabb.relate(&node) {
                    panic!(
                        "Inserting point failed - not inside bounds {:?} <== {} {} {} ",
                        self.aabb, node.point.x, node.point.y, node.point.z
                    );
                }
                v.push(node);
                if v.len() > MAX_NODES {
                    let quadrants = Self::sort(v, &self.aabb)
                        .map(|(points, aabb)| Box::new(Octree::new_with_aabb(points, aabb)));
                    self.contents = OctreeContent::Quadrants(quadrants);
                }
                self.aabb
            }

            OctreeContent::Quadrants(quadrants) => {
                let ix = Self::index(&self.aabb, &node.point);

                quadrants[ix].insert_recursive(node)
            }
        }
    }

    /*
        pub fn rebalance(&mut self) {
            if let OctreeContent::Quadrants(_) = &self.contents {
                let items = self.get_vec();
                let new_tree = Self::new(items);
                *self = new_tree;
            }
        }

        pub fn rebalance_mut(&mut self) {
            if let OctreeContent::Quadrants(_) = &self.contents {
                let items = self.get_vec();
                *self = Self::new(items);
            }
        }

    */
    pub fn get_vec(&self) -> Vec<Node<T, S>> {
        match &self.contents {
            OctreeContent::Empty => Vec::new(),
            OctreeContent::Container(v) => v.to_owned(),
            OctreeContent::Quadrants(ref qs) => qs.iter().flat_map(|q| q.get_vec()).collect(),
        }
    }

    fn index(aabb: &Aabb<S>, p: &Vector3<S>) -> usize {
        let middle = aabb.min.lerp(&aabb.max, S::half());

        #[allow(clippy::let_and_return)]
        let ix = if p.x > middle.x { 1 << 2 } else { 0 }
            + if p.y > middle.y { 1 << 1 } else { 0 }
            + if p.z > middle.z { 1 } else { 0 };
        ix
    }

    pub fn allocate<const Q: usize>(min: Vector3<S>, max: Vector3<S>) -> Self {
        Self {
            aabb: Aabb { min, max },
            contents: OctreeContent::Empty,
        }
    }

    pub fn allocate_default<const Q: usize>() -> Self {
        Self {
            aabb: Aabb::<S>::default(),
            contents: OctreeContent::Empty,
        }
    }

    #[allow(clippy::type_complexity)]
    fn sort(points: &Vec<Node<T, S>>, aabb: &Aabb<S>) -> [(Vec<Node<T, S>>, Aabb<S>); 8] {
        let mut octets = aabb.split_by_octs().map(|aabb| (Vec::new(), aabb));

        for p in points {
            let ix = Self::index(aabb, &p.point);
            octets[ix].0.push(p.clone());
        }
        octets
    }

    pub fn new_with_aabb(nodes: Vec<Node<T, S>>, aabb: Aabb<S>) -> Self {
        if nodes.is_empty() {
            Octree::empty(aabb)
        } else if nodes.len() <= MAX_NODES {
            Octree::container(nodes, aabb)
        } else {
            let quadrants = Self::sort(&nodes, &aabb)
                .map(|(nodes, aabb)| Box::new(Octree::new_with_aabb(nodes, aabb)));

            Octree {
                aabb,
                contents: OctreeContent::Quadrants(quadrants),
            }
        }
    }

    pub fn set_aabb(&mut self, aabb: Aabb<S>) {
        self.aabb = aabb
    }
}
