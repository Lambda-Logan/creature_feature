use crate::accum_ftzr::{Ftzr, IterFtzr};
use crate::feature_from::FeatureFrom;
use crate::internal::impl_ftrzs_2;
use crate::n_gram::NGram;
use crate::tokengroup::Token;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BookEnds<A, B> {
    front: A,
    back: B,
    front_size: usize,
    back_size: usize,
}

pub fn bookends<A, B>(front: (A, usize), back: (B, usize)) -> BookEnds<A, B> {
    BookEnds {
        front: front.0,
        front_size: front.1,
        back_size: back.1,
        back: back.0,
    }
}

impl<'a, T: 'a, TA: 'a, TB: 'a, A, B> IterFtzr<&'a [T]> for BookEnds<A, B>
where
    A: IterFtzr<&'a [T], TokenGroup = TA>,
    B: IterFtzr<&'a [T], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    type Iter = BookEndsIter<A::Iter, B::Iter>;

    fn iterate_features(&self, origin: &'a [T]) -> Self::Iter {
        BookEndsIter(
            true,
            self.front.iterate_features(&origin[..self.front_size]),
            self.back
                .iterate_features(&origin[origin.len() - self.back_size..]),
        )
    }
}
/*
impl<'a, TA: 'a, TB: 'a, A, B> IterFtzr<&'a str> for BookEnds<A, B>
where
    A: IterFtzr<&'a [u8], TokenGroup = TA>,
    B: IterFtzr<&'a [u8], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    type Iter = BookEndsIter<A::Iter, B::Iter>;

    fn iterate_features(&self, origin: &'a str) -> Self::Iter {
        self.iterate_features(origin.as_bytes())
    }
}

impl<'a, TA: 'a, TB: 'a, A, B> IterFtzr<&'a String> for BookEnds<A, B>
where
    A: IterFtzr<&'a [u8], TokenGroup = TA>,
    B: IterFtzr<&'a [u8], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    type Iter = BookEndsIter<A::Iter, B::Iter>;

    fn iterate_features(&self, origin: &'a String) -> Self::Iter {
        self.iterate_features(origin.as_str())
    }
}

impl<'a, T: 'a, TA: 'a, TB: 'a, A, B, const N: usize> IterFtzr<&'a [T; N]> for BookEnds<A, B>
where
    A: IterFtzr<&'a [T], TokenGroup = TA>,
    B: IterFtzr<&'a [T], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    type Iter = BookEndsIter<A::Iter, B::Iter>;

    fn iterate_features(&self, origin: &'a [T; N]) -> Self::Iter {
        self.iterate_features(&origin[..])
    }
}

impl<'a, T: 'a, TA: 'a, TB: 'a, A, B> IterFtzr<&'a Vec<T>> for BookEnds<A, B>
where
    A: IterFtzr<&'a [T], TokenGroup = TA>,
    B: IterFtzr<&'a [T], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    type Iter = BookEndsIter<A::Iter, B::Iter>;

    fn iterate_features(&self, origin: &'a Vec<T>) -> Self::Iter {
        self.iterate_features(origin.as_slice())
    }
}
*/

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BookEndsIter<A, B>(bool, A, B);

impl<A, B> Iterator for BookEndsIter<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = FrontBack<A::Item, B::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            // it's still in the first section
            match self.1.next() {
                None => {
                    self.0 = false;
                    self.2.next().map(FrontBack::Back)
                }
                otherwise => otherwise.map(FrontBack::Front),
            }
        } else {
            // it's in the last section
            self.2.next().map(FrontBack::Back)
        }
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FrontBack<A, B> {
    Front(A),
    Back(B),
}

impl<A, B, C> FeatureFrom<FrontBack<A, B>> for Token<C>
where
    C: FeatureFrom<A> + FeatureFrom<B>,
{
    fn from(x: FrontBack<A, B>) -> Self {
        Token(match x {
            FrontBack::Front(a) => FeatureFrom::from(a),
            FrontBack::Back(a) => FeatureFrom::from(a),
        })
    }
}

impl_ftrzs_2!(BookEnds<X,Y>);

impl<A, B, Ax, Bx> FeatureFrom<FrontBack<A, B>> for Result<Ax, Bx>
where
    Ax: FeatureFrom<A>,
    Bx: FeatureFrom<B>,
{
    fn from(x: FrontBack<A, B>) -> Self {
        match x {
            FrontBack::Front(a) => Ok(FeatureFrom::from(a)),
            FrontBack::Back(a) => Err(FeatureFrom::from(a)),
        }
    }
}
/*

TODO impl in terms of Ftzr, not IterFtzr

impl<'a, T, A, B, TA, TB> Ftzr<&'a [T]> for BookEnds<A, B>
where
    A: Ftzr<&'a [T], TokenGroup = TA>,
    B: Ftzr<&'a [T], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    fn push_tokens<Push>(&self, origin: &'a [T], push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        {
            let mut _push = |t| push(FrontBack::Front(t));
            self.front
                .push_tokens(&origin[..self.front_size], &mut _push);
        }
        {
            let mut _push = |t| push(FrontBack::Back(t));
            self.back
                .push_tokens(&origin[origin.len() - self.back_size..], &mut _push);
        }
    }
} */

impl<Origin, A, B> Ftzr<Origin> for BookEnds<A, B>
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
