use math::Tensor;

pub trait EdgeTensor: Tensor {
    type Vector: Tensor;

    fn get_point(&self) -> Self::Vector;
    fn get_edge_dir(&self) -> Self::Vector;
}
