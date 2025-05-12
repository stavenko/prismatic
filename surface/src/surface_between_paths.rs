use std::fmt::Debug;
use std::ops::Mul;

use math::ParametricIterator;
use math::Scalar;
use math::Tensor;
use num_traits::Float as _;
use num_traits::ToPrimitive;
use num_traits::Zero;
use path::GetLength;
use path::GetT;
use path::Path;
use path::PathItem;

use crate::edge_tensor::EdgeTensor;

pub struct SurfaceBetweenPaths<T: EdgeTensor> {
    leading_path: path::Path<T>,
    subdue_path: path::Path<T>,
    /// Triangles per unit of length.
    triangle_regularity: T::Scalar,
    invert_triangles: bool,
}

impl<S: Tensor, T: EdgeTensor<Vector = S, Scalar = S::Scalar>> SurfaceBetweenPaths<T> {
    pub fn new(
        lead: path::Path<T>,
        subdue: path::Path<T>,
        triangle_regularity: impl Into<T::Scalar>,
        invert_triangles: bool,
    ) -> Self {
        Self {
            leading_path: lead,
            subdue_path: subdue,
            triangle_regularity: triangle_regularity.into(),
            invert_triangles,
        }
    }

    pub fn polygonize(&self) -> Vec<[T::Vector; 3]>
    where
        T::Vector: 'static,
        PathItem<T>: Debug,
        T::Scalar: Scalar,
    {
        let path_len = self.leading_path.get_length();
        let mut before_len = T::Scalar::zero();
        let lines = self
            .leading_path
            .iter()
            .flat_map(|item| {
                let len = item.get_length();
                let param_len = len / path_len;

                let steps = (self.triangle_regularity * len).ceil();
                let curves = ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                    .map(|(t, tt)| {
                        let t = before_len + (t * param_len);
                        let tt = before_len + (tt * param_len);

                        let line_t = self.get_line_at(t);
                        let line_tt = self.get_line_at(tt);
                        (line_t, line_tt)
                    })
                    .collect::<Vec<_>>();
                before_len += param_len;
                curves
            })
            .collect::<Vec<_>>();

        if let Some(max_len) = lines.iter().max_by_key(|(p, _)| {
            p.get_length()
                .mul(T::Scalar::from_value(1e6))
                .round()
                .to_isize()
                .unwrap_or(0)
        }) {
            let steps = (self.triangle_regularity * max_len.0.get_length()).ceil();

            lines
                .into_iter()
                .flat_map(|(l1, l2)| {
                    ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0)).flat_map(
                        move |(s, ss)| {
                            let a = l1.get_t(s);
                            let b = l1.get_t(ss);
                            let c = l2.get_t(ss);
                            let d = l2.get_t(s);
                            if !self.invert_triangles {
                                [[a, b, c], [a, c, d]]
                            } else {
                                [[a, c, b], [a, d, c]]
                            }
                        },
                    )
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn subdue_points(&self) -> Vec<T::Vector> {
        let path_len = self.leading_path.get_length();
        let mut before_len = T::Scalar::zero();
        self.leading_path
            .iter()
            .flat_map(|item| {
                let len = item.get_length();
                let param_len = len / path_len;

                let steps = (self.triangle_regularity * len).ceil();
                let curves = ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                    .map(|(t, _)| {
                        let t = before_len + (t * param_len);

                        self.get_subdue_point_on(t)
                    })
                    .collect::<Vec<_>>();
                before_len += len;
                curves
            })
            .collect::<Vec<_>>()
    }

    fn get_subdue_point_on(&self, t: T::Scalar) -> T::Vector {
        self.subdue_path.get_t(t).get_point()
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
}
