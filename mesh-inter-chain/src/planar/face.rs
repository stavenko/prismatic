use math::{CrossProduct, Scalar, Vector3};

use super::plane::Plane;

#[derive(Clone, Debug, PartialEq)]
pub struct Face<S: Scalar> {
    pub vertices: [Vector3<S>; 3],
    pub normal: Vector3<S>,
}

impl<S: Scalar> Face<S> {
    pub fn new_with_normal(vertices: [Vector3<S>; 3], normal: Vector3<S>) -> Self {
        Face { vertices, normal }
    }
    pub fn get_plane(&self) -> Plane<S> {
        let [u, _v, _w] = &self.vertices;
        let origin = *u;
        Plane::new_from_normal_and_point(self.normal, origin)
    }
    pub fn get_normal(&self) -> Vector3<S> {
        self.normal
    }

    pub fn new(vertices: [Vector3<S>; 3]) -> Self {
        let [u, v, w] = &vertices;
        let _origin = *u;
        let a = *v - *u;
        let b = *w - *u;

        let cross = a.cross_product(&b);

        if cross.magnitude_squared().is_zero() {
            dbg!(a.normalize().dot(&b.normalize()));
            panic!("aaa {u} , {v} , {w} ({a} x {b} , {cross}");
        }
        let normal = cross.normalize();
        Self { vertices, normal }
    }
}
