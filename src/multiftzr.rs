use crate::accum_ftzr::Ftzr;
use crate::from_token::FromToken;
use crate::tokengroup::Token;
use std::hash::{Hash, Hasher};
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct MultiFtzr<A, B>(pub A, pub B);

impl<'a, Origin: 'a, TA, TB, A, B> Ftzr<&'a Origin> for MultiFtzr<A, B>
where
    Origin: ?Sized,
    A: Ftzr<&'a Origin, TokenGroup = TA>,
    B: Ftzr<&'a Origin, TokenGroup = TB>,
{
    type TokenGroup = EitherGroup<TA, TB>;
    type Iter = MultiFtzrIter<A::Iter, B::Iter>;

    fn chunk_size(&self) -> usize {
        (self.0.chunk_size() + self.1.chunk_size()) / 2
    }

    fn extract_tokens(&self, origin: &'a Origin) -> Self::Iter {
        MultiFtzrIter(
            true,
            self.0.extract_tokens(&origin),
            self.1.extract_tokens(&origin),
        )
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct MultiFtzrIter<A, B>(bool, A, B);

impl<A, B> Iterator for MultiFtzrIter<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = EitherGroup<A::Item, B::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            // it's still in the first section
            match self.1.next() {
                None => {
                    self.0 = false;
                    self.2.next().map(EitherGroup::Right)
                }
                otherwise => otherwise.map(EitherGroup::Left),
            }
        } else {
            // it's in the last section
            self.2.next().map(EitherGroup::Right)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum EitherGroup<A, B> {
    Left(A),
    Right(B),
}

/// This is needed so that identical features will
/// have the same hash in the case of MultiFtzr<A,A>
/// Notice that this is not desired for `bookends::FrontBack`
impl<A: Hash, B: Hash> Hash for EitherGroup<A, B> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        match self {
            EitherGroup::Left(x) => x.hash(state),
            EitherGroup::Right(x) => x.hash(state),
        }
    }
}

impl<A, B, Ax, Bx> From<EitherGroup<A, B>> for Result<Ax, Bx>
where
    Ax: From<A>,
    Bx: From<B>,
{
    fn from(x: EitherGroup<A, B>) -> Self {
        match x {
            EitherGroup::Left(a) => Ok(From::from(a)),
            EitherGroup::Right(a) => Err(From::from(a)),
        }
    }
}

impl<A, B, X> From<EitherGroup<A, B>> for Token<X>
where
    X: From<A> + From<B>,
{
    fn from(x: EitherGroup<A, B>) -> Self {
        Token({
            match x {
                EitherGroup::Left(a) => From::from(a),
                EitherGroup::Right(a) => From::from(a),
            }
        })
    }
}

impl<A, B, X> FromToken<EitherGroup<A, B>> for Token<X>
where
    X: FromToken<A> + FromToken<B>,
{
    fn from(x: EitherGroup<A, B>) -> Self {
        Token({
            match x {
                EitherGroup::Left(a) => FromToken::from(a),
                EitherGroup::Right(a) => FromToken::from(a),
            }
        })
    }
}
