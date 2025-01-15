use core::fmt;
use std::ops::Sub;

use num_traits::{One, Pow};

pub(crate) fn bernstein<F>(item: usize, of: usize, t: F) -> F
where
    F: One + Sub<F, Output = F> + Pow<u16, Output = F> + From<u16> + Copy + fmt::Debug,
{
    let opt = of - 1;
    let factor = (fact(opt) / (fact(item) * fact(opt - item))) as u16;
    let ot = F::one() - t;
    let o_item = opt - item;

    t.pow(item as u16) * ot.pow(o_item as u16) * factor.into()
}

const fn fact(i: usize) -> usize {
    match i {
        0 => 1,
        1 => 1,
        2 => 2,
        x => x * fact(x - 1),
    }
}
