use math::Tensor;

pub trait GetLength {
    type Tensor: Tensor;

    fn get_length(&self) -> <Self::Tensor as Tensor>::Scalar;
}
