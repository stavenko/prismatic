pub trait CrossProduct<Rhs = Self> {
    type Output;
    fn cross_product(&self, rhs: &Rhs) -> Self::Output;
}
