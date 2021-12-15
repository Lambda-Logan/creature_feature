use crate::accum_ftzr::{Ftzr, IterFtzr};
//use crate::tokengroup::TokenGroup;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct SliceGram {
    n: usize,
}

pub fn slice_gram(n: usize) -> SliceGram {
    SliceGram { n: n }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
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
    fn extract_tokens(&self, origin: Origin) -> Self::Iter {
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
            fn extract_tokens(&self, origin: &'a $t) -> Self::Iter {
                SliceGramIter {
                    n: self.n,
                    idx: 0,
                    data: $map_origin(origin),
                }
            }
        }
    };
} */

impl<'a, T> IterFtzr<&'a [T]> for SliceGram {
    type TokenGroup = &'a [T];
    type Iter = SliceGramIter<&'a [T]>;
    fn chunk_size(&self) -> usize {
        self.n
    }
    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: origin,
        }
    }
}

impl<'a, T> IterFtzr<&'a Vec<T>> for SliceGram {
    type TokenGroup = &'a [T];
    type Iter = SliceGramIter<&'a [T]>;
    fn chunk_size(&self) -> usize {
        self.n
    }
    fn extract_tokens(&self, origin: &'a Vec<T>) -> Self::Iter {
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
    fn extract_tokens(&self, origin: &'a str) -> Self::Iter {
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
    fn extract_tokens(&self, origin: &'a [T; N]) -> Self::Iter {
        SliceGramIter {
            n: self.n,
            idx: 0,
            data: origin,
        }
    }
}

impl<Origin> Ftzr<Origin> for SliceGram
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
