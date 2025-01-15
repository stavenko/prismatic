use math::Tensor;

pub trait UpdateStartEnd {
    type Tensor: Tensor;
    fn update_start(&mut self, start: Self::Tensor);
    fn update_end(&mut self, end: Self::Tensor);
}
