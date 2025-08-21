use math::BaseOrigin;
use math::CrossProduct;
use math::Dot;
use math::Vector3;
use num_traits::FromPrimitive;
use num_traits::One;
use std::fmt::Debug;
use std::process::Output;

use delaunator::Point;
use math::Scalar;
use math::Tensor;
use math::Vector2;
use num_traits::ToPrimitive;
use num_traits::Zero;
use path::GetLength;
use path::GetT;
use path::Path;
use path::PathItem;

use crate::edge_tensor::EdgeTensor;
use crate::SurfaceBetweenPathsBuilder;

pub struct SurfaceBetweenPaths<T: EdgeTensor> {
    pub(crate) leading_path: path::Path<T>,
    pub(crate) subdue_path: path::Path<T>,
    /// Triangles per unit of length.
    pub(crate) inner_points_t: usize,
    pub(crate) inner_points_s: usize,

    pub(crate) leading_path_points: Vec<T::Scalar>,
    pub(crate) subdue_path_path_points: Vec<T::Scalar>,
    pub(crate) zero_border_points: Vec<T::Scalar>,
    pub(crate) one_border_points: Vec<T::Scalar>,
    pub(crate) invert_triangles: bool,
    pub(crate) ignore_edge_power: bool,

    pub(crate) l_padding: T::Scalar,
    pub(crate) s_padding: T::Scalar,
    pub(crate) z_padding: T::Scalar,
    pub(crate) o_padding: T::Scalar,
}

impl<S: Tensor + CrossProduct<Output = S>, T: EdgeTensor<Vector = S, Scalar = S::Scalar>>
    SurfaceBetweenPaths<T>
{
    pub fn build() -> SurfaceBetweenPathsBuilder<T> {
        SurfaceBetweenPathsBuilder::default()
    }

    pub fn polygonize(&self) -> Vec<[T::Vector; 3]>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
        T::Scalar: Scalar,
    {
        let border_points = self.get_border_points();
        let inner_points = self.get_inner_points();
        let triangles = self
            .triangulate(border_points.into_iter().chain(inner_points).collect())
            .unwrap();

        if self.ignore_edge_power {
            triangles
                .into_iter()
                .map(|[a, b, c]| {
                    [
                        self.get_st_no_edge(a),
                        self.get_st_no_edge(b),
                        self.get_st_no_edge(c),
                    ]
                })
                .collect()
        } else {
            triangles
                .into_iter()
                .map(|[a, b, c]| [self.get_st(a), self.get_st(b), self.get_st(c)])
                .collect()
        }
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
        let from_f64 = |a| S::Scalar::from_f64(a).unwrap_or(Zero::zero());
        for tri in 0..total_triangles {
            let a = faces.triangles[tri * 3];
            let b = faces.triangles[tri * 3 + 1];
            let c = faces.triangles[tri * 3 + 2];

            if self.invert_triangles {
                result.push([
                    Vector2::new(from_f64(points[a].x), from_f64(points[a].y)),
                    Vector2::new(from_f64(points[c].x), from_f64(points[c].y)),
                    Vector2::new(from_f64(points[b].x), from_f64(points[b].y)),
                ]);
            } else {
                result.push([
                    Vector2::new(from_f64(points[a].x), from_f64(points[a].y)),
                    Vector2::new(from_f64(points[b].x), from_f64(points[b].y)),
                    Vector2::new(from_f64(points[c].x), from_f64(points[c].y)),
                ]);
            }
        }
        Ok(result)
    }

    fn get_line_at(&self, t: T::Scalar) -> path::Path<T::Vector>
    where
        T::Vector: 'static,
    {
        let leading_point = self.leading_path.get_t(t);
        let subdue_point = self.subdue_path.get_t(t);
        let leading_point_start = leading_point.get_point();
        let leading_point_weight = leading_point_start + leading_point.get_edge_dir();
        let subdue_point_start = subdue_point.get_point();
        let subdue_point_weight = subdue_point_start + subdue_point.get_edge_dir();

        Path::build()
            .start(leading_point_start)
            .quad_4_to(
                leading_point_weight,
                subdue_point_weight,
                subdue_point_start,
            )
            .build()
    }

    fn get_line_no_edge_at(&self, t: T::Scalar) -> path::Path<T::Vector>
    where
        T::Vector: 'static,
    {
        let leading_point = self.leading_path.get_t(t);
        let subdue_point = self.subdue_path.get_t(t);
        let leading_point_start = leading_point.get_point();
        //let leading_point_weight = leading_point_start + leading_point.get_edge_dir();
        let subdue_point_start = subdue_point.get_point();
        //let subdue_point_weight = subdue_point_start + subdue_point.get_edge_dir();

        Path::build()
            .start(leading_point_start)
            .line_to(subdue_point_start)
            .build()
    }

    fn get_st(&self, a: Vector2<S::Scalar>) -> S
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
        T::Scalar: Scalar,
    {
        let l = self.get_line_at(a.x);
        l.get_t(a.y)
    }

    fn get_st_no_edge(&self, a: Vector2<S::Scalar>) -> S
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
        T::Scalar: Scalar,
    {
        let l = self.get_line_no_edge_at(a.x);
        l.get_t(a.y)
    }

    fn get_border_points(&self) -> Vec<Vector2<S::Scalar>>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
        T::Scalar: Scalar,
    {
        let mut border_points = Vec::new();
        for p in &self.leading_path_points {
            border_points.push(Vector2::new(*p, S::Scalar::zero()));
        }

        let path_len = self.leading_path.get_length();
        let mut total_len = S::Scalar::zero();

        for p in self.leading_path.iter().take(self.leading_path.len() - 1) {
            let len = p.get_length();
            let param_len = len / path_len;
            total_len += param_len;
            border_points.push(Vector2::new(total_len, S::Scalar::zero()));
        }

        for p in &self.subdue_path_path_points {
            border_points.push(Vector2::new(*p, S::Scalar::one()));
        }

        for p in &self.zero_border_points {
            border_points.push(Vector2::new(S::Scalar::zero(), *p));
        }

        for p in &self.one_border_points {
            border_points.push(Vector2::new(S::Scalar::one(), *p));
        }

        border_points
    }

    fn get_inner_points(&self) -> Vec<Vector2<S::Scalar>>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
        T::Scalar: Scalar,
    {
        let mut inner_points = Vec::new();
        let total_area_s = S::Scalar::one() - (self.z_padding + self.o_padding);
        let total_area_t = S::Scalar::one() - (self.l_padding + self.s_padding);
        let step_s = total_area_s / S::Scalar::from_value(self.inner_points_s);
        let step_t = total_area_t / S::Scalar::from_value(self.inner_points_t);
        for s in 0..self.inner_points_s {
            for t in 0..self.inner_points_t {
                inner_points.push(Vector2::new(
                    S::Scalar::from_value(s) * step_s + self.o_padding,
                    S::Scalar::from_value(t) * step_t + self.l_padding,
                ));
            }
        }

        inner_points
    }
}
impl<S: Scalar + 'static, T: EdgeTensor<Vector = Vector3<S>, Scalar = S>> SurfaceBetweenPaths<T> {
    pub fn get_basis_at(&self, st: Vector2<S>) -> math::BaseOrigin<S> {
        let pt = self.get_st(st);
        let d = S::from_value(0.0001);
        let ds = if st.y + d <= S::one() {
            let s_pt = dbg!(self.get_st(st + Vector2::new(S::zero(), d)));
            (s_pt - pt).normalize()
        } else {
            let s_pt = self.get_st(st - Vector2::new(S::zero(), d));
            (pt - s_pt).normalize()
        };

        let dt = if st.x + d <= S::one() {
            let s_pt = dbg!(self.get_st(st + Vector2::new(d, S::zero())));
            (s_pt - pt).normalize()
        } else {
            let s_pt = self.get_st(st - Vector2::new(d, S::zero()));
            (pt - s_pt).normalize()
        };
        let normal = dt.cross_product(&ds).normalize();
        BaseOrigin::new().offset(pt).align_z(normal).align_x(dt)
    }
}
