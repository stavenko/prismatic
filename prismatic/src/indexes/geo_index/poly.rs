use std::fmt;

use itertools::Itertools;
use math::Scalar;
use num_traits::Bounded;
use rand::Rng;

use crate::{indexes::aabb::Aabb, planar::plane::Plane, polygon_basis::PolygonBasis};
use math::Vector2;
use math::Vector3;

use super::{
    face::{Face, FaceId},
    geo_object::{GeoObject, UnRef},
    index::GeoIndex,
    mesh::MeshId,
    seg::{SegRef, SegmentDir},
};

#[derive(PartialEq, Eq, Clone, Debug, Hash, Copy)]
pub struct UnrefPoly {
    pub mesh_id: MeshId,
    pub poly_id: PolyId,
}

impl<'a, S: Scalar> PolyRef<'a, S> {
    pub fn segments(&self) -> impl Iterator<Item = SegRef<'a, S>> + 'a {
        let poly = self.polygon();
        let face = self.index.load_face_ref(poly.face_id());
        face.segments(poly.dir())
    }

    pub fn svg_debug(&self, vertices: Vec<Vector2<S>>) -> String
    where
        S: fmt::Display,
    {
        let mut items = Vec::new();
        let colors = ["red", "green", "blue", "orange", "purple"];
        let mut path = Vec::new();
        for (ix, vv) in vertices.iter().enumerate() {
            //let vv = basis.project_on_plane_z(v);
            if ix <= 2 {
                items.push(format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"0.08\" fill=\"{}\"/> ",
                    vv.x.round_dp(2),
                    vv.y.round_dp(2),
                    colors[ix],
                ))
            }
            if ix == 0 {
                path.push(format!("M {} {}", vv.x, vv.y));
            } else {
                path.push(format!("L {} {}", vv.x, vv.y));
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

    pub fn svg_debug_fill(&self, basis: &PolygonBasis<S>, fill: &str) -> String
    where
        S: fmt::Display,
    {
        let mut items = Vec::new();
        let mut points = Vec::new();
        let colors = ["red", "green", "blue", "orange", "purple"];
        let mut path = Vec::new();

        let mut aabb = Vec::new();
        let mut min_distance_between_points = <S as Bounded>::max_value();

        for seg in self.segments() {
            let pt = seg.from_pt();
            let to_pt = seg.to_pt();
            let v = self.index.vertices.get_point(pt);
            let to = self.index.vertices.get_point(to_pt);
            let v2 = basis.project_on_plane_z(&v) * S::from_value(1000);
            let to_v2 = basis.project_on_plane_z(&to) * S::from_value(1000);
            let d = (to_v2 - v2).magnitude();
            min_distance_between_points = num_traits::Float::min(min_distance_between_points, d);
            //let vv = basis.project_on_plane_z(v);
            let v3 = Vector3::new(v2.x, v2.y, S::zero());
            aabb.push(v3);
        }

        let aabb = Aabb::from_points(&aabb);
        let mut width = aabb.max.x - aabb.min.x;
        let mut height = aabb.max.y - aabb.min.y;
        let circle_size: S = min_distance_between_points * S::from_value(4);
        let top = aabb.min.y - (circle_size / S::from_value(2));
        let left = aabb.min.x - (circle_size / S::from_value(2));
        width += circle_size * S::from_value(2);
        height += circle_size * S::from_value(2);
        let aspect = width / height;
        let img_width = S::from_value(800);
        let img_height = img_width / aspect;
        let font = circle_size * S::from_value(0.7);

        items.push(format!("<svg viewBox=\" {left} {top} {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{img_width}\" height=\"{img_height}\">"));
        items.push(format!(
            "<style> text{{ font: italic {font}pt sans-serif; }} </style>"
        ));
        for (ix, pt) in self.segments().map(|s| s.from_pt()).enumerate() {
            let v = self.index.vertices.get_point(pt);
            let v2 = basis.project_on_plane_z(&v) * S::from_value(1000);
            points.push(format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{circle_size}\" fill=\"{}\"/> <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" >{pt} </text>
                ",
                v2.x.round_dp(9),
                v2.y.round_dp(9),
                colors[ix % colors.len()],
                v2.x.round_dp(9),
                v2.y.round_dp(9),
            ));

            if ix == 0 {
                path.push(format!("M {} {}", v2.x.round_dp(9), v2.y.round_dp(9)));
            } else {
                path.push(format!("L {} {}", v2.x.round_dp(9), v2.y.round_dp(9)));
            }
        }
        path.push("z".to_string());
        let c = colors[rand::thread_rng().gen_range(0..colors.len())];

        items.push(format!(
            "<path stroke=\"{}\" fill=\"{fill}\" stroke-width=\"0.0\" d = \"{}\" />",
            c,
            path.join(" ")
        ));
        items.extend(points);
        items.push("</svg>".to_string());
        items.join("\n")
    }

    pub(crate) fn serialized_polygon_pt(&self) -> String {
        let mut collect_vec = self
            .segments()
            .map(|seg| format!("{}", seg.from_pt()))
            .collect_vec();
        collect_vec.reverse();
        collect_vec.join(", ")
    }

    pub(crate) fn plane(&self) -> Plane<S> {
        match self.dir() {
            SegmentDir::Fow => self.polygon().face().plane().to_owned(),
            SegmentDir::Rev => self.polygon().face().plane().to_owned().flipped(),
        }
    }

    fn polygon(&self) -> PolyRef<S> {
        self.index.load_polygon_ref(self.mesh_id, self.poly_id)
    }

    fn face(&self) -> &Face<S> {
        let face_id = self.polygon().face_id();
        &self.index.faces[&face_id]
    }

    fn print_debug(&self) -> String {
        self.segments()
            .map(|s| {
                let f = s.from();
                let t = s.to();
                format!("{} {} {} -> {} {} {}", f.x, f.y, f.z, t.x, t.y, t.z)
            })
            .join("\n")
    }

    pub fn face_id(&self) -> FaceId {
        self.index.meshes[&self.mesh_id].polies[&self.poly_id].face_id
    }

    pub fn dir(&self) -> SegmentDir {
        self.index.meshes[&self.mesh_id].polies[&self.poly_id].dir
    }

    pub fn poly_id(&self) -> PolyId {
        self.poly_id
    }

    pub fn mesh_id(&self) -> MeshId {
        self.mesh_id
    }

    pub(crate) fn normal(&self) -> Vector3<S> {
        self.plane().normal()
    }
}

#[derive(Clone)]
pub struct PolyRef<'a, S: Scalar> {
    pub(super) poly_id: PolyId,
    pub(super) mesh_id: MeshId,
    pub(super) index: &'a GeoIndex<S>,
}

impl<'a, S: Scalar> UnRef<'a, S> for PolyRef<'a, S> {
    type Obj = UnrefPoly;

    fn un_ref(self) -> Self::Obj {
        UnrefPoly {
            mesh_id: self.mesh_id,
            poly_id: self.poly_id,
        }
    }
}

impl<'a, S: Scalar> UnRef<'a, S> for PolyRefMut<'a, S> {
    type Obj = UnrefPoly;

    fn un_ref(self) -> Self::Obj {
        UnrefPoly {
            mesh_id: self.mesh_id,
            poly_id: self.poly_id,
        }
    }
}

impl<'a, S: Scalar + 'a> GeoObject<'a, S> for UnrefPoly {
    type Ref = PolyRef<'a, S>;

    type MutRef = PolyRefMut<'a, S>;

    fn make_ref(&self, index: &'a GeoIndex<S>) -> Self::Ref {
        PolyRef {
            poly_id: self.poly_id,
            mesh_id: self.mesh_id,
            index,
        }
    }

    fn make_mut_ref(&self, index: &'a mut GeoIndex<S>) -> Self::MutRef {
        PolyRefMut {
            poly_id: self.poly_id,
            mesh_id: self.mesh_id,
            index,
        }
    }
}

pub struct PolyRefMut<'a, S: Scalar> {
    pub(super) poly_id: PolyId,
    pub(super) mesh_id: MeshId,
    pub(super) index: &'a mut GeoIndex<S>,
}
impl<S: Scalar> PolyRefMut<'_, S> {
    pub(crate) fn change_face(&mut self, face_id: FaceId) {
        if let Some(p) = self
            .index
            .meshes
            .get_mut(&self.mesh_id)
            .and_then(|m| m.polies.get_mut(&self.poly_id))
        {
            p.face_id = face_id;
        }
    }
    pub(crate) fn replace(&mut self, replacement: Vec<Poly>) {
        let poly_ix = self.poly_id;
        if self.mesh_id == 6 && (poly_ix == 35) {
            println!("replace poly 35 with {replacement:?}");
        }
        if let Some(mesh) = self.index.meshes.get_mut(&self.mesh_id) {
            mesh.polies.remove(&poly_ix);
            for p in replacement {
                mesh.add(p);
            }
        }
    }

    pub fn remove(&mut self) {
        self.index.remove_polygon(self.poly_id, self.mesh_id)
    }

    pub fn flip(&mut self) {
        if let Some(poly) = self
            .index
            .meshes
            .get_mut(&self.mesh_id)
            .and_then(|m| m.polies.get_mut(&self.poly_id))
        {
            poly.flip()
        }
    }
}

impl<S: Scalar> fmt::Debug for PolyRef<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_debug())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Poly {
    pub(crate) face_id: FaceId,
    pub(crate) dir: SegmentDir,
}

impl Poly {
    pub(crate) fn fow(face_id: FaceId) -> Self {
        Self {
            face_id,
            dir: SegmentDir::Fow,
        }
    }

    pub(crate) fn rev(face_id: FaceId) -> Self {
        Self {
            face_id,
            dir: SegmentDir::Rev,
        }
    }

    fn flip(&mut self) {
        self.dir = self.dir.flip();
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct PolyId(pub usize);

impl PartialEq<usize> for PolyId {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl From<usize> for PolyId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl fmt::Debug for PolyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PolyId:{}", self.0)
    }
}
