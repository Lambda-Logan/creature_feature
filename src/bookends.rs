use crate::accum_ftzr::Ftzr;
use crate::from_token::FromToken;
use crate::tokengroup::Token;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct BookEnds<A, B> {
    front: A,
    back: B,
    front_size: usize,
    back_size: usize,
}

pub fn bookends<A, B>(front: (usize, A), back: (B, usize)) -> BookEnds<A, B> {
    BookEnds {
        front: front.1,
        front_size: front.0,
        back_size: back.1,
        back: back.0,
    }
}

impl<'a, T: 'a, TA: 'a, TB: 'a, A, B> Ftzr<&'a [T]> for BookEnds<A, B>
where
    A: Ftzr<&'a [T], TokenGroup = TA>,
    B: Ftzr<&'a [T], TokenGroup = TB>,
{
    type TokenGroup = FrontBack<TA, TB>;
    type Iter = BookEndsIter<A::Iter, B::Iter>;

    fn chunk_size(&self) -> usize {
        (self.front.chunk_size() + self.back.chunk_size()) / 2
    }

    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        BookEndsIter(
            true,
            self.front.extract_tokens(&origin[..self.front_size]),
            self.back
                .extract_tokens(&origin[origin.len() - self.back_size..]),
        )
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub enum FrontBack<A, B> {
    Front(A),
    Back(B),
}

impl<A, B, C> FromToken<FrontBack<A, B>> for Token<C>
where
    C: FromToken<A> + FromToken<B>,
{
    fn from(x: FrontBack<A, B>) -> Self {
        Token(match x {
            FrontBack::Front(a) => FromToken::from(a),
            FrontBack::Back(a) => FromToken::from(a),
        })
    }
}

impl<A, B, Ax, Bx> FromToken<FrontBack<A, B>> for Result<Ax, Bx>
where
    Ax: FromToken<A>,
    Bx: FromToken<B>,
{
    fn from(x: FrontBack<A, B>) -> Self {
        match x {
            FrontBack::Front(a) => Ok(FromToken::from(a)),
            FrontBack::Back(a) => Err(FromToken::from(a)),
        }
    }
}
