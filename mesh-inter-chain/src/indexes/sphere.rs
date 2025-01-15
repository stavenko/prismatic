use math::Vector3;

#[derive(Debug, Clone, Default, Copy)]
pub struct Sphere<S> {
    pub center: Vector3<S>,
    pub radius: S,
}
