use math::{ParametricIterator, Tensor};
use num_traits::{Float, ToPrimitive};
use path::{GetLength, GetT, Path};

use crate::EdgeTensor;

pub struct PolygonFromLineInPlane<T: Tensor> {
    path: Path<T>,
    triangle_regularity: T::Scalar,
    is_inverted: bool,
}

impl<T: Tensor> PolygonFromLineInPlane<T>
where
    T::Scalar: Float,
{
    pub fn new(path: Path<T>, triangles: impl Into<T::Scalar>, inverted: bool) -> Self {
        Self {
            path,
            triangle_regularity: triangles.into(),
            is_inverted: inverted,
        }
    }

    pub fn polygonize(&self) -> Vec<Vec<T::Vector>>
    where
        T: EdgeTensor,
    {
        let vertices: Vec<T> = self
            .path
            .iter()
            .flat_map(move |path_item| {
                let len = path_item.get_length();

                let steps = (self.triangle_regularity * len).ceil();

                ParametricIterator::<T::Scalar>::new(steps.to_usize().unwrap_or(0))
                    .map(|(t, _)| path_item.get_t(t))
            })
            .collect();

        if self.is_inverted {
            vec![vertices
                .into_iter()
                .map(|t| t.get_point())
                .rev()
                .collect::<Vec<_>>()]
        } else {
            vec![vertices
                .into_iter()
                .map(|t| t.get_point())
                .collect::<Vec<_>>()]
        }
    }
}
