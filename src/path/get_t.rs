pub trait GetT {
    type Value;
    type Scalar;
    fn get_t(&self, t: Self::Scalar) -> Self::Value;
}
