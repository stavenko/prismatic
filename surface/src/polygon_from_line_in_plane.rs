use derive_builder::Builder;
use math::{Scalar, Tensor};
use num_traits::{Float, One, Zero};
use path::{GetLength, GetT, Path};

use crate::EdgeTensor;

#[derive(Builder)]
pub struct PolygonFromLineInPlane<T: Tensor> {
    path: Path<T>,
    path_points: Vec<T::Scalar>,
    is_inverted: bool,
}

impl<T: Tensor> PolygonFromLineInPlane<T>
where
    T::Scalar: Float,
{
    pub fn build() -> PolygonFromLineInPlaneBuilder<T> {
        Default::default()
    }
    pub fn polygonize(&self) -> Vec<Vec<T::Vector>>
    where
        T: EdgeTensor,
    {
        let vertices: Vec<T> = self
            .path_points
            .iter()
            .map(|t| self.path.get_t(*t))
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

impl<T: Tensor> PolygonFromLineInPlaneBuilder<T> {
    pub fn set_path_points_per_component(&mut self, count: u32) -> &mut Self {
        self.path_points = Some(Self::points_per_path_item(
            self.path
                .as_ref()
                .expect("Initialize with path before calling this function"),
            count as usize + 1,
        ));
        self
    }

    fn points_per_path_item(path: &path::Path<T>, points: usize) -> Vec<T::Scalar> {
        let mut result = Vec::new();
        let path_length = path.get_length();
        let mut prev_end = T::Scalar::zero();

        for path_item in path.iter() {
            let length = path_item.get_length();
            let param_len = length / path_length;
            for p in 0..points {
                let p_param = T::Scalar::from_value(p) / T::Scalar::from_value(points);
                let point_param = p_param * param_len;
                result.push(prev_end + point_param);
            }
            prev_end += param_len;
        }

        result
    }
}
