use crate::accum_ftzr::{Ftzr, IterFtzr};
use crate::internal::impl_ftrzs;
use crate::token_from::TokenFrom;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Whole;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WholeAtom<T>(pub T);

impl<'a, T> TokenFrom<WholeAtom<T>> for &'a str
where
    &'a str: TokenFrom<T>,
{
    fn from(t: WholeAtom<T>) -> &'a str {
        TokenFrom::from(t.0)
    }
}
impl<'a, T, U> TokenFrom<WholeAtom<T>> for &'a [U]
where
    &'a [U]: TokenFrom<T>,
{
    fn from(t: WholeAtom<T>) -> &'a [U] {
        TokenFrom::from(t.0)
    }
}
impl<'a, T, U, const N: usize> TokenFrom<WholeAtom<T>> for &'a [U; N]
where
    &'a [U; N]: TokenFrom<T>,
{
    fn from(t: WholeAtom<T>) -> &'a [U; N] {
        TokenFrom::from(t.0)
    }
}
impl<T, U, const N: usize> TokenFrom<WholeAtom<T>> for [U; N]
where
    [U; N]: TokenFrom<T>,
{
    fn from(t: WholeAtom<T>) -> [U; N] {
        TokenFrom::from(t.0)
    }
}

impl<T> TokenFrom<WholeAtom<T>> for String
where
    String: TokenFrom<T>,
{
    fn from(t: WholeAtom<T>) -> String {
        TokenFrom::from(t.0)
    }
}

impl<T, Q: IntoIterator<Item = T>> TokenFrom<WholeAtom<Q>> for Vec<T> {
    fn from(t: WholeAtom<Q>) -> Vec<T> {
        Iterator::collect(t.0.into_iter())
    }
}

impl<'a, T> IterFtzr<&'a [T]> for Whole {
    type TokenGroup = WholeAtom<&'a [T]>;
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn chunk_size(&self) -> usize {
        unimplemented!()
    }
    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        Some(WholeAtom(origin)).into_iter()
    }
}
impl_ftrzs!(Whole);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Empty;

impl<'a, T> IterFtzr<&'a [T]> for Empty {
    type TokenGroup = [T; 1];
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn chunk_size(&self) -> usize {
        unimplemented!()
    }
    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
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
