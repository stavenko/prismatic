use delaunator::Point;
use math::{ParametricIterator, Scalar, Tensor, Vector2, Vector3};
use num_traits::Zero;
use path::{Curve, GetLength, GetT, PathItem};
use std::fmt::Debug;

use crate::EdgeTensor;

pub struct BoundedFourSurface<T: EdgeTensor> {
    pub left: path::Path<T>, //order left -> top -> right -> bottom
    pub top: path::Path<T>,
    pub right: path::Path<T>,
    pub bottom: path::Path<T>,
    pub invert_triangles: bool,
    pub triangle_regularity: T::Scalar,
}

impl<S, T: EdgeTensor<Scalar = S, Vector = Vector3<S>>> BoundedFourSurface<T>
where
    S: Scalar + 'static,
{
    pub fn polygonize(&self) -> Vec<[T::Vector; 3]>
    where
        PathItem<T>: Debug,
    {
        let mut point_cloud = Vec::new();
        let border_points = self.get_border_points();
        //println!("BORDER: {:#?}", border_points);
        point_cloud.extend(border_points);
        let inner = self.get_inner_points();
        // println!("INNER: {:#?}", inner);
        point_cloud.extend(inner);

        let triangles = self.triangulate(point_cloud).unwrap();

        /*
                for [a, b, c] in &triangles {
                    println!(
                        "TRIS: [{:?}, {:?}], [{:?}, {:?}], [{:?}, {:?}]",
                        a.x, a.y, b.x, b.y, c.x, c.y,
                    );
                }

        */
        triangles
            .into_iter()
            //.map(|v| [v[0], v[1], v[2]])
            .map(|[a, b, c]| [self.get_st(a), self.get_st(b), self.get_st(c)])
            .collect()
    }

    fn get_border_points(&self) -> Vec<Vector2<T::Scalar>>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
    {
        let left_len = self.left.get_length();
        let left_x = S::zero();
        let lefts = self.left.iter().flat_map(|pi| {
            let item_len = pi.get_length();
            let param_len = item_len / left_len;
            let steps = (self.triangle_regularity * item_len).ceil();
            ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                .map(move |(l, _)| Vector2::new(left_x, l * param_len))
        });

        let right_x = S::one();
        let right_len = self.right.get_length();
        let rights = self.right.iter().flat_map(|pi| {
            let item_len = pi.get_length();
            let param_len = item_len / right_len;
            let steps = (self.triangle_regularity * item_len).ceil();
            ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                .map(move |(l, _)| Vector2::new(right_x, S::one() - l * param_len))
        });

        let top_y = S::one();
        let top_len = self.top.get_length();
        let tops = self.top.iter().flat_map(|pi| {
            let item_len = pi.get_length();
            let param_len = item_len / top_len;
            let steps = (self.triangle_regularity * item_len).ceil();
            ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                .map(move |(l, _)| Vector2::new(l * param_len, top_y))
        });

        let bottom_y = S::zero();
        let bottom_len = self.bottom.get_length();
        let bottoms = self.top.iter().flat_map(|pi| {
            let item_len = pi.get_length();
            let param_len = item_len / bottom_len;
            let steps = (self.triangle_regularity * item_len).ceil();
            ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                .map(move |(l, _)| Vector2::new(S::one() - l * param_len, bottom_y))
        });

        lefts
            .chain(tops)
            .chain(rights)
            .chain(bottoms)
            //.map(|v| v * <S as From<usize>>::from(10))
            .collect()
    }

    fn get_inner_points(&self) -> Vec<Vector2<T::Scalar>>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
    {
        let top_len = (self.top.get_length() * self.triangle_regularity).ceil();
        let bot_len = (self.bottom.get_length() * self.triangle_regularity).ceil();
        let lef_len = (self.left.get_length() * self.triangle_regularity).ceil();
        let rig_len = (self.right.get_length() * self.triangle_regularity).ceil();
        let s_steps: usize = num_traits::Float::max(top_len, bot_len)
            .to_usize()
            .unwrap_or(0);
        let t_steps: usize = num_traits::Float::max(lef_len, rig_len)
            .to_usize()
            .unwrap_or(0);

        if s_steps > 1 && t_steps > 1 {
            let ss = ParametricIterator::<T::Scalar>::new(s_steps)
                .map(|(s, _)| s)
                .skip(1)
                .take(s_steps - 1);
            let tt = ParametricIterator::<T::Scalar>::new(t_steps)
                .map(|(t, _)| t)
                .skip(1)
                .take(t_steps - 1)
                .collect::<Vec<_>>();
            ss.flat_map(|y| tt.iter().map(move |x| Vector2::new(*x, y)))
                .collect()
        } else {
            Vec::new()
        }
    }

    fn get_st(&self, pt: Vector2<<T as Tensor>::Scalar>) -> <T as EdgeTensor>::Vector
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
    {
        //Vector3::new(pt.x, pt.y, <T as Tensor>::Scalar::zero())
        let s = pt.x;
        let t = pt.y;
        let s_line = self.get_line_at_s(s);

        let t_line = self.get_line_at_t(t);

        let fs = (t - S::half()).abs() * S::two();
        let ts = (s - S::half()).abs() * S::two();

        let mut value = s_line.get_t(t) * fs + t_line.get_t(s) * (S::one() - fs);

        value += t_line.get_t(s) * ts + s_line.get_t(t) * (S::one() - ts);
        value *= S::half();

        if t < S::from(0.01_f32).unwrap() || t > S::from(0.99_f32).unwrap() {
            value = s_line.get_t(t); // This is stupid, but works for now
        }

        if s < S::from(0.01_f32).unwrap() || s > S::from(0.99_f32).unwrap() {
            value = t_line.get_t(s); // TODO: Create comprehensive math idea.
        }

        value
    }

    fn triangulate(
        &self,
        point_cloud: Vec<Vector2<<T as Tensor>::Scalar>>,
    ) -> anyhow::Result<Vec<[Vector2<<T as Tensor>::Scalar>; 3]>>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
    {
        let points: Vec<_> = point_cloud
            .into_iter()
            .map(|v| Point {
                x: v.x.to_f64().expect("Convertion error"),
                y: v.y.to_f64().expect("Convertion error"),
            })
            .collect();
        let faces = delaunator::triangulate(&points);
        let total_triangles = faces.triangles.len() / 3;
        let mut result = Vec::new();
        let from_f64 = |a| S::from_f64(a).unwrap_or(Zero::zero());
        for tri in 0..total_triangles {
            let a = faces.triangles[tri * 3];
            let b = faces.triangles[tri * 3 + 1];
            let c = faces.triangles[tri * 3 + 2];

            // println!("{tri}:TRI: {a}, {b}, {c} :");
            // println!("  --- {} {}", points[a].x, points[a].y);
            // println!("  --- {} {}", points[b].x, points[b].y);
            // println!("  --- {} {}", points[c].x, points[c].y);
            result.push([
                Vector2::new(from_f64(points[a].x), from_f64(points[a].y)),
                Vector2::new(from_f64(points[b].x), from_f64(points[b].y)),
                Vector2::new(from_f64(points[c].x), from_f64(points[c].y)),
            ]);
        }
        Ok(result)
    }

    fn get_line_at_t(&self, t: S) -> Curve<T::Vector>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
    {
        let leading_point = self.left.get_t(t);
        let subdue_point = self.right.get_t(S::one() - t);
        let leading_point_start = leading_point.get_point();
        let leading_point_weight = leading_point_start + leading_point.get_edge_dir();
        let subdue_point_start = subdue_point.get_point();
        let subdue_point_weight = subdue_point_start + subdue_point.get_edge_dir();

        Curve::new_4(
            leading_point_start,
            leading_point_weight,
            subdue_point_weight,
            subdue_point_start,
        )
    }

    fn get_line_at_s(&self, s: S) -> Curve<Vector3<S>>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
    {
        let leading_point = self.top.get_t(s);
        let subdue_point = self.bottom.get_t(S::one() - s);
        let leading_point_start = leading_point.get_point();
        let leading_point_weight = leading_point_start + leading_point.get_edge_dir();
        let subdue_point_start = subdue_point.get_point();
        let subdue_point_weight = subdue_point_start + subdue_point.get_edge_dir();

        Curve::new_4(
            subdue_point_start,
            subdue_point_weight,
            leading_point_weight,
            leading_point_start,
        )
    }
}

fn weight<S: Scalar>(i: usize, ofi: usize, j: usize, ofj: usize, t: S, s: S) -> S {
    bernstein(i, ofi, t) * bernstein(j, ofj, s)
}
fn weight_4x4<S: Scalar>(i: usize, j: usize, t: S, s: S) -> S {
    weight(i, 4, j, 4, t, s)
}

pub(crate) fn bernstein<S>(item: usize, of: usize, t: S) -> S
where
    S: Scalar,
{
    let opt = of - 1;
    let factor: S =
        S::from_usize(fact(opt) / (fact(item) * fact(opt - item))).expect("Convertion error");
    let ot = S::one() - t;
    let o_item = opt - item;

    t.pow(item as i32) * ot.pow(o_item as i32) * factor
}

const fn fact(i: usize) -> usize {
    match i {
        0 => 1,
        1 => 1,
        2 => 2,
        x => x * fact(x - 1),
    }
}
