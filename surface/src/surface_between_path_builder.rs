use crate::{EdgeTensor, SurfaceBetweenPaths};
use math::Scalar;
use num_traits::{NumCast, One, ToPrimitive, Zero};
use path::GetLength;

pub struct SurfaceBetweenPathsBuilder<T: EdgeTensor> {
    leading_path: Option<path::Path<T>>,
    subdue_path: Option<path::Path<T>>,
    /// Triangles per unit of length.
    inner_points_t: usize,
    inner_points_s: usize,

    leading_path_points: Vec<T::Scalar>,
    subdue_path_path_points: Vec<T::Scalar>,
    zero_border_points: Vec<T::Scalar>,
    one_border_points: Vec<T::Scalar>,
    invert_triangles: bool,
    ignore_edge_power: bool,
    l_padding: T::Scalar,
    s_padding: T::Scalar,
    z_padding: T::Scalar,
    o_padding: T::Scalar,
}

impl<T: EdgeTensor> Default for SurfaceBetweenPathsBuilder<T> {
    fn default() -> Self {
        Self {
            leading_path: Default::default(),
            subdue_path: Default::default(),
            inner_points_t: 2,
            inner_points_s: 2,
            leading_path_points: vec![T::Scalar::zero(), T::Scalar::one()],
            subdue_path_path_points: vec![T::Scalar::zero(), T::Scalar::one()],
            zero_border_points: vec![T::Scalar::zero(), T::Scalar::one()],
            one_border_points: vec![T::Scalar::zero(), T::Scalar::one()],
            invert_triangles: Default::default(),
            l_padding: T::Scalar::from_value(0.05),
            s_padding: T::Scalar::from_value(0.05),
            z_padding: T::Scalar::from_value(0.05),
            o_padding: T::Scalar::from_value(0.05),
            ignore_edge_power: false,
        }
    }
}

impl<T: EdgeTensor> SurfaceBetweenPathsBuilder<T>
where
    T::Scalar: Scalar,
{
    pub fn build(self) -> SurfaceBetweenPaths<T> {
        SurfaceBetweenPaths {
            leading_path: self.leading_path.unwrap(),
            subdue_path: self.subdue_path.unwrap(),
            inner_points_t: self.inner_points_t,
            inner_points_s: self.inner_points_s,
            leading_path_points: self.leading_path_points,
            subdue_path_path_points: self.subdue_path_path_points,
            zero_border_points: self.zero_border_points,
            one_border_points: self.one_border_points,
            invert_triangles: self.invert_triangles,
            l_padding: self.l_padding,
            s_padding: self.s_padding,
            z_padding: self.z_padding,
            o_padding: self.o_padding,
            ignore_edge_power: self.ignore_edge_power,
        }
    }
    pub fn set_leading_path(mut self, leading_path: path::Path<T>) -> Self {
        self.leading_path = Some(leading_path);
        self
    }
    pub fn is_inverted(mut self, is_inverted: bool) -> Self {
        self.invert_triangles = is_inverted;
        self
    }

    pub fn set_subdue_path(mut self, subdue_path: path::Path<T>) -> Self {
        self.subdue_path = Some(subdue_path);
        self
    }

    pub fn set_inner_points_t(mut self, t: usize) -> Self {
        self.inner_points_t = t;
        self
    }

    pub fn set_inner_points_spacing(mut self, t: usize) -> Self {
        self.inner_points_t = t;
        self.inner_points_s = t;
        self
    }

    pub fn set_inner_points_s(mut self, s: usize) -> Self {
        self.inner_points_s = s;
        self
    }

    pub fn set_leading_path_points(mut self, count: u32) -> Self {
        assert!(count >= 2);
        self.leading_path_points.clear();
        let s = 1_f32 / (count - 1) as f32;
        for i in 0..count {
            self.leading_path_points
                .push(T::Scalar::from_value(i as f32 * s));
        }
        self
    }
    pub fn set_leading_path_points_per_component(mut self, count: u32) -> Self {
        self.leading_path_points.clear();
        self.leading_path_points = Self::points_per_path_item(
            self.leading_path
                .as_ref()
                .expect("Initialize with path before calling this function"),
            count as usize + 1,
        );
        self
    }

    pub fn set_subdue_path_points(mut self, count: u32) -> Self {
        assert!(count >= 2);
        self.subdue_path_path_points.clear();
        let s = 1_f32 / (count - 1) as f32;
        for i in 0..count {
            self.subdue_path_path_points
                .push(T::Scalar::from_value(i as f32 * s));
        }
        self
    }
    pub fn set_subdue_path_points_per_component(mut self, count: u32) -> Self {
        self.subdue_path_path_points.clear();
        self.subdue_path_path_points = Self::points_per_path_item(
            self.subdue_path
                .as_ref()
                .expect("Initialize with path before calling this function"),
            count as usize + 1,
        );
        self
    }

    pub fn set_zero_border_points(mut self, count: u32) -> Self {
        assert!(count >= 2);
        self.zero_border_points.clear();
        let s = 1_f32 / (count - 1) as f32;
        for i in 0..count {
            self.zero_border_points
                .push(T::Scalar::from_value(i as f32 * s));
        }
        self
    }

    pub fn set_one_border_points(mut self, count: u32) -> Self {
        assert!(count >= 2);
        self.one_border_points.clear();
        let s = 1_f32 / (count - 1) as f32;
        for i in 0..count {
            self.one_border_points
                .push(T::Scalar::from_value(i as f32 * s));
        }
        self
    }

    pub fn set_leading_path_padding(mut self, l_padding: impl ToPrimitive) -> Self {
        self.l_padding = <T::Scalar as NumCast>::from(l_padding).unwrap();
        self
    }

    pub fn set_subdue_path_padding(mut self, s_padding: impl ToPrimitive) -> Self {
        self.s_padding = <T::Scalar as NumCast>::from(s_padding).unwrap();
        self
    }

    pub fn set_zero_path_padding(mut self, z_padding: impl ToPrimitive) -> Self {
        self.z_padding = <T::Scalar as NumCast>::from(z_padding).unwrap();
        self
    }

    pub fn set_one_path_padding(mut self, o_padding: impl ToPrimitive) -> Self {
        self.o_padding = <T::Scalar as NumCast>::from(o_padding).unwrap();
        self
    }

    fn points_per_path_item(path: &path::Path<T>, points: usize) -> Vec<T::Scalar> {
        let mut result = Vec::new();
        let path_length = path.get_length();
        let mut prev_end = T::Scalar::zero();

        for path_item in path.iter() {
            let length = path_item.get_length();
            let param_len = length / path_length;
            for p in 0..points {
                let p_param = T::Scalar::from_value(p) / T::Scalar::from_value(points);
                let point_param = p_param * param_len;
                result.push(prev_end + point_param);
            }
            prev_end += param_len;
        }
        result.push(T::Scalar::one());

        result
    }

    pub fn set_ignore_edge_power(mut self, ignore_edge_power: bool) -> Self {
        self.ignore_edge_power = ignore_edge_power;
        self
    }
}
