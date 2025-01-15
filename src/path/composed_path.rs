use std::{collections::VecDeque, marker::PhantomData};

use super::path_trait::Path;

pub struct ComposedPath<Tensor, Scalar> {
    items: VecDeque<Box<dyn Path<Tensor, Scalar>>>,
    _phs: PhantomData<Scalar>,
    _pht: PhantomData<Tensor>,
}
