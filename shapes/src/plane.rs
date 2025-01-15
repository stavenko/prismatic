use itertools::Itertools;
use math::BaseOrigin;
use math::ParametricIterator;
use math::Scalar;

use math::Vector3;
use mesh_inter_chain::geometry::GeometryDyn;
use mesh_inter_chain::indexes::geo_index::mesh::MeshRefMut;

pub struct Plane<F: Scalar> {
    origin: BaseOrigin<F>,
    width: F,
    height: F,
    resolution: usize,
}

impl<F> Plane<F>
where
    F: Scalar,
{
    pub fn centered(
        origin: BaseOrigin<F>,
        width: impl Into<F>,
        height: impl Into<F>,
        resolution: usize,
    ) -> Self {
        Self {
            origin,
            width: width.into(),
            height: height.into(),
            resolution,
        }
    }

    pub fn render(&self) -> Vec<Vec<Vector3<F>>> {
        let wf = self.origin.center
            - self.origin.x() * (self.width / <F as From<usize>>::from(2usize))
            - self.origin.y() * (self.height / <F as From<usize>>::from(2usize));

        ParametricIterator::<F>::new(self.resolution)
            .flat_map(|(s, ss)| {
                ParametricIterator::<F>::new(self.resolution).map(move |(t, tt)| {
                    let ws: Vector3<F> = self.origin.x() * self.width * s;
                    let wss: Vector3<F> = self.origin.x() * self.width * ss;

                    let ht: Vector3<F> = self.origin.y() * self.width * t;
                    let htt: Vector3<F> = self.origin.y() * self.width * tt;
                    let a = wf + ws + ht;
                    let b = wf + wss + ht;
                    let c = wf + wss + htt;
                    let d = wf + ws + htt;
                    vec![a, b, c, d]
                })
            })
            .collect_vec()
    }
}

impl<F: Scalar> GeometryDyn<F> for Plane<F> {
    fn polygonize(&self, mut mesh: MeshRefMut<F>, _complexity: usize) -> anyhow::Result<()> {
        for p in self.render() {
            mesh.add_polygon(p.as_slice())?;
        }

        Ok(())
    }
}
