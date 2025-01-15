use super::{get_t::GetT, length::Length, path_point::PathPoint};

pub trait Path<Tensor, Scalar>:
    GetT<Value = PathPoint<Scalar>, Scalar = Scalar> + Length<Scalar = Scalar>
{
}
