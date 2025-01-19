use rand::Rng;
use std::fmt;

use anyhow::anyhow;
use num_traits::Zero;

use crate::{
    indexes::quadtree::STABILITY_ROUNDING,
    linear::{segment::Segment, segment2d::Segment2D},
    polygon_basis::PolygonBasis,
    reversable::Reversable,
};
use math::{CrossProduct, Scalar, Vector3};

use super::plane::Plane;

#[derive(Clone)]
pub struct Polygon<S: Scalar> {
    pub vertices: Vec<Vector3<S>>,
    plane: Plane<S>,
}

pub enum PointLoc {
    Inside,
    Outside,
    Edge,
    Vertex,
}

impl<S: Scalar> PartialEq for Polygon<S> {
    fn eq(&self, other: &Self) -> bool {
        if self.vertices.is_empty() && other.vertices.is_empty() {
            true
        } else if self.vertices.len() != other.vertices.len() {
            false
        } else {
            let first = self.vertices.first().unwrap();
            let other_ix = other.vertices.iter().position(|p| {
                let d = (p - first).magnitude_squared().round_dp(STABILITY_ROUNDING);

                d == S::zero()
            });

            other_ix.is_some_and(|other_ix| {
                for i in 1..self.vertices.len() {
                    let oix = (other_ix + i) % self.vertices.len();
                    let d = (self.vertices[i] - other.vertices[oix])
                        .magnitude_squared()
                        .round_dp(STABILITY_ROUNDING);
                    if d != S::zero() {
                        return false;
                    }
                }
                true
            })
        }
    }
}

impl<S: Scalar> fmt::Debug for Polygon<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "poly")?;
        for v in &self.vertices {
            writeln!(
                f,
                "  v {} {} {}",
                v.x.round_dp(4),
                v.y.round_dp(4),
                v.z.round_dp(4)
            )?;
        }
        Ok(())
    }
}

impl<S: Scalar> Reversable for Polygon<S> {
    fn flip(mut self) -> Self {
        self.plane.flip();

        Self {
            vertices: self.vertices.into_iter().rev().collect(),
            //basis: self.basis,
            plane: self.plane,
        }
    }
}

impl<S: Scalar> Polygon<S> {
    pub fn svg_debug(&self, basis: &PolygonBasis<S>) -> String {
        let mut items = Vec::new();
        let colors = ["red", "green", "blue", "orange", "purple"];
        let mut path = Vec::new();
        for (ix, v) in self.vertices.iter().enumerate() {
            let vv = basis.project_on_plane_z(v);
            if ix <= 2 {
                items.push(format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"0.08\" fill=\"{}\"/> ",
                    vv.x.round_dp(4),
                    vv.y.round_dp(4),
                    colors[ix],
                ))
            }
            if ix == 0 {
                path.push(format!("M {} {}", vv.x.round_dp(4), vv.y.round_dp(4)));
            } else {
                path.push(format!("L {} {}", vv.x.round_dp(4), vv.y.round_dp(4)));
            }
        }
        path.push("z".to_string());
        let c = colors[rand::thread_rng().gen_range(0..colors.len())];
        items.push(format!(
            "<path stroke=\"{}\" stroke-width=\"0.06\" d = \"{}\" />",
            c,
            path.join(" ")
        ));
        items.join("\n")
    }

    pub fn get_segments_2d(&self, basis: &PolygonBasis<S>) -> Vec<Segment2D<S>> {
        let mut vv = self.vertices.iter().peekable();
        let mut segments = Vec::new();
        let first: Vector3<S> = self.vertices[0];
        while let Some(v) = vv.next() {
            let prev = basis.project_on_plane_z(v);

            let next: Vector3<S> = if let Some(p) = vv.peek() { **p } else { first };
            let next = basis.project_on_plane_z(&next);
            segments.push(Segment2D::new(prev, next));
        }
        segments
    }

    pub fn get_segments(&self) -> Vec<Segment<S>> {
        let mut vv = self.vertices.iter().peekable();
        let mut segments = Vec::new();
        let first: Vector3<S> = self.vertices[0];
        while let Some(&from) = vv.next() {
            let to: Vector3<S> = if let Some(p) = vv.peek() { **p } else { first };
            segments.push(Segment { from, to });
        }
        segments
    }

    pub fn calculate_plane(vertices: &[Vector3<S>]) -> anyhow::Result<Plane<S>> {
        let u = vertices[0];
        let v = vertices[1];
        let w = vertices[vertices.len() - 1];
        let a = v - u;
        let b = w - u;

        if a.magnitude_squared().is_zero() || b.magnitude_squared().is_zero() {
            return Err(anyhow::anyhow!(
                "Cannot calculate plane of polygon, we got repeated points"
            ));
        }
        let cross = &a.cross_product(&b);
        //dbg!(cross.magnitude());
        if cross.magnitude().is_zero() {
            return Err(anyhow::anyhow!(
                "Cannot calculate plane of polygon, cross product have zero length"
            ));
        }
        let mut plane = Plane::new_from_normal_and_point(cross.normalize(), u);
        let x = a.normalize();
        let y = b.normalize();

        let mut total_area = S::zero();
        for current in 0..vertices.len() {
            let next = (current + 1) % vertices.len();
            let x1 = vertices[current].dot(&x);
            let y1 = vertices[current].dot(&y);
            let x2 = vertices[next].dot(&x);
            let y2 = vertices[next].dot(&y);
            total_area += x1 * y2 - x2 * y1;
        }
        if total_area.is_negative() {
            plane.flip();
        }

        if total_area.is_zero() {
            return Err(anyhow!("Zero area"));
        }

        Ok(plane)
    }

    pub fn calculate_basis_2d(vertices: &[Vector3<S>]) -> anyhow::Result<PolygonBasis<S>> {
        let plane = Self::calculate_plane(vertices)?;
        let sum: Vector3<S> = vertices.iter().copied().fold(Vector3::zero(), |a, b| a + b);
        let center = sum / S::from_value(vertices.len());
        let v = vertices.first().ok_or(anyhow!("not a single point"))?;
        let plane_x = (v - center).normalize();
        let plane_y = plane.normal().cross_product(&plane_x).normalize();

        Ok(PolygonBasis {
            center,
            x: plane_x,
            y: plane_y,
        })
    }

    pub fn new(vertices: Vec<Vector3<S>>) -> anyhow::Result<Self> {
        let plane = Self::calculate_plane(&vertices)?;
        let this = Self { vertices, plane };
        Ok(this)
    }

    pub fn new_with_plane(vertices: Vec<Vector3<S>>, plane: Plane<S>) -> anyhow::Result<Self> {
        Ok(Self { vertices, plane })
    }

    pub fn get_plane(&self) -> Plane<S> {
        self.plane.clone()
    }

    pub fn get_normal(&self) -> Vector3<S> {
        self.plane.normal()
    }
}
