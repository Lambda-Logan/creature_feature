use crate::accum_ftzr::{Ftzr, IterFtzr, LinearFixed};
use crate::feature_from::FeatureFrom;
use crate::internal::impl_ftrzs;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Whole;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WholeAtom<T>(pub T);

impl<'a, T> FeatureFrom<WholeAtom<T>> for &'a str
where
    &'a str: FeatureFrom<T>,
{
    fn from(t: WholeAtom<T>) -> &'a str {
        FeatureFrom::from(t.0)
    }
}
impl<'a, T, U> FeatureFrom<WholeAtom<T>> for &'a [U]
where
    &'a [U]: FeatureFrom<T>,
{
    fn from(t: WholeAtom<T>) -> &'a [U] {
        FeatureFrom::from(t.0)
    }
}
impl<'a, T, U, const N: usize> FeatureFrom<WholeAtom<T>> for &'a [U; N]
where
    &'a [U; N]: FeatureFrom<T>,
{
    fn from(t: WholeAtom<T>) -> &'a [U; N] {
        FeatureFrom::from(t.0)
    }
}
impl<T, U, const N: usize> FeatureFrom<WholeAtom<T>> for [U; N]
where
    [U; N]: FeatureFrom<T>,
{
    fn from(t: WholeAtom<T>) -> [U; N] {
        FeatureFrom::from(t.0)
    }
}

impl<T> FeatureFrom<WholeAtom<T>> for String
where
    String: FeatureFrom<T>,
{
    fn from(t: WholeAtom<T>) -> String {
        FeatureFrom::from(t.0)
    }
}

impl<T, Q: IntoIterator<Item = T>> FeatureFrom<WholeAtom<Q>> for Vec<T> {
    fn from(t: WholeAtom<Q>) -> Vec<T> {
        Iterator::collect(t.0.into_iter())
    }
}

impl<'a, T> IterFtzr<&'a [T]> for Whole {
    type TokenGroup = WholeAtom<&'a [T]>;
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn iterate_features(&self, origin: &'a [T]) -> Self::Iter {
        Some(WholeAtom(origin)).into_iter()
    }
}
impl_ftrzs!(Whole);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Empty;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EmptyAtom;

impl<D: Default> FeatureFrom<EmptyAtom> for D {
    fn from(x: EmptyAtom) -> D {
        Default::default()
    }
}

impl LinearFixed for Empty {
    fn chunk_size(&self) -> usize {
        0
    }
}

impl<'a, T> IterFtzr<&'a [T]> for Empty {
    type TokenGroup = EmptyAtom;
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn iterate_features(&self, origin: &'a [T]) -> Self::Iter {
        None.into_iter()
    }
}
pub fn whole() -> Whole {
    Whole
}
pub fn empty() -> Empty {
    Empty
}
impl_ftrzs!(Empty);
