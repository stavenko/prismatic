use crate::{
    get_length::GetLength, get_t::GetT, path_builder::PathBuilder, path_item::PathItem,
    update_start_end::UpdateStartEnd,
};
use math::{Scalar, Tensor};
use num_traits::{Float, One, Zero};

#[derive(Clone, Debug)]
pub struct Path<T> {
    pub(crate) items: Vec<PathItem<T>>,
}

impl<T> Default for Path<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}

impl<T> Path<T> {
    pub fn push_back(mut self, item: impl Into<PathItem<T>>) -> Self
    where
        T: Tensor,
        PathItem<T>: GetLength<Tensor = T>,
    {
        let item = item.into();
        if item.get_length().is_zero() {
            println!("TRYING to put zero length element");
            self
        } else {
            self.items.push(item);
            self
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> GetLength for Path<T>
where
    T: Tensor,
{
    type Tensor = T;

    fn get_length(&self) -> <Self::Tensor as Tensor>::Scalar {
        self.items
            .iter()
            .map(|i| i.get_length())
            .fold(T::Scalar::zero(), |a, f| a + f)
    }
}

impl<T> GetT for Path<T>
where
    T: Tensor,
{
    type Tensor = T;

    fn get_t(&self, mut t: <Self::Tensor as Tensor>::Scalar) -> Self::Tensor {
        let max_delta = T::Scalar::one() / T::Scalar::from_value(1_000_000);

        let delta = (T::Scalar::zero() - t).abs();
        if t < T::Scalar::zero() {
            if delta < max_delta {
                t = T::Scalar::zero();
            } else {
                panic!(
                "Difference too big on zero side: {t:?}, delta: {delta:?}, max_delta: {max_delta:?}"
            );
            }
        }

        let delta = (T::Scalar::one() - t).abs();
        if t > T::Scalar::one() {
            if delta < max_delta {
                t = T::Scalar::one();
            } else {
                panic!(
                "Difference too big on one side: {t:?}, delta: {delta:?}, max_delta: {max_delta:?}"

            );
            }
        }

        //println!("Try to get t: {t:?}");
        let total_len = self.get_length();
        //println!("total len: {total_len:?}/ items: {:?}", self.items.len());

        for item in &self.items {
            let item_len = item.get_length();
            let mut item_param_len = item_len / total_len;
            let delta = (t - item_param_len).abs();
            if delta < max_delta {
                item_param_len += delta;
            }
            /*
            println!(
                "  item_len: {item_len:?}, param_len: {item_param_len:?}, t: {t:?}, d: {delta:?}"
            );
            */
            if item_param_len < t {
                t -= item_param_len;
            } else {
                let inline_param = t / item_param_len;
                return item.get_t(inline_param);
            }
        }

        unreachable!("Cannot get here: t left = {t:?}")
    }
}

impl<V> FromIterator<PathItem<V>> for Path<V> {
    fn from_iter<T: IntoIterator<Item = PathItem<V>>>(iter: T) -> Self {
        let items = iter.into_iter().collect::<Vec<_>>();
        Self { items }
    }
}
impl<T> IntoIterator for Path<T> {
    type Item = PathItem<T>;

    type IntoIter = <Vec<PathItem<T>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: Tensor> Path<T> {
    pub fn iter(&self) -> impl Iterator<Item = &PathItem<T>> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PathItem<T>> {
        self.items.iter_mut()
    }

    pub fn build() -> PathBuilder<T> {
        PathBuilder::default()
    }

    pub fn connect_ends(&mut self) {
        let half = <T as Tensor>::Scalar::one() / <T as Tensor>::Scalar::two();
        for cur in 0..(self.items.len() - 1) {
            let next = cur + 1;
            let f = self.items[cur].get_t(T::Scalar::one());
            let l = self.items[next].get_t(T::Scalar::zero());
            let d = l - f;
            let d = d * half;
            let m = d + f;

            self.items[cur].update_end(m);
            self.items[next].update_start(m);
        }
    }

    pub fn connect_ends_circular(&mut self) {
        let half = <T as Tensor>::Scalar::one() / <T as Tensor>::Scalar::two();
        for cur in 0..(self.items.len()) {
            let next = (cur + 1) % self.items.len();
            let f = self.items[cur].get_t(T::Scalar::one());
            let l = self.items[next].get_t(T::Scalar::zero());
            let d = l - f;
            let d = d * half;
            let m = d + f;

            self.items[cur].update_end(m);
            self.items[next].update_start(m);
        }
    }
}
