use crate::accum_ftzr::{Ftzr, IterFtzr, LinearFixed};
use crate::internal::impl_ftrzs_2;
use crate::token_from::TokenFrom;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GapGramIter<A, B, T, U1, U2> {
    a: A,
    b: B,
    gap: usize,
    data: T,
    idx: usize,
    tok2: PhantomData<(U1, U2)>,
    total_size: usize,
}

impl<'a, A, B, T, U1: 'a, U2: 'a> GapGramIter<A, B, &'a [T], U1, U2> {
    fn new<
        AF: LinearFixed + IterFtzr<&'a [T], TokenGroup = U1, Iter = A>,
        BF: LinearFixed + IterFtzr<&'a [T], TokenGroup = U2, Iter = B>,
    >(
        origin: &'a [T],
        af: &AF,
        gap: usize,
        bf: &BF,
    ) -> Self {
        let a = af.extract_tokens(origin);
        let b = bf.extract_tokens(&origin[af.chunk_size() + gap..]);
        GapGramIter {
            a,
            b,
            gap,
            idx: 0,
            data: origin,
            tok2: Default::default(),
            total_size: af.chunk_size() + gap + bf.chunk_size(),
        }
    }
}

impl<'a, A, B, T, U1, U2: 'a> Iterator for GapGramIter<A, B, &'a [T], U1, U2>
where
    A: Iterator<Item = U1>,
    B: Iterator<Item = U2>,
{
    type Item = GapPair<U1, U2>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let jdx = self.idx + self.total_size;
        if jdx <= self.data.len() {
            let a = self.a.next();
            let b = self.b.next();
            match (a, b) {
                (Some(aa), Some(bb)) => {
                    self.idx += 1;
                    return Some(GapPair(aa, bb, self.gap as u16));
                }
                _ => return None,
            }
        }
        return None;
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GapGram<A, B> {
    a: A,
    gap: usize,
    b: B,
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GapPair<A, B>(pub(crate) A, pub(crate) B, pub(crate) u16); //TODO make names fields

impl<A1, A2: From<A1>, B1, B2: From<B1>> From<GapPair<A1, B1>> for (A2, B2) {
    fn from(sp: GapPair<A1, B1>) -> Self {
        (From::from(sp.0), From::from(sp.1))
    }
}

impl<A1, A2, B1, B2, C1, C2, D1, D2> TokenFrom<GapPair<GapPair<A1, B1>, GapPair<C1, D1>>>
    for (A2, B2, C2, D2)
where
    A2: TokenFrom<A1>,
    B2: TokenFrom<B1>,
    C2: TokenFrom<C1>,
    D2: TokenFrom<D1>,
{
    fn from(t: GapPair<GapPair<A1, B1>, GapPair<C1, D1>>) -> Self {
        (
            TokenFrom::from(t.0 .0),
            TokenFrom::from(t.0 .1),
            TokenFrom::from(t.1 .0),
            TokenFrom::from(t.1 .1),
        )
    }
}

impl<A, B: TokenFrom<A>> TokenFrom<GapPair<GapPair<A, A>, GapPair<A, A>>> for [B; 4] {
    fn from(t: GapPair<GapPair<A, A>, GapPair<A, A>>) -> Self {
        [
            TokenFrom::from(t.0 .0),
            TokenFrom::from(t.0 .1),
            TokenFrom::from(t.1 .0),
            TokenFrom::from(t.1 .1),
        ]
    }
}

impl<A1, A2: TokenFrom<A1>, B1, B2: TokenFrom<B1>> TokenFrom<GapPair<A1, B1>> for (A2, B2) {
    fn from(sp: GapPair<A1, B1>) -> Self {
        (TokenFrom::from(sp.0), TokenFrom::from(sp.1))
    }
}

impl<A: LinearFixed, B: LinearFixed> LinearFixed for GapGram<A, B> {
    fn chunk_size(&self) -> usize {
        self.a.chunk_size() + self.gap + self.b.chunk_size()
    }
}

impl<'a, T: 'a, A, B, U1: 'a, U2: 'a> IterFtzr<&'a [T]> for GapGram<A, B>
where
    A: LinearFixed + IterFtzr<&'a [T], TokenGroup = U1>,
    B: LinearFixed + IterFtzr<&'a [T], TokenGroup = U2>,
{
    type TokenGroup = GapPair<U1, U2>;
    type Iter = GapGramIter<A::Iter, B::Iter, &'a [T], U1, U2>;

    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        GapGramIter::new(origin, &self.a, self.gap, &self.b)
    }
}

impl_ftrzs_2!(GapGram<X,Y>);
/*
impl<'a, A, B, U1: 'a, U2: 'a> IterFtzr<&'a str> for GapGram<A, B>
where
    A: IterFtzr<&'a [u8], TokenGroup = U1>,
    B: IterFtzr<&'a [u8], TokenGroup = U2>,
{
    type TokenGroup = GapPair<U1, U2>;
    type Iter = GapGramIter<'a, A::Iter, B::Iter, u8, U1, U2>;

    fn chunk_size(&self) -> usize {
        self.a.chunk_size() + self.gap + self.b.chunk_size()
    }

    fn extract_tokens(&self, origin: &'a str) -> Self::Iter {
        GapGramIter::new(&origin.as_bytes(), &self.a, self.gap, &self.b)
    }
}

impl<'a, A, B, U1: 'a, U2: 'a> IterFtzr<&'a String> for GapGram<A, B>
where
    A: IterFtzr<&'a [u8], TokenGroup = U1>,
    B: IterFtzr<&'a [u8], TokenGroup = U2>,
{
    type TokenGroup = GapPair<U1, U2>;
    type Iter = GapGramIter<'a, A::Iter, B::Iter, u8, U1, U2>;

    fn chunk_size(&self) -> usize {
        self.a.chunk_size() + self.gap + self.b.chunk_size()
    }

    fn extract_tokens(&self, origin: &'a String) -> Self::Iter {
        GapGramIter::new(&origin.as_bytes(), &self.a, self.gap, &self.b)
    }
}*/

pub fn gap_gram<A, B>(a: A, gap: usize, b: B) -> GapGram<A, B> {
    GapGram { a, gap, b }
}

/*
TODO implement Ftzr in terms of Ftzr, for IterFtzr
impl<'a, T, A, B, TA, TB> Ftzr<&'a [T]> for GapGram<A, B>
where
    A: Ftzr<&'a [T], TokenGroup = TA>,
    B: Ftzr<&'a [T], TokenGroup = TB>,
{
    type TokenGroup = GapPair<TA, TB>;
    fn push_tokens<Push>(&self, origin: &'a [T], push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        {
            let mut _push = |t| push(FrontBack::Front(t));
            self.front.push_tokens(origin, &mut _push);
        }
        {
            let mut _push = |t| push(FrontBack::Back(t));
            self.back.push_tokens(origin, &mut _push);
        }
    }
} */

///TODO: a user-implemented featurizer 'F' must impl  IterFtzr (not just Ftzr)
/// in order for GapGram<F,_> or GapGram<_,F> to impl Ftzr
/// ForEach and MultiFtzr do not have this limitation
/// (but BookEnds does)
/// maybe use a macro similar to internal::impl_ftrzs ??

impl<Origin, A, B> Ftzr<Origin> for GapGram<A, B>
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
/*
impl<Origin: Copy, A: Ftzr<Origin>, B: Ftzr<Origin>> Ftzr<Origin> for GapGram<A, B> {
    type TokenGroup = GapPair<A::TokenGroup, B::TokenGroup>;
    fn push_tokens<Push>(&self, origin: Origin, push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        for
    }
}
*/
