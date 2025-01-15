use math::Scalar;

use super::index::GeoIndex;

pub trait GeoObject<'a, S: Scalar> {
    type Ref: UnRef<'a, S>;
    type MutRef: UnRef<'a, S>;

    fn make_ref(&self, index: &'a GeoIndex<S>) -> Self::Ref;
    fn make_mut_ref(&self, index: &'a mut GeoIndex<S>) -> Self::MutRef;
}

pub trait UnRef<'a, S: Scalar> {
    type Obj: GeoObject<'a, S>;

    fn un_ref(self) -> Self::Obj;
}
