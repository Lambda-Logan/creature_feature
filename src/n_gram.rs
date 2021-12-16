use crate::accum_ftzr::{Ftzr, IterFtzr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NGram<const N: usize>();

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
    fn chunk_size(&self) -> usize {
        N
    }
    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: origin,
        }
    }
}

impl<'a, T, const N: usize> IterFtzr<&'a Vec<T>> for NGram<N>
where
    [T; N]: TryFrom<&'a [T]>,
{
    type TokenGroup = [T; N];
    type Iter = NGramIter<'a, T, N>;
    fn chunk_size(&self) -> usize {
        N
    }
    fn extract_tokens(&self, origin: &'a Vec<T>) -> Self::Iter {
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
    fn chunk_size(&self) -> usize {
        N
    }
    fn extract_tokens(&self, origin: &'a str) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: origin.as_bytes(),
        }
    }
}

impl<'a, T, const N: usize> IterFtzr<&'a [T; N]> for NGram<N>
where
    [T; N]: TryFrom<&'a [T]>,
{
    type TokenGroup = [T; N];
    type Iter = NGramIter<'a, T, N>;
    fn chunk_size(&self) -> usize {
        N
    }
    fn extract_tokens(&self, origin: &'a [T; N]) -> Self::Iter {
        NGramIter {
            idx: 0,
            data: &origin[..],
        }
    }
}

pub fn n_gram<const N: usize>() -> NGram<N> {
    NGram::<N>()
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
        for t in self.extract_tokens(origin) {
            push(t)
        }
    }
}
