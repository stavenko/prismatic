use num_traits::Zero;

use crate::{curve::Curve, path::Path, path_item::PathItem};

pub struct PathBuilder<T> {
    last: T,
    items: Vec<PathItem<T>>,
}

impl<T> PathBuilder<T> {
    pub fn start(mut self, t: T) -> Self {
        self.last = t;
        self
    }

    pub fn line_to(mut self, t: T) -> Self
    where
        T: Copy,
    {
        let last = std::mem::replace(&mut self.last, t);
        self.items.push(Curve::new_2(last, t).into());
        self
    }

    pub fn quad_3_to(mut self, weight: T, last: T) -> Self
    where
        T: Copy,
    {
        let first = std::mem::replace(&mut self.last, last);
        let quad = Curve::new_3(first, weight, last);
        self.items.push(quad.into());
        self
    }

    pub fn quad_4_to(mut self, weight: T, weight2: T, last: T) -> Self
    where
        T: Copy,
    {
        let first = std::mem::replace(&mut self.last, last);
        let quad = Curve::new_4(first, weight, weight2, last);
        self.items.push(quad.into());
        self
    }

    pub fn build(mut self) -> Path<T> {
        Path {
            items: std::mem::take(&mut self.items),
        }
    }
}

impl<T> Default for PathBuilder<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self {
            last: T::zero(),
            items: Default::default(),
        }
    }
}
