use crate::accum_ftzr::{Ftzr, IterFtzr};
use crate::convert::Merged;
use crate::feature_from::FeatureFrom;
use crate::internal::impl_ftrzs_2;
use crate::n_gram::NGram;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// `Bookends<A,B>` is a featurizer combinator that will run 'A' on the beggining of the data and run 'B' on the end of the data.
/// Its main purpose is to make it easier to handle prefixes and suffices. Created by `bookends`
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BookEnds<A, B> {
    front: A,
    back: B,
    front_size: usize,
    back_size: usize,
}

/// A tool to featurize prefixes and suffixes. `bookends((a, n), (b, m))` will run featurizer `a` for the first `n` tokens and will run featurizer `b` for the last `n` tokens. All tokens between are skipped.
/// # Example
/// ```
///use creature_feature::ftzrs::misc::FrontBack;
///use creature_feature::ftzrs::{bislice, bookends, trigram};
///use creature_feature::traits::Ftzr;
///
///
///let ftzr = bookends((bislice(), 4), (trigram(), 4));
///
///let feats: Vec<FrontBack<&str, String>> = ftzr.featurize("sesquipedalian");
///
///println!("{:?}", feats);
///
/// //>>> [Front("se"), Front("es"), Front("sq"), Back("lia"), Back("ian")]
/// ```
///
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

/// The associated Iterator for the [`IterFtzr`] implementation of [`BookEnds`]
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

/// This is the TokenGroup for [`BookEnds`]. We need to differentiate between features at the front of a word and features at the back of a word. Otherwise suffixes and prefixes would be identical.
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FrontBack<A, B> {
    /// the TokenGroup from running `A` on the front of the input
    Front(A),
    /// the TokenGroup from running `B` on the Back of the input
    Back(B),
}

impl<A, B, C> FeatureFrom<FrontBack<A, B>> for Merged<C>
where
    C: FeatureFrom<A> + FeatureFrom<B>,
{
    fn from(x: FrontBack<A, B>) -> Self {
        Merged(match x {
            FrontBack::Front(a) => FeatureFrom::from(a),
            FrontBack::Back(a) => FeatureFrom::from(a),
        })
    }
}
/*
impl<A, W, C> FeatureFrom<FrontBack<A, W>> for C
where
    C: FeatureFrom<A> + FeatureFrom<W::Target>,
    W: Deref,
    W::Target: Sized,
{
    fn from(x: FrontBack<A, W>) -> Self {
        match x {
            FrontBack::Front(a) => FeatureFrom::from(a),
            FrontBack::Back(a) => FeatureFrom::from(&*a),
        }
    }
} */

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

impl<A, B, Ax, Bx> FeatureFrom<FrontBack<A, B>> for FrontBack<Ax, Bx>
where
    Ax: FeatureFrom<A>,
    Bx: FeatureFrom<B>,
{
    fn from(x: FrontBack<A, B>) -> Self {
        match x {
            FrontBack::Front(a) => FrontBack::Front(FeatureFrom::from(a)),
            FrontBack::Back(a) => FrontBack::Back(FeatureFrom::from(a)),
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
