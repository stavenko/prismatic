use core::fmt;
use math::ParametricIterator;
use std::ops::Sub;

use math::{CrossProduct, Tensor, Vector3};
use num_traits::{One, Pow, Zero};

use crate::{
    get_length::GetLength,
    get_t::GetT,
    lerp::Lerp,
    path_item::PathItem,
    shift_in_plane::{GetPosition, ShiftInPlane},
    update_start_end::UpdateStartEnd,
};

#[derive(Clone, Debug)]
pub struct Curve<T>(Vec<T>);

impl<T> From<Curve<T>> for PathItem<T> {
    fn from(value: Curve<T>) -> Self {
        PathItem::Curve(value)
    }
}

impl<T> Curve<T> {
    pub fn new_2(t1: T, t2: T) -> Self {
        Self(vec![t1, t2])
    }
    pub fn new_3(t1: T, t2: T, t3: T) -> Self {
        Self(vec![t1, t2, t3])
    }
    pub fn new_4(t1: T, t2: T, t3: T, t4: T) -> Self {
        Self(vec![t1, t2, t3, t4])
    }

    pub fn for_each_point(mut self, map: impl Fn(T) -> T) -> Self {
        self.0 = self.0.into_iter().map(map).collect();
        self
    }
}

impl<T> GetT for Curve<T>
where
    T: Tensor,
{
    type Tensor = T;

    fn get_t(&self, t: <Self::Tensor as Tensor>::Scalar) -> Self::Tensor {
        assert!(t >= T::Scalar::zero());
        assert!(t <= T::Scalar::one());
        let o = self.0.len();
        let ws = (0..o).map(|i| bernstein::<_>(i, o, t));
        let v: Vec<T> = ws.zip(&self.0).map(|(w, t)| *t * w).collect::<Vec<_>>();
        v.into_iter().fold(T::zero(), |a, t| a + t)
    }
}

impl<T: Tensor> GetLength for Curve<T> {
    type Tensor = T;

    fn get_length(&self) -> <Self::Tensor as Tensor>::Scalar {
        if self.0.len() == 2 {
            let distance = self.0[0] - self.0[1];
            distance.magnitude()
        } else {
            let mut total = T::Scalar::zero();
            for (t, tt) in ParametricIterator::<T::Scalar>::new(10) {
                let t0 = self.get_t(t);
                let t1 = self.get_t(tt);
                total += (t0 - t1).magnitude();
            }
            total
        }
    }
}

impl<T: Tensor> UpdateStartEnd for Curve<T> {
    type Tensor = T;

    fn update_start(&mut self, start: Self::Tensor) {
        if let Some(s) = self.0.first_mut() {
            *s = start;
        }
    }

    fn update_end(&mut self, end: Self::Tensor) {
        if let Some(e) = self.0.last_mut() {
            *e = end;
        }
    }
}

pub(crate) fn bernstein<F>(item: usize, of: usize, t: F) -> F
where
    F: One + Sub<F, Output = F> + Pow<usize, Output = F> + Copy + fmt::Debug + From<usize>,
{
    let opt = of - 1;
    let factor: F = (fact(opt) / (fact(item) * fact(opt - item))).into();
    let ot = F::one() - t;
    let o_item = opt - item;

    t.pow(item) * ot.pow(o_item) * factor
}

const fn fact(i: usize) -> usize {
    match i {
        0 => 1,
        1 => 1,
        2 => 2,
        x => x * fact(x - 1),
    }
}

impl<T> ShiftInPlane for Curve<T>
where
    T: Tensor
        + Lerp<Scalar = <T as Tensor>::Scalar>
        + GetPosition<Position = Vector3<<T as Tensor>::Scalar>>,
    Vector3<<T as Tensor>::Scalar>:
        CrossProduct<Vector3<<T as Tensor>::Scalar>, Output = Vector3<<T as Tensor>::Scalar>>,
{
    type Vector3 = Vector3<<T as Tensor>::Scalar>;
    type Scalar = <T as Tensor>::Scalar;

    fn shift_in_plane(mut self, normal: Self::Vector3, amount: <T as Tensor>::Scalar) -> Self {
        let delta = <T as Tensor>::Scalar::one() / <T as Tensor>::Scalar::from(65535);
        let half = <T as Tensor>::Scalar::one() / <T as Tensor>::Scalar::from(2);
        let b = self.get_t(<T as Tensor>::Scalar::zero()).get_position();
        let bb = self.get_t(delta).get_position();
        let e = self
            .get_t(<T as Tensor>::Scalar::one() - delta)
            .get_position();
        let ee = self.get_t(<T as Tensor>::Scalar::one()).get_position();
        let dir_b = (bb - b).normalize();
        let dir_e = (ee - e).normalize();
        let ext_b = dir_b.cross_product(&normal);
        let ext_e = dir_e.cross_product(&normal);

        if self.0.len() % 2 == 0 {
            let center = self.0.len() / 2;
            for (ix, p) in self.0.iter_mut().enumerate() {
                if ix < center {
                    *p.get_position_mut() += ext_b * amount;
                } else {
                    *p.get_position_mut() += ext_e * amount;
                }
            }
        } else {
            let center = self.0.len() / 2;
            for (ix, p) in self.0.iter_mut().enumerate() {
                match ix.cmp(&center) {
                    std::cmp::Ordering::Less => *p.get_position_mut() += ext_b * amount,
                    std::cmp::Ordering::Equal => {
                        *p.get_position_mut() += (ext_b.lerp(&ext_e, half)) * amount;
                    }
                    std::cmp::Ordering::Greater => *p.get_position_mut() += ext_e * amount,
                }
            }
        }

        self
    }
}
