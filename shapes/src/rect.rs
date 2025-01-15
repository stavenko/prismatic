use math::{BaseOrigin, Scalar, Vector3};
use mesh_inter_chain::{geometry::GeometryDyn, indexes::geo_index::mesh::MeshRefMut};

pub struct Rect<S: Scalar> {
    width: S,
    height: S,
    depth: S,
    basis: BaseOrigin<S>,
}

pub enum Align {
    Neg,
    Pos,
    Middle,
}

pub struct RectBuilder<S: Scalar> {
    width: S,
    height: S,
    depth: S,
    origin: BaseOrigin<S>,
    align_z: Align,
    align_y: Align,
    align_x: Align,
}

impl<S: Scalar> RectBuilder<S> {
    pub fn width(mut self, width: impl Into<S>) -> Self {
        self.width = width.into();
        self
    }

    pub fn origin(mut self, origin: BaseOrigin<S>) -> Self {
        self.origin = origin;
        self
    }

    pub fn height(mut self, height: impl Into<S>) -> Self {
        self.height = height.into();
        self
    }

    pub fn depth(mut self, depth: impl Into<S>) -> Self {
        self.depth = depth.into();
        self
    }

    pub fn align_x(mut self, x: Align) -> Self {
        self.align_x = x;
        self
    }

    pub fn align_y(mut self, x: Align) -> Self {
        self.align_y = x;
        self
    }

    pub fn align_z(mut self, x: Align) -> Self {
        self.align_z = x;
        self
    }

    pub fn build(self) -> Rect<S> {
        let b = match self.align_x {
            Align::Neg => self.origin.offset_x(self.width / S::two()),
            Align::Pos => self.origin.offset_x(-self.width / S::two()),
            Align::Middle => self.origin,
        };

        let b = match self.align_y {
            Align::Neg => b.offset_y(self.height / S::two()),
            Align::Pos => b.offset_y(-self.height / S::two()),
            Align::Middle => b,
        };

        let b = match self.align_z {
            Align::Neg => b.offset_z(self.depth / S::two()),
            Align::Pos => b.offset_z(-self.depth / S::two()),
            Align::Middle => b,
        };

        Rect {
            width: self.width,
            height: self.height,
            depth: self.depth,
            basis: b,
        }
    }
}

impl<S: Scalar> Default for RectBuilder<S> {
    fn default() -> Self {
        Self {
            width: S::one(),
            height: S::one(),
            depth: S::one(),
            origin: Default::default(),
            align_z: Align::Middle,
            align_y: Align::Middle,
            align_x: Align::Middle,
        }
    }
}

impl<S: Scalar> Rect<S> {
    pub fn build() -> RectBuilder<S> {
        RectBuilder::default()
    }

    pub fn centered(b: BaseOrigin<S>, w: S, h: S, d: S) -> Self {
        RectBuilder::default()
            .origin(b)
            .width(w)
            .height(h)
            .depth(d)
            .build()
    }

    pub fn with_top_at(b: BaseOrigin<S>, w: S, h: S, d: S) -> Self {
        RectBuilder::default()
            .origin(b)
            .align_z(Align::Pos)
            .width(w)
            .height(h)
            .depth(d)
            .build()
    }

    pub fn with_bottom_at(b: BaseOrigin<S>, w: S, h: S, d: S) -> Self {
        Self {
            width: w,
            height: h,
            depth: d,
            basis: b.offset_z(d / S::two()),
        }
    }

    pub fn render(&self) -> Vec<Vec<Vector3<S>>> {
        let ww: Vector3<S> = self.basis.x() * (self.width / S::two());
        let hh: Vector3<S> = self.basis.y() * (self.height / S::two());
        let dd: Vector3<S> = self.basis.z() * (self.depth / S::two());

        let top = vec![
            self.basis.center + hh + ww - dd,
            self.basis.center + hh - ww - dd,
            self.basis.center + hh - ww + dd,
            self.basis.center + hh + ww + dd,
        ];
        let bottom = vec![
            self.basis.center - hh - ww + dd,
            self.basis.center - hh - ww - dd,
            self.basis.center - hh + ww - dd,
            self.basis.center - hh + ww + dd,
        ];
        let left = vec![
            self.basis.center - ww + hh + dd,
            self.basis.center - ww + hh - dd,
            self.basis.center - ww - hh - dd,
            self.basis.center - ww - hh + dd,
        ];
        let right = vec![
            self.basis.center + ww - hh + dd,
            self.basis.center + ww - hh - dd,
            self.basis.center + ww + hh - dd,
            self.basis.center + ww + hh + dd,
        ];
        let near = vec![
            self.basis.center - dd - hh + ww,
            self.basis.center - dd - hh - ww,
            self.basis.center - dd + hh - ww,
            self.basis.center - dd + hh + ww,
        ];
        let far = vec![
            self.basis.center + dd + hh + ww,
            self.basis.center + dd + hh - ww,
            self.basis.center + dd - hh - ww,
            self.basis.center + dd - hh + ww,
        ];
        vec![top, bottom, right, left, near, far]
    }
}

impl<S: Scalar> GeometryDyn<S> for Rect<S> {
    fn polygonize(&self, mut mesh: MeshRefMut<S>, _complexity: usize) -> anyhow::Result<()> {
        for p in self.render() {
            mesh.add_polygon(&p)?;
        }

        Ok(())
    }
}
