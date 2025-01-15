use crate::decimal::Dec;
use math::Vector3;

#[derive(Debug, Clone, Default, Copy)]
#[allow(dead_code)]

pub struct Sphere {
    pub center: Vector3<Dec>,
    pub radius: Dec,
}
