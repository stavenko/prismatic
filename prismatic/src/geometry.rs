use math::{BaseOrigin, Scalar, Vector3};

use crate::indexes::geo_index::mesh::MeshRefMut;

pub trait Geometry {
    fn polygonize<S: Scalar>(
        self,
        mesh: &mut MeshRefMut<S>,
        complexity: usize,
    ) -> anyhow::Result<()>;
}

pub trait GeometryDyn<S: Scalar> {
    fn render(&self) -> Vec<Vec<Vector3<S>>>;

    fn render_with_origin(&self, basis: BaseOrigin<S>) -> Vec<Vec<Vector3<S>>>;

    fn polygonize(&self, mut mesh: MeshRefMut<S>) -> anyhow::Result<()> {
        for p in self.render() {
            mesh.add_polygon(p.as_slice())?;
        }
        Ok(())
    }

    fn polygonize_with_origin(
        &self,
        mut mesh: MeshRefMut<S>,
        origin: BaseOrigin<S>,
    ) -> anyhow::Result<()> {
        for p in self.render_with_origin(origin) {
            mesh.add_polygon(p.as_slice())?;
        }
        Ok(())
    }
}
