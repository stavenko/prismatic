pub trait GetPosition {
    type Position;
    fn get_position(&self) -> Self::Position;
    fn get_position_mut(&mut self) -> &mut Self::Position;
}

pub trait ShiftInPlane {
    type Vector3;
    type Scalar;
    fn shift_in_plane(self, normal: Self::Vector3, amount: Self::Scalar) -> Self;
}
