use itertools::Itertools;
use math::BaseOrigin;
use math::ParametricIterator;
use math::Scalar;

use math::Vector3;
use num_traits::ToPrimitive;
use prismatic::{geometry::GeometryDyn, indexes::geo_index::mesh::MeshRefMut};

#[derive(Clone)]
pub struct Plane<F: Scalar> {
    zero: BaseOrigin<F>,
    width: F,
    height: F,
    resolution: usize,
}

impl<F> Plane<F>
where
    F: Scalar,
{
    pub fn centered(
        center: BaseOrigin<F>,
        width: impl ToPrimitive,
        height: impl ToPrimitive,
        resolution: usize,
    ) -> Self {
        let w = F::from(width).unwrap();
        let h = F::from(height).unwrap();
        Self {
            zero: center.offset_x(-w / F::two()).offset_y(-h / F::two()),
            width: w,
            height: h,
            resolution,
        }
    }

    pub fn render(&self) -> Vec<Vec<Vector3<F>>> {
        ParametricIterator::<F>::new(self.resolution)
            .flat_map(|(s, ss)| {
                ParametricIterator::<F>::new(self.resolution).map(move |(t, tt)| {
                    let ws: Vector3<F> = self.zero.x() * self.width * s;
                    let wss: Vector3<F> = self.zero.x() * self.width * ss;

                    let ht: Vector3<F> = self.zero.y() * self.height * t;
                    let htt: Vector3<F> = self.zero.y() * self.height * tt;

                    let a = self.zero.center + ws + ht;
                    let b = self.zero.center + wss + ht;
                    let c = self.zero.center + wss + htt;
                    let d = self.zero.center + ws + htt;
                    vec![a, b, c, d]
                })
            })
            .collect_vec()
    }
}

impl<F: Scalar> GeometryDyn<F> for Plane<F> {
    fn render(&self) -> Vec<Vec<Vector3<F>>> {
        self.render()
    }

    fn render_with_origin(&self, basis: BaseOrigin<F>) -> Vec<Vec<Vector3<F>>> {
        let mut this = self.clone();
        this.zero.apply_mut(&basis);
        this.render()
    }
}
