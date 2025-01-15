use math::Tensor;

pub trait GetT {
    type Tensor: Tensor;

    fn get_t(&self, t: <Self::Tensor as Tensor>::Scalar) -> Self::Tensor;
}
