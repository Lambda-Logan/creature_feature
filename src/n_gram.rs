use crate::accum_ftzr::{Ftzr, IterFtzr, LinearFixed};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

/// The type of a fixed-length n-gram over copied data. Created by `n_gram::<N>()`
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NGram<const N: usize>();

/// The associated iterator for `<NGram<N> as IterFtzr<T>>::Iter`
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NGramIter<'a, T, const N: usize> {
    idx: usize,
    data: &'a [T],
}

impl<'a, T: 'a, const N: usize> Iterator for NGramIter<'a, T, N>
where
    [T; N]: TryFrom<&'a [T]>,
{
    type Item = [T; N];
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let jdx = self.idx + N;
        if jdx <= self.data.len() {
            let ret = Some(
                TryInto::try_into(&self.data[self.idx..jdx])
                    .map_err(|_| ())
                    .expect("Error converting from slice to [T;N]"),
            );
            self.idx += 1;
            ret
        } else {
            None
        }
    }
}

impl<'a, T, const N: usize> IterFtzr<&'a [T]> for NGram<N>
where
    [T; N]: TryFrom<&'a [T]>,
{
    type TokenGroup = [T; N];
    type Iter = NGramIter<'a, T, N>;

    fn iterate_features(&self, origin: &'a [T]) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: origin,
        }
    }
}

impl<const N: usize> LinearFixed for NGram<N> {
    fn chunk_size(&self) -> usize {
        N
    }
}

impl<'a, T, const N: usize> IterFtzr<&'a Vec<T>> for NGram<N>
where
    [T; N]: TryFrom<&'a [T]>,
{
    type TokenGroup = [T; N];
    type Iter = NGramIter<'a, T, N>;

    fn iterate_features(&self, origin: &'a Vec<T>) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: origin.as_slice(),
        }
    }
}

impl<'a, const N: usize> IterFtzr<&'a str> for NGram<N>
where
    [u8; N]: TryFrom<&'a [u8]>,
{
    type TokenGroup = [u8; N];
    type Iter = NGramIter<'a, u8, N>;

    fn iterate_features(&self, origin: &'a str) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: origin.as_bytes(),
        }
    }
}

impl<'a, const N: usize> IterFtzr<&'a String> for NGram<N>
where
    [u8; N]: TryFrom<&'a [u8]>,
{
    type TokenGroup = [u8; N];
    type Iter = NGramIter<'a, u8, N>;

    fn iterate_features(&self, origin: &'a String) -> Self::Iter {
        self.iterate_features(origin.as_str())
    }
}

impl<'a, T, const N: usize, const M: usize> IterFtzr<&'a [T; M]> for NGram<N>
where
    [T; N]: TryFrom<&'a [T]>,
{
    type TokenGroup = [T; N];
    type Iter = NGramIter<'a, T, N>;

    fn iterate_features(&self, origin: &'a [T; M]) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: &origin[..],
        }
    }
}

/// general n-grams over copied data, produces owned data (like String) or multiple `[T; N]`. (Compare to `n_slice`)
/// ```
/// use creature_feature::ftzrs::n_gram;
///
/// let my_ftzr = n_gram::<7>();
///
/// let feats: Vec<[T; 7]> = my_ftzr.featurize(my_data);
/// let feats: Vec<String> = my_ftzr.featurize(my_other_data);
/// ```
pub fn n_gram<const N: usize>() -> NGram<N> {
    NGram::<N>()
}

/// bigrams over copied data, produces owned data (like String) or multiple `[T; 2]`. (Compare to `bislice`)
pub fn bigram() -> NGram<2> {
    NGram::<2>()
}

/// trigrams over copied data, produces owned data (like String) or multiple `[T; 3]`. (Compare to `trislice`)
pub fn trigram() -> NGram<3> {
    NGram::<3>()
}

impl<Origin, const N: usize> Ftzr<Origin> for NGram<N>
where
    Self: IterFtzr<Origin>,
{
    type TokenGroup = <Self as IterFtzr<Origin>>::TokenGroup;
    fn push_tokens<Push>(&self, origin: Origin, push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        for t in self.iterate_features(origin) {
            push(t)
        }
    }
}
