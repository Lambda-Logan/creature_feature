use crate::accum_ftzr::{Ftzr, IterFtzr, LinearFixed};
use crate::feature_from::FeatureFrom;
use crate::internal::impl_ftrzs_2;
use crate::multiftzr::EitherGroup;
use crate::skip_schema::SkipSchema;

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
        let a = af.iterate_features(origin);
        let b = bf.iterate_features(&origin[af.chunk_size() + gap..]);
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

impl<A1, A2, B1, B2, C1, C2, D1, D2> FeatureFrom<GapPair<GapPair<A1, B1>, GapPair<C1, D1>>>
    for (A2, B2, C2, D2)
where
    A2: FeatureFrom<A1>,
    B2: FeatureFrom<B1>,
    C2: FeatureFrom<C1>,
    D2: FeatureFrom<D1>,
{
    fn from(t: GapPair<GapPair<A1, B1>, GapPair<C1, D1>>) -> Self {
        (
            FeatureFrom::from(t.0 .0),
            FeatureFrom::from(t.0 .1),
            FeatureFrom::from(t.1 .0),
            FeatureFrom::from(t.1 .1),
        )
    }
}

impl<A, B: FeatureFrom<A>> FeatureFrom<GapPair<GapPair<A, A>, GapPair<A, A>>> for [B; 4] {
    fn from(t: GapPair<GapPair<A, A>, GapPair<A, A>>) -> Self {
        [
            FeatureFrom::from(t.0 .0),
            FeatureFrom::from(t.0 .1),
            FeatureFrom::from(t.1 .0),
            FeatureFrom::from(t.1 .1),
        ]
    }
}

impl<A1, A2: FeatureFrom<A1>, B1, B2: FeatureFrom<B1>> FeatureFrom<GapPair<A1, B1>> for (A2, B2) {
    fn from(sp: GapPair<A1, B1>) -> Self {
        (FeatureFrom::from(sp.0), FeatureFrom::from(sp.1))
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

    fn iterate_features(&self, origin: &'a [T]) -> Self::Iter {
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

    fn iterate_features(&self, origin: &'a str) -> Self::Iter {
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

    fn iterate_features(&self, origin: &'a String) -> Self::Iter {
        GapGramIter::new(&origin.as_bytes(), &self.a, self.gap, &self.b)
    }
}*/

pub fn gap_gram<A, B>(a: A, gap: usize, b: B) -> GapGram<A, B> {
    GapGram { a, gap, b }
}
/*
impl<'a, T, A, B, TA, TB> Ftzr<&'a [T]> for GapGram<A, B>
where
    A: LinearFixed + Ftzr<&'a [T], TokenGroup = TA>,
    B: LinearFixed + Ftzr<&'a [T], TokenGroup = TB>,

{
    type TokenGroup = GapPair<TA, TB>;
    fn push_tokens<Push>(&self, origin: &'a [T], push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        let (a, b) = (self.a.chunk_size(), self.b.chunk_size());
        let schema = SkipSchema {
            group_a: (a, a),
            gap: (self.gap, self.gap),
            group_b: (b, b),
        };
        let mut _push = |t: EitherGroup<_, _>| match t {
            EitherGroup::Right(x) => push(x),
            _ => unreachable!(),
        };
        schema.push_tokens(origin, &mut _push);
    }
}*/

/*
///TODO: a user-implemented featurizer 'F' must impl  IterFtzr (not just Ftzr)
/// in order for GapGram<F,_> or GapGram<_,F> to impl Ftzr
/// ForEach and MultiFtzr do not have this limitation
/// (but BookEnds does)
/// maybe use a macro similar to internal::impl_ftrzs ??
*/

impl<Origin, A, B> Ftzr<Origin> for GapGram<A, B>
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
