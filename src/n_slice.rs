use crate::accum_ftzr::{Ftzr, IterFtzr, LinearFixed};

use crate::internal::impl_ftrzs;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A fixed-length n-gram over referenced data. Created with `n_slice(n)`
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SliceGram {
    n: usize,
}

/// general n-grams over referenced data, produces borrowed data (like &str) or multiple `&[T]` of a fixed length. (Compare to `n_gram`)
/// ```
/// use creature_feature::ftzrs::n_slice;
///
/// let my_ftzr = n_slice(7);
///
/// let feats: Vec<&[T]> = my_ftzr.featurize(my_data);
/// let feats: Vec<&str> = my_ftzr.featurize(my_other_data);
/// ```
pub fn n_slice(n: usize) -> SliceGram {
    SliceGram { n }
}

/// bigrams over referenced data, produces borrowed data (like &str) or multiple `&[T]` of length 2. (Compare to `bigram`)
pub fn bislice() -> SliceGram {
    SliceGram { n: 2 }
}

/// trigrams over referenced data, produces borrowed data (like &str) or multiple `&[T]` of length 3. (Compare to `trigram`)
pub fn trislice() -> SliceGram {
    SliceGram { n: 3 }
}

/// The associated iterator for SliceGram as IterFtzr<T>>::Iter
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SliceGramIter<Origin> {
    n: usize,
    idx: usize,
    data: Origin,
}

impl<'a, T: 'a> Iterator for SliceGramIter<&'a [T]> {
    type Item = &'a [T];
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let jdx = self.idx + self.n;
        if jdx <= self.data.len() {
            let ret = Some(&self.data[self.idx..jdx]);
            self.idx += 1;
            ret
        } else {
            None
        }
    }
}

/*
impl<'a, Origin, T: 'a> IterFtzr<Origin> for SliceGram
where
    Origin: Deref<Target = &'a [T]>,
{
    type TokenGroup = TokenGroup<&'a [T]>;
    type Iter = SliceGramIter<'a, T>;
    fn chunk_size(&self) -> usize {
        self.n
    }
    fn iterate_features(&self, origin: Origin) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: &*origin,
        }
    }
} */
/*
macro_rules! impl_slice_gram {
    ($($params:ty),*; $t:ty, $inner:ty, $map_origin:ident) => {
        impl<'a, $($params),*> IterFtzr<&'a $t> for SliceGram {
            type TokenGroup = TokenGroup<&'a $t>;
            type Iter = SliceGramIter<&'a [$inner]>;
            fn chunk_size(&self) -> usize {
                self.n
            }
            fn iterate_features(&self, origin: &'a $t) -> Self::Iter {
                SliceGramIter {
                    n: self.n,
                    idx: 0,
                    data: $map_origin(origin),
                }
            }
        }
    };
} */
impl LinearFixed for SliceGram {
    fn chunk_size(&self) -> usize {
        self.n
    }
}
impl<'a, T> IterFtzr<&'a [T]> for SliceGram {
    type TokenGroup = &'a [T];
    type Iter = SliceGramIter<&'a [T]>;

    fn iterate_features(&self, origin: &'a [T]) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: origin,
        }
    }
}

impl_ftrzs!(SliceGram);

/*
impl<'a, T> IterFtzr<&'a Vec<T>> for SliceGram {
    type TokenGroup = &'a [T];
    type Iter = SliceGramIter<&'a [T]>;
    fn chunk_size(&self) -> usize {
        self.n
    }
    fn iterate_features(&self, origin: &'a Vec<T>) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: origin.as_slice(),
        }
    }
}

impl<'a> IterFtzr<&'a str> for SliceGram {
    type TokenGroup = &'a [u8];
    type Iter = SliceGramIter<&'a [u8]>;
    fn chunk_size(&self) -> usize {
        self.n
    }
    fn iterate_features(&self, origin: &'a str) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: origin.as_bytes(),
        }
    }
}

impl<'a, T, const N: usize> IterFtzr<&'a [T; N]> for SliceGram {
    type TokenGroup = &'a [T];
    type Iter = SliceGramIter<&'a [T]>;
    fn chunk_size(&self) -> usize {
        self.n
    }
    fn iterate_features(&self, origin: &'a [T; N]) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: origin,
        }
    }
}*/
