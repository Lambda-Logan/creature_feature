use crate::accum_ftzr::Ftzr;
use crate::from_token::FromToken;
use std::marker::PhantomData;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct GapGramIter<'a, A, B, T, U1, U2> {
    a: A,
    b: B,
    gap: usize,
    data: &'a [T],
    idx: usize,
    tok2: PhantomData<(U1, U2)>,
    total_size: usize,
}

impl<'a, A, B, T, U1: 'a, U2: 'a> GapGramIter<'a, A, B, T, U1, U2> {
    fn new<
        AF: Ftzr<&'a [T], TokenGroup = U1, Iter = A>,
        BF: Ftzr<&'a [T], TokenGroup = U2, Iter = B>,
    >(
        origin: &'a [T],
        af: &AF,
        gap: usize,
        bf: &BF,
    ) -> Self {
        let a = af.extract_tokens(origin);
        let b = bf.extract_tokens(&origin[af.chunk_size() + gap..]);
        GapGramIter {
            a: a,
            b: b,
            gap: gap,
            idx: 0,
            data: origin,
            tok2: Default::default(),
            total_size: af.chunk_size() + gap + bf.chunk_size(),
        }
    }
}

impl<'a, A, B, T, U1, U2: 'a> Iterator for GapGramIter<'a, A, B, T, U1, U2>
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
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct GapGram<A, B> {
    a: A,
    gap: usize,
    b: B,
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct GapPair<A, B>(pub(crate) A, pub(crate) B, pub(crate) u16); //TODO make names fields

impl<A1, A2: From<A1>, B1, B2: From<B1>> From<GapPair<A1, B1>> for (A2, B2) {
    fn from(sp: GapPair<A1, B1>) -> Self {
        (From::from(sp.0), From::from(sp.1))
    }
}

impl<A1, A2: FromToken<A1>, B1, B2: FromToken<B1>> FromToken<GapPair<A1, B1>> for (A2, B2) {
    fn from(sp: GapPair<A1, B1>) -> Self {
        (FromToken::from(sp.0), FromToken::from(sp.1))
    }
}

impl<'a, T: 'a, A, B, U1: 'a, U2: 'a> Ftzr<&'a [T]> for GapGram<A, B>
where
    A: Ftzr<&'a [T], TokenGroup = U1>,
    B: Ftzr<&'a [T], TokenGroup = U2>,
{
    type TokenGroup = GapPair<U1, U2>;
    type Iter = GapGramIter<'a, A::Iter, B::Iter, T, U1, U2>;

    fn chunk_size(&self) -> usize {
        self.a.chunk_size() + self.gap + self.b.chunk_size()
    }

    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        //unimplemented!()
        GapGramIter::new(origin, &self.a, self.gap, &self.b)
    }
}

impl<'a, A, B, U1: 'a, U2: 'a> Ftzr<&'a str> for GapGram<A, B>
where
    A: Ftzr<&'a [u8], TokenGroup = U1>,
    B: Ftzr<&'a [u8], TokenGroup = U2>,
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

impl<'a, A, B, U1: 'a, U2: 'a> Ftzr<&'a String> for GapGram<A, B>
where
    A: Ftzr<&'a [u8], TokenGroup = U1>,
    B: Ftzr<&'a [u8], TokenGroup = U2>,
{
    type TokenGroup = GapPair<U1, U2>;
    type Iter = GapGramIter<'a, A::Iter, B::Iter, u8, U1, U2>;

    fn chunk_size(&self) -> usize {
        self.a.chunk_size() + self.gap + self.b.chunk_size()
    }

    fn extract_tokens(&self, origin: &'a String) -> Self::Iter {
        GapGramIter::new(&origin.as_bytes(), &self.a, self.gap, &self.b)
    }
}

pub fn gap_gram<A, B>(a: A, gap: usize, b: B) -> GapGram<A, B> {
    GapGram {
        a: a,
        gap: gap,
        b: b,
    }
}
