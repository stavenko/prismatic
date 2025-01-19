use std::collections::BTreeMap;

use itertools::Itertools;
use num_traits::{Bounded, Zero};
use rand::Rng;

use crate::{
    indexes::{aabb::Aabb, vertex_index::PtId},
    planar::plane::Plane,
    polygon_basis::PolygonBasis,
};
use math::{CrossProduct, Scalar, Vector3};

use super::{
    geo_object::{GeoObject, UnRef},
    index::GeoIndex,
    rib::RibId,
    seg::{Seg, SegRef, SegmentDir},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct FaceId(pub usize);

#[derive(Debug, Clone)]
pub struct Face<S: Scalar> {
    segments: Vec<Seg>,
    aabb: Aabb<S>,
    pub(crate) ribs: Vec<RibId>,
    plane: Plane<S>,
}

impl From<usize> for FaceId {
    fn from(value: usize) -> Self {
        FaceId(value)
    }
}

pub(crate) enum FaceToFaceRelation {
    Same,
    Opposite,
    Different,
}

impl<S: Scalar> Face<S> {
    fn get_flipped(seg_one: &[Seg]) -> Vec<Seg> {
        seg_one.iter().map(|s| s.flip()).rev().collect()
    }

    fn same_segs(seg_one: &[Seg], other: &[Seg]) -> bool {
        let len_is_same = seg_one.len() == other.len();
        if seg_one.is_empty() && len_is_same {
            return true;
        }
        let mut straight = other.to_owned();

        if let Some(other_ix) = straight.iter().position(|s| *s == seg_one[0]) {
            straight.rotate_left(other_ix);

            straight == seg_one
        } else {
            false
        }
    }

    pub(crate) fn is_opposite_face(&self, other: &Self) -> FaceToFaceRelation {
        if self != other {
            FaceToFaceRelation::Different
        } else if Self::same_segs(&self.segments, &other.segments) {
            FaceToFaceRelation::Same
        } else if Self::same_segs(&self.segments, &Self::get_flipped(&other.segments)) {
            FaceToFaceRelation::Opposite
        } else {
            unreachable!("We are not different (So we have same ribs), but also not same and not opposite, like ribs are messed up");
        }
    }

    pub(crate) fn create(segments: Vec<Seg>, plane: Plane<S>, aabb: Aabb<S>) -> Self {
        let ribs = segments.iter().map(|s| s.rib_id).sorted().collect_vec();
        if ribs.contains(&RibId(645))
            && ribs.contains(&RibId(657))
            //&& ribs.contains(&RibId(653))
            && ribs.contains(&RibId(654))
            && ribs.contains(&RibId(659))
        {
            println!("CREATE ~~~~",);
        }

        Self {
            segments,
            plane,
            aabb,
            ribs,
        }
    }

    pub(crate) fn plane(&self) -> &Plane<S> {
        &self.plane
    }

    pub(crate) fn aabb(&self) -> &Aabb<S> {
        &self.aabb
    }

    pub(crate) fn replace_segments(
        &mut self,
        replacing_segment_ix: usize,
        replacement: impl IntoIterator<Item = Seg>,
    ) {
        self.segments.splice(
            replacing_segment_ix..(replacing_segment_ix + 1),
            replacement,
        );
        let ribs = self
            .segments
            .iter()
            .map(|s| s.rib_id)
            .sorted()
            .collect_vec();
        self.ribs = ribs
    }

    pub(crate) fn update_rib_index(
        &self,
        my_id: FaceId,
        rib_index: &mut BTreeMap<RibId, Vec<FaceId>>,
    ) {
        for s in &self.segments {
            GeoIndex::<S>::save_index(rib_index, s.rib_id, my_id);
        }
    }

    pub(crate) fn delete_me_from_rib_index(
        &self,
        my_id: FaceId,
        rib_index: &mut BTreeMap<RibId, Vec<FaceId>>,
    ) {
        for s in &self.segments {
            GeoIndex::<S>::remove_item_from_index(rib_index, &s.rib_id, &my_id);
        }
    }

    pub(crate) fn segments<'a>(
        &'a self,
        dir: SegmentDir,
    ) -> Box<dyn Iterator<Item = &'a Seg> + 'a> {
        match dir {
            SegmentDir::Fow => Box::new(self.segments.iter()),
            SegmentDir::Rev => Box::new(self.segments.iter().rev()),
        }
    }
}

impl<S: Scalar> PartialEq for Face<S> {
    fn eq(&self, other: &Self) -> bool {
        self.ribs == other.ribs
    }
}

pub struct FaceRef<'a, S: Scalar> {
    pub(super) face_id: FaceId,
    pub(super) index: &'a GeoIndex<S>,
}

#[allow(dead_code)]
pub struct FaceRefMut<'a, S: Scalar> {
    pub(super) face_id: FaceId,
    pub(super) index: &'a mut GeoIndex<S>,
}

impl<'a, S: Scalar> FaceRef<'a, S> {
    pub fn svg_debug_fill(
        &self,
        basis: &PolygonBasis<S>,
        fill: &str,
        additional_points: &[PtId],
    ) -> String {
        let mut items = Vec::new();
        let mut points = Vec::new();
        let colors = ["red", "green", "blue", "orange", "purple"];
        let mut path = Vec::new();

        let mut aabb = Vec::new();
        let mut min_distance_between_points = <S as Bounded>::max_value();

        for seg in self.segments(SegmentDir::Fow) {
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
        let img_width = S::from_value(800);
        let circle_size: S = width * S::from_value(20) / img_width;
        let top = aabb.min.y - (circle_size);
        let left = aabb.min.x - (circle_size);
        width += circle_size * S::from_value(2);
        height += circle_size * S::from_value(2);
        let aspect = width / height;
        let img_height = img_width / aspect;
        let font = (circle_size * S::from_value(0.7)).round();

        items.push(format!("<svg viewBox=\" {left} {top} {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{img_width}\" height=\"{img_height}\">"));
        items.push(format!(
            "<style> text{{ font: italic {font}pt sans-serif; }} </style>"
        ));
        for (ix, pt) in self
            .segments(SegmentDir::Fow)
            .map(|s| s.from_pt())
            .enumerate()
            .chain(additional_points.iter().cloned().enumerate())
        {
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
    pub(crate) fn calculate_2d_basis(&self) -> PolygonBasis<S> {
        let plane = self.plane();
        let vertices = self
            .index
            .get_face_points(self.face_id)
            .into_iter()
            .map(|pt| self.index.vertices.get_point(pt))
            .collect_vec();
        let sum: Vector3<S> = vertices.iter().copied().fold(Vector3::zero(), |a, b| a + b);
        let center = sum / S::from_value(vertices.len());
        let v = vertices
            .into_iter()
            .max_by(|a, b| {
                let aa = (*a - center).magnitude_squared();
                let bb = (*b - center).magnitude_squared();
                aa.partial_cmp(&bb).unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("Cannot calculate max distance from center");

        let distance = (v - center).magnitude();

        let plane_x = (v - center) / distance;
        let plane_y = plane.normal().cross_product(&plane_x).normalize();

        PolygonBasis {
            center,
            x: plane_x,
            y: plane_y,
        }
    }

    pub(crate) fn segments<'b>(
        &'b self,
        dir: SegmentDir,
    ) -> impl Iterator<Item = SegRef<'a, S>> + 'a {
        let get = self.index.faces.get(&self.face_id);
        get.into_iter()
            .flat_map(move |face| face.segments(dir))
            .map(|seg| self.index.load_segref(seg))
    }

    pub(crate) fn plane(&self) -> &Plane<S> {
        self.index.faces[&self.face_id].plane()
    }

    pub(crate) fn aabb(&self) -> &Aabb<S> {
        self.index.faces[&self.face_id].aabb()
    }
}

impl<'a, S: Scalar> UnRef<'a, S> for FaceRef<'a, S> {
    type Obj = FaceId;

    fn un_ref(self) -> Self::Obj {
        self.face_id
    }
}

impl<'a, S: Scalar> UnRef<'a, S> for FaceRefMut<'a, S> {
    type Obj = FaceId;

    fn un_ref(self) -> Self::Obj {
        self.face_id
    }
}

impl<'a, S: Scalar + 'a> GeoObject<'a, S> for FaceId {
    type Ref = FaceRef<'a, S>;

    type MutRef = FaceRefMut<'a, S>;

    fn make_ref(&self, index: &'a GeoIndex<S>) -> Self::Ref {
        FaceRef {
            index,
            face_id: *self,
        }
    }

    fn make_mut_ref(&self, index: &'a mut GeoIndex<S>) -> Self::MutRef {
        FaceRefMut {
            index,
            face_id: *self,
        }
    }
}
