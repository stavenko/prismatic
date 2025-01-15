use std::marker::PhantomData;

use crate::hyper_path::hyper_point::Tensor;

pub struct StraitLine<T, S>(T, T, PhantomData<S>)
where
    T: Tensor<Scalar = S>;

impl<T> GetT for HyperLine<T>
where
    T: Tensor + Mul<T::Scalar, Output = T>,
    T::Scalar: From<u16>,
{
    type Value = T;
    type Scalar = T::Scalar;

    fn get_t(&self, t: Self::Scalar) -> Self::Value
where {
        let o = self.0.len();
        let ws = (0..o).map(|i| bernstein::<_>(i, o, t));
        let v: Vec<T> = ws.zip(&self.0).map(|(w, t)| *t * w).collect_vec();
        v.into_iter().fold(T::zero(), |a, t| a + t)
    }
}
