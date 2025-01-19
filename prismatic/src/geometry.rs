use math::Scalar;

use crate::indexes::geo_index::mesh::MeshRefMut;

pub trait Geometry {
    fn polygonize<S: Scalar>(
        self,
        mesh: &mut MeshRefMut<S>,
        complexity: usize,
    ) -> anyhow::Result<()>;
}

pub trait GeometryDyn<S: Scalar> {
    fn polygonize(&self, mesh: MeshRefMut<S>, complexity: usize) -> anyhow::Result<()>;
}
